use grindr::{DeviceInfo, GrindrClient, GrindrHeaders, Method};
use serde::Serialize;
use tauri::AppHandle;

use crate::session::{activate_stored, partial_session, set_active_client};
use crate::state::AppState;
use crate::store::{AccountInfo, StoredAccount};

const BASE_URL: &str = "https://grindr.mobi";

#[derive(Serialize)]
pub(crate) struct ResponsePayload {
    status: u16,
    body: String,
}

#[tauri::command]
pub(crate) fn generate_device() -> DeviceInfo {
    DeviceInfo::generate()
}

#[tauri::command]
pub(crate) async fn list_accounts(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<AccountInfo>, String> {
    let store = state.store.lock().await;
    Ok(store.accounts.iter().map(AccountInfo::from).collect())
}

#[tauri::command]
pub(crate) async fn get_active(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, String> {
    Ok(state.store.lock().await.active.clone())
}

#[tauri::command]
pub(crate) async fn add_account(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    email: String,
    auth_token: String,
    device: Option<DeviceInfo>,
) -> Result<AccountInfo, String> {
    let device = device.unwrap_or_else(DeviceInfo::generate);
    let session = partial_session(&email, &auth_token)?;
    let client = GrindrClient::new(device.clone(), Some(session)).map_err(|e| e.to_string())?;

    client.refresh_token().await.map_err(|e| e.to_string())?;
    let session = client
        .session_receiver()
        .borrow()
        .clone()
        .ok_or_else(|| "no session after refresh".to_string())?;

    let id = session.profile_id.clone();
    let account = StoredAccount {
        id: id.clone(),
        email,
        profile_id: session.profile_id.clone(),
        session,
        device,
    };
    let info = AccountInfo::from(&account);

    {
        let mut store = state.store.lock().await;
        if let Some(existing) = store.accounts.iter_mut().find(|a| a.id == id) {
            *existing = account;
        } else {
            store.accounts.push(account);
        }
        store.active = Some(id.clone());
        state.persist(&store)?;
    }

    set_active_client(&state, &app, client, id).await;
    Ok(info)
}

#[tauri::command]
pub(crate) async fn set_active(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: Option<String>,
) -> Result<(), String> {
    match &id {
        Some(id) => activate_stored(&state, &app, id).await?,
        None => *state.active_client.lock().await = None,
    }
    let mut store = state.store.lock().await;
    store.active = id;
    state.persist(&store)
}

#[tauri::command]
pub(crate) async fn delete_account(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Option<String>, String> {
    let (was_active, new_active) = {
        let mut store = state.store.lock().await;
        let idx = store
            .accounts
            .iter()
            .position(|a| a.id == id)
            .ok_or_else(|| format!("account not found: {id}"))?;
        store.accounts.remove(idx);

        let was_active = store.active.as_deref() == Some(id.as_str());
        let new_active = if was_active {
            if store.accounts.is_empty() {
                None
            } else {
                let next = idx.min(store.accounts.len() - 1);
                Some(store.accounts[next].id.clone())
            }
        } else {
            store.active.clone()
        };
        store.active = new_active.clone();
        state.persist(&store)?;
        (was_active, new_active)
    };

    if was_active {
        match &new_active {
            Some(id) => activate_stored(&state, &app, id).await?,
            None => *state.active_client.lock().await = None,
        }
    }
    Ok(new_active)
}

#[tauri::command]
pub(crate) async fn send_request(
    state: tauri::State<'_, AppState>,
    method: String,
    path: String,
    body: Option<serde_json::Value>,
) -> Result<ResponsePayload, String> {
    let method =
        Method::from_bytes(method.as_bytes()).map_err(|e| format!("invalid method: {e}"))?;

    let client = state.active_client.lock().await.clone();
    if let Some(client) = client {
        let resp = client
            .request_authenticated_raw(method, &path, body)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(ResponsePayload {
            status: resp.status,
            body: String::from_utf8_lossy(&resp.body).into_owned(),
        });
    }

    if !path.starts_with('/') {
        return Err("path must begin with '/'".to_string());
    }
    let headers = GrindrHeaders::build(&state.noauth_device, &state.noauth_ua, None, None)
        .map_err(|e| e.to_string())?;
    let mut req = state
        .noauth_client
        .request(method, format!("{BASE_URL}{path}"));
    for (name, value) in headers.items {
        req = req.header(name, value);
    }
    if let Some(b) = body {
        req = req.json(&b);
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    Ok(ResponsePayload {
        status,
        body: String::from_utf8_lossy(&bytes).into_owned(),
    })
}

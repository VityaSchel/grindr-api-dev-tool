use std::time::Duration;

use grindr::{DeviceInfo, GrindrClient, GrindrHeaders, Method};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tokio::sync::oneshot;

use crate::session::{activate_stored, partial_session, set_active_client};
use crate::state::AppState;
use crate::store::{AccountInfo, StoredAccount};

const BASE_URL: &str = "https://grindr.mobi";

const OPENAPI_URL: &str = "https://opengrind.org/openapi.json";

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

const UPLOAD_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Serialize)]
pub(crate) struct ResponsePayload {
    status: u16,
    body: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BodyFile {
    path: String,
    content_type: String,
    #[serde(default)]
    signed: bool,
}

#[derive(Serialize)]
pub(crate) struct FileMeta {
    name: String,
    size: u64,
}

#[tauri::command]
pub(crate) fn generate_device() -> DeviceInfo {
    DeviceInfo::generate()
}

#[tauri::command]
pub(crate) async fn stat_file(path: String) -> Result<FileMeta, String> {
    let meta = tokio::fs::metadata(&path)
        .await
        .map_err(|e| format!("could not read {path}: {e}"))?;
    let name = std::path::Path::new(&path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_owned();
    Ok(FileMeta {
        name,
        size: meta.len(),
    })
}

#[tauri::command]
pub(crate) async fn fetch_openapi(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let fetch = async {
        let resp = state
            .noauth_client
            .request(Method::GET, OPENAPI_URL)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("{OPENAPI_URL} returned HTTP {}", resp.status()));
        }
        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    };
    tokio::time::timeout(REQUEST_TIMEOUT, fetch)
        .await
        .map_err(|_| format!("{OPENAPI_URL} timed out"))?
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

#[derive(Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub(crate) enum SignInCredentials {
    #[serde(rename_all = "camelCase")]
    Token { email: String, auth_token: String },
    #[serde(rename_all = "camelCase")]
    Password { email: String, password: String },
    #[serde(rename_all = "camelCase")]
    Google { google_token: String },
}

async fn establish_session(
    device: &DeviceInfo,
    credentials: &SignInCredentials,
    geohash: Option<&str>,
) -> Result<GrindrClient, String> {
    let initial = match credentials {
        SignInCredentials::Token { email, auth_token } => Some(partial_session(email, auth_token)?),
        _ => None,
    };
    let client = GrindrClient::new(device.clone(), initial).map_err(|e| e.to_string())?;

    match credentials {
        SignInCredentials::Token { .. } => {
            client.refresh_token_with_geohash(geohash).await.map(drop)
        }
        SignInCredentials::Password { email, password } => client
            .login_with_geohash(email, password, geohash)
            .await
            .map(drop),
        SignInCredentials::Google { google_token } => client
            .google_sign_in_with_geohash(google_token, geohash)
            .await
            .map(drop),
    }
    .map_err(|e| e.to_string())?;

    Ok(client)
}

#[tauri::command]
pub(crate) async fn add_account(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    credentials: SignInCredentials,
    device: Option<DeviceInfo>,
    geohash: Option<String>,
) -> Result<AccountInfo, String> {
    let device = device.unwrap_or_else(DeviceInfo::generate);
    let geohash = geohash.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let client = establish_session(&device, &credentials, geohash).await?;

    let session = client
        .session_receiver()
        .borrow()
        .clone()
        .ok_or_else(|| "no session after sign-in".to_string())?;

    let id = session.profile_id.clone();
    let account = StoredAccount {
        id: id.clone(),
        email: session.email.clone(),
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
    request_id: String,
    body_file: Option<BodyFile>,
) -> Result<ResponsePayload, String> {
    let method =
        Method::from_bytes(method.as_bytes()).map_err(|e| format!("invalid method: {e}"))?;

    let timeout = if body_file.is_some() {
        UPLOAD_TIMEOUT
    } else {
        REQUEST_TIMEOUT
    };

    // Register a cancellation handle so `cancel_request` can abort this from the UI.
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();
    state
        .inflight
        .lock()
        .await
        .insert(request_id.clone(), cancel_tx);

    let result = tokio::select! {
        res = perform_request(&state, method, &path, body, body_file) => res,
        // Sender dropped by `cancel_request` (or below on completion) resolves this.
        _ = cancel_rx => Err("request cancelled".to_string()),
        _ = tokio::time::sleep(timeout) => Err("request timed out".to_string()),
    };

    state.inflight.lock().await.remove(&request_id);
    result
}

/// Abort an in-flight `send_request` by id. No-op if it already finished.
#[tauri::command]
pub(crate) async fn cancel_request(
    state: tauri::State<'_, AppState>,
    request_id: String,
) -> Result<(), String> {
    state.inflight.lock().await.remove(&request_id);
    Ok(())
}

/// The actual HTTP work: authenticated via the active client, or an unauthenticated
/// fallback for no-auth endpoints. Dropped (cancelled) when its `select!` arm loses.
async fn perform_request(
    state: &AppState,
    method: Method,
    path: &str,
    body: Option<serde_json::Value>,
    body_file: Option<BodyFile>,
) -> Result<ResponsePayload, String> {
    let file_body = match body_file {
        Some(bf) => {
            let bytes = tokio::fs::read(&bf.path)
                .await
                .map_err(|e| format!("could not read {}: {e}", bf.path))?;
            Some((bytes, bf.content_type, bf.signed))
        }
        None => None,
    };

    let client = state.active_client.lock().await.clone();
    if let Some(client) = client {
        let resp = match file_body {
            Some((bytes, content_type, true)) => {
                client
                    .request_signed_bytes(method, path, &content_type, bytes)
                    .await
            }
            Some((bytes, content_type, false)) => {
                client
                    .request_authenticated_bytes(method, path, &content_type, bytes)
                    .await
            }
            None => client.request_authenticated_raw(method, path, body).await,
        }
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
    if let Some((bytes, content_type, _signed)) = file_body {
        req = req.header("content-type", content_type).body(bytes);
    } else if let Some(b) = body {
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

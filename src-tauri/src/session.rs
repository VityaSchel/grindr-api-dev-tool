use grindr::{GrindrClient, Session};
use serde_json::json;
use tauri::{AppHandle, Manager};
use tokio::sync::watch;

use crate::state::AppState;

pub(crate) fn partial_session(email: &str, auth_token: &str) -> Result<Session, String> {
    serde_json::from_value(json!({
        "email": email,
        "expires_at": 0u64,
        "profile_id": "",
        "session_id": "",
        "auth_token": auth_token,
        "kind": "Email",
        "third_party_user_id": null,
    }))
    .map_err(|e| format!("could not build session: {e}"))
}

pub(crate) fn spawn_persist_watch(
    app: AppHandle,
    id: String,
    mut rx: watch::Receiver<Option<Session>>,
) {
    tauri::async_runtime::spawn(async move {
        while rx.changed().await.is_ok() {
            let session = rx.borrow().clone();
            let Some(session) = session else { continue };
            let state = app.state::<AppState>();
            let mut store = state.store.lock().await;
            if let Some(acc) = store.accounts.iter_mut().find(|a| a.id == id) {
                acc.session = session;
                if let Err(e) = state.persist(&store) {
                    eprintln!("failed to persist refreshed session: {e}");
                }
            }
        }
    });
}

pub(crate) async fn set_active_client(
    state: &AppState,
    app: &AppHandle,
    client: GrindrClient,
    id: String,
) {
    let rx = client.session_receiver();
    *state.active_client.lock().await = Some(client);
    spawn_persist_watch(app.clone(), id, rx);
}

pub(crate) async fn activate_stored(
    state: &AppState,
    app: &AppHandle,
    id: &str,
) -> Result<(), String> {
    let (device, session) = {
        let store = state.store.lock().await;
        let acc = store
            .accounts
            .iter()
            .find(|a| a.id == id)
            .ok_or_else(|| format!("account not found: {id}"))?;
        (acc.device.clone(), acc.session.clone())
    };
    let client = GrindrClient::new(device, Some(session)).map_err(|e| e.to_string())?;
    set_active_client(state, app, client, id.to_string()).await;
    Ok(())
}

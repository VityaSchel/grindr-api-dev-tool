use std::time::Duration;

use grindr::Method;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use crate::commands::REQUEST_TIMEOUT;
use crate::state::AppState;

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

/// The actual HTTP work: authenticated via the active client, or unauthenticated
/// for no-auth endpoints. Dropped (cancelled) when its `select!` arm loses.
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
    let resp = match (client, file_body) {
        (Some(client), Some((bytes, content_type, true))) => {
            client
                .request_signed_bytes(method, path, &content_type, bytes)
                .await
        }
        (Some(client), Some((bytes, content_type, false))) => {
            client
                .request_authenticated_bytes(method, path, &content_type, bytes)
                .await
        }
        (Some(client), None) => client.request_authenticated_raw(method, path, body).await,
        (None, Some(_)) => {
            return Err("Select an account to upload a file.".to_string());
        }
        (None, None) => {
            state
                .noauth_client
                .request_no_auth_raw(method, path, body)
                .await
        }
    }
    .map_err(|e| e.to_string())?;

    Ok(ResponsePayload {
        status: resp.status,
        body: String::from_utf8_lossy(&resp.body).into_owned(),
    })
}

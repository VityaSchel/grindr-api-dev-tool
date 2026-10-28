use grindr::Method;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use crate::session::ensure_session_token;
use crate::state::AppState;

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
	let method = Method::from_bytes(method.as_bytes())
		.map_err(|e| format!("invalid method: {e}"))?;

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
	let active_client = state.active_client.lock().await.clone();
	let mut request = match &active_client {
		Some(client) => {
			ensure_session_token(client).await?;
			client.request(method, path)
		}
		None if body_file.is_some() => {
			return Err("Select an account to upload a file.".to_string());
		}
		None => state.noauth_client.request(method, path).unauthenticated(),
	};

	if let Some(file) = body_file {
		let bytes = tokio::fs::read(&file.path)
			.await
			.map_err(|e| format!("could not read {}: {e}", file.path))?;
		request = if file.signed {
			request.signed_bytes(&file.content_type, bytes)
		} else {
			request.bytes(&file.content_type, bytes)
		};
	} else if let Some(body) = body {
		request = request.json(&body);
	}

	let resp = request.send().await.map_err(|e| e.to_string())?;
	Ok(ResponsePayload {
		status: resp.status,
		body: String::from_utf8_lossy(&resp.body).into_owned(),
	})
}

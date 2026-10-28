use grindr::{DeviceInfo, DeviceSigningKey, GrindrClient, Session};
use tauri::{AppHandle, Manager};
use tokio::sync::watch;

use crate::state::AppState;

pub(crate) fn spawn_persist_watch(
	app: AppHandle,
	id: String,
	mut rx: watch::Receiver<Option<Session>>,
) {
	tauri::async_runtime::spawn(async move {
		while rx.changed().await.is_ok() {
			let credentials = rx
				.borrow()
				.as_ref()
				.map(|session| session.credentials.clone());
			let Some(credentials) = credentials else {
				continue;
			};
			let state = app.state::<AppState>();
			let mut store = state.store.lock().await;
			if let Some(acc) = store.accounts.iter_mut().find(|a| a.id == id) {
				if acc.credentials == credentials {
					continue;
				}
				acc.credentials = credentials;
				if let Err(e) = state.persist(&store) {
					eprintln!("failed to persist refreshed session: {e}");
				}
			}
		}
	});
}

pub(crate) fn spawn_signing_key_watch(
	app: AppHandle,
	id: String,
	mut rx: watch::Receiver<Option<DeviceSigningKey>>,
) {
	tauri::async_runtime::spawn(async move {
		while rx.changed().await.is_ok() {
			let key = rx.borrow().clone();
			let state = app.state::<AppState>();
			let mut store = state.store.lock().await;
			if let Some(acc) = store.accounts.iter_mut().find(|a| a.id == id) {
				acc.signing_key = key;
				if let Err(e) = state.persist(&store) {
					eprintln!("failed to persist signing key: {e}");
				}
			}
		}
	});
}

pub(crate) async fn ensure_session_token(
	client: &GrindrClient,
) -> Result<(), String> {
	let token_missing = client
		.session_receiver()
		.borrow()
		.as_ref()
		.is_some_and(|session| session.token.is_none());
	if token_missing {
		client.refresh_session().await.map_err(|e| e.to_string())?;
	}
	Ok(())
}

pub(crate) async fn set_active_client(
	state: &AppState,
	app: &AppHandle,
	client: GrindrClient,
	id: String,
) {
	let session_rx = client.session_receiver();
	let key_rx = client.signing_key_receiver();
	*state.active_client.lock().await = Some(client);
	spawn_persist_watch(app.clone(), id.clone(), session_rx);
	spawn_signing_key_watch(app.clone(), id, key_rx);
}

pub(crate) async fn activate_stored(
	state: &AppState,
	app: &AppHandle,
	id: &str,
) -> Result<(), String> {
	let device = {
		let store = state.store.lock().await;
		store
			.accounts
			.iter()
			.find(|a| a.id == id)
			.map(|a| a.device.clone())
			.ok_or_else(|| format!("account not found: {id}"))?
	};
	activate_stored_with_device(state, app, id, device).await
}

pub(crate) async fn activate_stored_with_device(
	state: &AppState,
	app: &AppHandle,
	id: &str,
	device: DeviceInfo,
) -> Result<(), String> {
	let (credentials, signing_key) = {
		let store = state.store.lock().await;
		store
			.accounts
			.iter()
			.find(|a| a.id == id)
			.map(|a| (a.credentials.clone(), a.signing_key.clone()))
			.ok_or_else(|| format!("account not found: {id}"))?
	};
	let session = Session {
		credentials,
		token: None,
	};
	let client =
		GrindrClient::new(device, Some(session)).map_err(|e| e.to_string())?;
	if let Some(key) = signing_key {
		if !client.restore_signing_key(key).await {
			eprintln!(
				"stored signing key rejected for {id}, registering a new one"
			);
		}
	}
	set_active_client(state, app, client, id.to_string()).await;
	Ok(())
}

use grindr::{
	Credentials, DeviceInfo, GrindrClient, Session, SessionKind, SignInResult,
};
use serde::Deserialize;
use tauri::AppHandle;

use crate::session::{
	activate_stored, activate_stored_with_device, set_active_client,
};
use crate::state::AppState;
use crate::store::{AccountDetails, AccountInfo, StoredAccount, TokenDetails};

#[derive(Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub(crate) enum SignInCredentials {
	#[serde(rename_all = "camelCase")]
	Token { email: String, auth_token: String },
	#[serde(rename_all = "camelCase")]
	Password {
		email: String,
		password: String,
		captcha_token: Option<String>,
	},
	#[serde(rename_all = "camelCase")]
	Google { google_token: String },
	#[serde(rename_all = "camelCase")]
	Facebook { facebook_token: String },
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
pub(crate) async fn account_details(
	state: tauri::State<'_, AppState>,
	id: String,
) -> Result<AccountDetails, String> {
	let (device, credentials, is_active) = {
		let store = state.store.lock().await;
		let account = store
			.accounts
			.iter()
			.find(|a| a.id == id)
			.ok_or_else(|| format!("account not found: {id}"))?;
		(
			account.device.clone(),
			account.credentials.clone(),
			store.active.as_deref() == Some(id.as_str()),
		)
	};
	let token = match state.active_client.lock().await.as_ref() {
		Some(client) if is_active => client
			.session_receiver()
			.borrow()
			.as_ref()
			.and_then(|session| session.token.as_ref())
			.map(TokenDetails::from),
		_ => None,
	};
	Ok(AccountDetails {
		device,
		credentials,
		token,
	})
}

/// Rotates before persisting so a device the client rejects never reaches disk.
#[tauri::command]
pub(crate) async fn update_account_device(
	app: AppHandle,
	state: tauri::State<'_, AppState>,
	id: String,
	device: DeviceInfo,
) -> Result<(), String> {
	let is_active = {
		let store = state.store.lock().await;
		if !store.accounts.iter().any(|a| a.id == id) {
			return Err(format!("account not found: {id}"));
		}
		store.active.as_deref() == Some(id.as_str())
	};

	if is_active {
		let client = state.active_client.lock().await.clone();
		match client {
			Some(client) => client
				.rotate_device(device.clone())
				.await
				.map(drop)
				.map_err(|e| e.to_string())?,
			None => {
				activate_stored_with_device(&state, &app, &id, device.clone())
					.await?
			}
		}
	}

	let mut store = state.store.lock().await;
	let account = store
		.accounts
		.iter_mut()
		.find(|a| a.id == id)
		.ok_or_else(|| format!("account not found: {id}"))?;
	account.device = device;
	state.persist(&store)
}

async fn establish_session(
	device: &DeviceInfo,
	credentials: &SignInCredentials,
	geohash: Option<&str>,
) -> Result<(GrindrClient, SignInResult), String> {
	let resumed = match credentials {
		SignInCredentials::Token { email, auth_token } => Some(Session {
			credentials: Credentials {
				email: email.clone(),
				profile_id: None,
				auth_token: auth_token.clone(),
				kind: SessionKind::Email,
				third_party_user_id: None,
			},
			token: None,
		}),
		_ => None,
	};
	let client = GrindrClient::new(device.clone(), resumed)
		.map_err(|e| e.to_string())?;

	let result = match credentials {
		SignInCredentials::Token { .. } => {
			client.refresh_session_at_geohash(geohash).await
		}
		SignInCredentials::Password {
			email,
			password,
			captcha_token: Some(captcha_token),
		} => {
			if geohash.is_some() {
				return Err(
					"A geohash can't be sent with a reCAPTCHA sign-in.".into(),
				);
			}
			client
				.sign_in_with_email_captcha(email, password, captcha_token)
				.await
		}
		SignInCredentials::Password {
			email,
			password,
			captcha_token: None,
		} => {
			client
				.sign_in_with_email_at_geohash(email, password, geohash)
				.await
		}
		SignInCredentials::Google { google_token } => {
			client
				.sign_in_with_google_at_geohash(google_token, geohash)
				.await
		}
		SignInCredentials::Facebook { facebook_token } => {
			client
				.sign_in_with_facebook_at_geohash(facebook_token, geohash)
				.await
		}
	}
	.map_err(|e| e.to_string())?;

	Ok((client, result))
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
	let (client, result) =
		establish_session(&device, &credentials, geohash).await?;

	let credentials = client
		.session_receiver()
		.borrow()
		.as_ref()
		.map(|session| session.credentials.clone())
		.ok_or_else(|| "no session after sign-in".to_string())?;

	let id = result.profile_id;
	let account = StoredAccount {
		id: id.clone(),
		email: credentials.email.clone(),
		profile_id: id.clone(),
		credentials,
		device,
		signing_key: None,
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

use std::path::Path;

use grindr::{
	Credentials, DeviceInfo, DeviceSigningKey, Restriction, SessionToken,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct StoredAccount {
	pub(crate) id: String,
	pub(crate) email: String,
	pub(crate) profile_id: String,
	#[serde(alias = "session")]
	pub(crate) credentials: Credentials,
	pub(crate) device: DeviceInfo,
	#[serde(default)]
	pub(crate) signing_key: Option<DeviceSigningKey>,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Store {
	#[serde(default)]
	pub(crate) accounts: Vec<StoredAccount>,
	#[serde(default)]
	pub(crate) active: Option<String>,
}

#[derive(Serialize, Clone)]
pub(crate) struct AccountInfo {
	pub(crate) id: String,
	pub(crate) email: String,
	pub(crate) profile_id: String,
}

impl From<&StoredAccount> for AccountInfo {
	fn from(a: &StoredAccount) -> Self {
		AccountInfo {
			id: a.id.clone(),
			email: a.email.clone(),
			profile_id: a.profile_id.clone(),
		}
	}
}

#[derive(Serialize)]
pub(crate) struct AccountDetails {
	pub(crate) device: DeviceInfo,
	pub(crate) credentials: Credentials,
	pub(crate) token: Option<TokenDetails>,
}

#[derive(Serialize)]
pub(crate) struct TokenDetails {
	session_id: String,
	expires_at: u64,
	restriction: Option<Restriction>,
}

impl From<&SessionToken> for TokenDetails {
	fn from(token: &SessionToken) -> Self {
		TokenDetails {
			session_id: token.session_id.clone(),
			expires_at: token.expires_at,
			restriction: token.restriction.clone(),
		}
	}
}

pub(crate) fn load_store(path: &Path) -> Store {
	let Ok(bytes) = std::fs::read(path) else {
		return Store::default();
	};
	serde_json::from_slice(&bytes).unwrap_or_else(|e| {
		eprintln!("accounts store is unreadable, starting empty: {e}");
		Store::default()
	})
}

pub(crate) fn save_store(path: &Path, store: &Store) -> Result<(), String> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
	}
	let bytes = serde_json::to_vec_pretty(store).map_err(|e| e.to_string())?;
	std::fs::write(path, bytes).map_err(|e| e.to_string())?;
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		let _ = std::fs::set_permissions(
			path,
			std::fs::Permissions::from_mode(0o600),
		);
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use grindr::SessionKind;

	use super::*;

	const DEVICE: &str = r#"{
        "device_type": 2, "device_id": "aaaabbbbccccdddd", "os": "Android 14",
        "screen_resolution": "2400x1080", "total_ram": "8589934592",
        "advertising_id": "1111", "device_model": "Pixel 8",
        "manufacturer": "Google", "timezone": "Europe/Berlin",
        "locale": "en_US", "accept_language": "en-US"
    }"#;

	fn store_with(account_fields: &str) -> Store {
		let json = format!(
			r#"{{
                "accounts": [{{
                    "id": "1", "email": "a@b.c", "profile_id": "1",
                    {account_fields},
                    "device": {DEVICE}
                }}],
                "active": "1"
            }}"#
		);
		serde_json::from_str(&json).expect("store must parse")
	}

	#[test]
	fn a_store_without_signing_keys_still_loads() {
		let store = store_with(
			r#""credentials": {
                "email": "a@b.c", "profile_id": "1", "auth_token": "tok",
                "kind": "Email", "third_party_user_id": null
            }"#,
		);
		assert_eq!(store.active.as_deref(), Some("1"));
		assert!(store.accounts[0].signing_key.is_none());
	}

	#[test]
	fn a_session_saved_by_grindr_0_10_loads_as_credentials() {
		let store = store_with(
			r#""session": {
                "email": "a@b.c", "expires_at": 1700000000, "profile_id": "1",
                "session_id": "sid", "auth_token": "tok", "kind": "Google",
                "third_party_user_id": "g-1",
                "restriction": "TrustVendorRejected"
            }"#,
		);
		let credentials = &store.accounts[0].credentials;
		assert_eq!(credentials.auth_token, "tok");
		assert_eq!(credentials.profile_id.as_deref(), Some("1"));
		assert_eq!(credentials.kind, SessionKind::Google);
		assert_eq!(credentials.third_party_user_id.as_deref(), Some("g-1"));
	}

	#[test]
	fn credentials_are_saved_under_their_own_name() {
		let store = store_with(
			r#""session": {
                "email": "a@b.c", "expires_at": 0, "profile_id": "1",
                "session_id": "sid", "auth_token": "tok", "kind": "Email"
            }"#,
		);
		let saved = serde_json::to_value(&store).unwrap();
		let account = &saved["accounts"][0];
		assert_eq!(account["credentials"]["auth_token"], "tok");
		assert!(account.get("session").is_none());
		assert!(account["credentials"].get("session_id").is_none());
	}
}

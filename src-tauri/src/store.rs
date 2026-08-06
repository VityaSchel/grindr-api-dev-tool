use std::path::Path;

use grindr::{DeviceInfo, DeviceSigningKey, Session};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct StoredAccount {
	pub(crate) id: String,
	pub(crate) email: String,
	pub(crate) profile_id: String,
	pub(crate) session: Session,
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

#[derive(Serialize, Clone)]
pub(crate) struct AccountDetails {
	pub(crate) device: DeviceInfo,
	pub(crate) session: Session,
}

impl From<&StoredAccount> for AccountDetails {
	fn from(a: &StoredAccount) -> Self {
		AccountDetails {
			device: a.device.clone(),
			session: a.session.clone(),
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
	use super::*;

	/// Accounts written before signing keys were stored must keep loading.
	#[test]
	fn a_store_without_signing_keys_still_loads() {
		let json = r#"{
            "accounts": [{
                "id": "1", "email": "a@b.c", "profile_id": "1",
                "session": {
                    "email": "a@b.c", "expires_at": 0, "profile_id": "1",
                    "session_id": "sid", "auth_token": "tok", "kind": "Email",
                    "third_party_user_id": null
                },
                "device": {
                    "device_type": 2, "device_id": "aaaabbbbccccdddd", "os": "Android 14",
                    "screen_resolution": "2400x1080", "total_ram": "8589934592",
                    "advertising_id": "1111", "device_model": "Pixel 8",
                    "manufacturer": "Google", "timezone": "Europe/Berlin",
                    "locale": "en_US", "accept_language": "en-US"
                }
            }],
            "active": "1"
        }"#;

		let store: Store =
			serde_json::from_str(json).expect("legacy store must parse");
		assert_eq!(store.active.as_deref(), Some("1"));
		assert!(store.accounts[0].signing_key.is_none());
	}
}

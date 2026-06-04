use std::path::Path;

use grindr::{DeviceInfo, Session};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct StoredAccount {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) profile_id: String,
    pub(crate) session: Session,
    pub(crate) device: DeviceInfo,
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
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

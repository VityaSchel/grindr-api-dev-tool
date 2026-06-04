use std::path::PathBuf;

use grindr::{DeviceInfo, GrindrClient};
use tokio::sync::Mutex;

use crate::store::{save_store, Store};

pub(crate) struct AppState {
    pub(crate) store_path: PathBuf,
    pub(crate) store: Mutex<Store>,
    pub(crate) active_client: Mutex<Option<GrindrClient>>,
    pub(crate) noauth_client: wreq::Client,
    pub(crate) noauth_device: DeviceInfo,
    pub(crate) noauth_ua: String,
}

impl AppState {
    pub(crate) fn persist(&self, store: &Store) -> Result<(), String> {
        save_store(&self.store_path, store)
    }
}

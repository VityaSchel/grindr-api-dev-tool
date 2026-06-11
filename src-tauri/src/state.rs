use std::collections::HashMap;
use std::path::PathBuf;

use grindr::{DeviceInfo, GrindrClient};
use tokio::sync::{oneshot, Mutex};

use crate::store::{save_store, Store};

pub(crate) struct AppState {
    pub(crate) store_path: PathBuf,
    pub(crate) store: Mutex<Store>,
    pub(crate) active_client: Mutex<Option<GrindrClient>>,
    pub(crate) noauth_client: wreq::Client,
    pub(crate) noauth_device: DeviceInfo,
    pub(crate) noauth_ua: String,
    /// In-flight requests keyed by the frontend's request id. Dropping the sender
    /// (via `cancel_request` or on completion) cancels the request's `select!` arm.
    pub(crate) inflight: Mutex<HashMap<String, oneshot::Sender<()>>>,
}

impl AppState {
    pub(crate) fn persist(&self, store: &Store) -> Result<(), String> {
        save_store(&self.store_path, store)
    }
}

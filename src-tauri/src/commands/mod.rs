mod accounts;
mod requests;

use std::time::Duration;

use grindr::{requires_device_signature, DeviceInfo, Method};
use serde::Serialize;

use crate::state::AppState;

pub(crate) use accounts::{
	account_details, add_account, delete_account, get_active, list_accounts,
	set_active, update_account_device,
};
pub(crate) use requests::{cancel_request, send_request};

const OPENAPI_URL: &str = "https://opengrind.org/openapi.json";

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Serialize)]
pub(crate) struct FileMeta {
	name: String,
	size: u64,
}

#[tauri::command]
pub(crate) fn generate_device() -> DeviceInfo {
	DeviceInfo::generate()
}

#[tauri::command]
pub(crate) fn requires_signature(path: String) -> bool {
	requires_device_signature(&path)
}

#[tauri::command]
pub(crate) async fn stat_file(path: String) -> Result<FileMeta, String> {
	let meta = tokio::fs::metadata(&path)
		.await
		.map_err(|e| format!("could not read {path}: {e}"))?;
	let name = std::path::Path::new(&path)
		.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("")
		.to_owned();
	Ok(FileMeta {
		name,
		size: meta.len(),
	})
}

#[tauri::command]
pub(crate) async fn fetch_openapi(
	state: tauri::State<'_, AppState>,
) -> Result<String, String> {
	let fetch = async {
		let resp = state
			.openapi_client
			.request(Method::GET, OPENAPI_URL)
			.send()
			.await
			.map_err(|e| e.to_string())?;
		if !resp.status().is_success() {
			return Err(format!(
				"{OPENAPI_URL} returned HTTP {}",
				resp.status()
			));
		}
		let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
		Ok(String::from_utf8_lossy(&bytes).into_owned())
	};
	tokio::time::timeout(REQUEST_TIMEOUT, fetch)
		.await
		.map_err(|_| format!("{OPENAPI_URL} timed out"))?
}

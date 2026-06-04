mod commands;
mod session;
mod state;
mod store;

use grindr::{build_user_agent, probe_emulation, DeviceInfo};
use tauri::Manager;
use tokio::sync::Mutex;

use crate::commands::{
    add_account, delete_account, generate_device, get_active, list_accounts, send_request,
    set_active,
};
use crate::session::activate_stored;
use crate::state::AppState;
use crate::store::load_store;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let store_path = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("no app data dir: {e}"))?
                .join("accounts.json");
            let store = load_store(&store_path);
            let active = store.active.clone();

            let device = DeviceInfo::generate();
            let ua = build_user_agent(&device, "Free");
            let noauth_client = wreq::Client::builder()
                .emulation(probe_emulation())
                .gzip(true)
                .build()
                .map_err(|e| format!("failed to build http client: {e}"))?;

            app.manage(AppState {
                store_path,
                store: Mutex::new(store),
                active_client: Mutex::new(None),
                noauth_client,
                noauth_device: device,
                noauth_ua: ua,
            });

            if let Some(id) = active {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    let state = app_handle.state::<AppState>();
                    if let Err(e) = activate_stored(&state, &app_handle, &id).await {
                        eprintln!("failed to restore active account {id}: {e}");
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_device,
            list_accounts,
            get_active,
            add_account,
            set_active,
            delete_account,
            send_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

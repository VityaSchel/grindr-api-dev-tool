mod commands;
mod session;
mod state;
mod store;

use std::collections::HashMap;
use std::time::Duration;

use grindr::{DeviceInfo, GrindrClient};
use tauri::Manager;
use tokio::sync::Mutex;

use crate::commands::{
    account_details, add_account, cancel_request, delete_account, fetch_openapi, generate_device,
    get_active, list_accounts, requires_signature, send_request, set_active, stat_file,
    update_account_device,
};
use crate::session::activate_stored;
use crate::state::AppState;
use crate::store::load_store;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let store_path = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("no app data dir: {e}"))?
                .join("accounts.json");
            let store = load_store(&store_path);
            let active = store.active.clone();

            let noauth_client = GrindrClient::new(DeviceInfo::generate(), None)
                .map_err(|e| format!("failed to build http client: {e}"))?;
            // Our own docs host: no reason to reach it wearing the app's fingerprint.
            let openapi_client = wreq::Client::builder()
                .gzip(true)
                .connect_timeout(Duration::from_secs(10))
                .read_timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| format!("failed to build http client: {e}"))?;

            app.manage(AppState {
                store_path,
                store: Mutex::new(store),
                active_client: Mutex::new(None),
                noauth_client,
                openapi_client,
                inflight: Mutex::new(HashMap::new()),
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

            #[cfg(target_os = "macos")]
            {
                use tauri::menu::{AboutMetadata, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
                use tauri::Emitter;

                let close_tab = MenuItemBuilder::with_id("close_tab", "Close Tab")
                    .accelerator("CmdOrCtrl+W")
                    .build(app)?;

                let app_menu = SubmenuBuilder::new(app, "Grindr API developer tool")
                    .about(Some(AboutMetadata::default()))
                    .separator()
                    .services()
                    .separator()
                    .hide()
                    .hide_others()
                    .show_all()
                    .separator()
                    .quit()
                    .build()?;

                let edit_menu = SubmenuBuilder::new(app, "Edit")
                    .undo()
                    .redo()
                    .separator()
                    .cut()
                    .copy()
                    .paste()
                    .select_all()
                    .build()?;

                let window_menu = SubmenuBuilder::new(app, "Window")
                    .item(&close_tab)
                    .separator()
                    .minimize()
                    .build()?;

                let menu = MenuBuilder::new(app)
                    .item(&app_menu)
                    .item(&edit_menu)
                    .item(&window_menu)
                    .build()?;

                app.set_menu(menu)?;
                app.on_menu_event(move |app, event| {
                    if event.id() == close_tab.id() {
                        let _ = app.emit("close-tab", ());
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_device,
            requires_signature,
            fetch_openapi,
            list_accounts,
            get_active,
            account_details,
            add_account,
            set_active,
            delete_account,
            update_account_device,
            send_request,
            cancel_request,
            stat_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

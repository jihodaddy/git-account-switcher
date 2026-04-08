mod commands;
mod credential;
mod git;
mod models;
mod storage;
mod validation;

use storage::Storage;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            let storage = Storage::new(app_dir);
            app.manage(storage);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::account::get_accounts,
            commands::account::get_account,
            commands::account::create_account,
            commands::account::update_account,
            commands::account::delete_account,
            commands::account::switch_account,
            commands::account::validate_token,
            commands::binding::get_bindings,
            commands::binding::bind_repository,
            commands::binding::unbind_repository,
            commands::system::get_current_git_user,
            commands::system::get_settings,
            commands::system::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

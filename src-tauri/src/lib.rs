mod commands;
mod config;
mod core;
mod error;
mod state;
mod subscription;
mod system;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_core_status,
            commands::get_settings,
            commands::update_settings,
            commands::get_hwid,
            commands::get_platform,
            commands::connect,
            commands::disconnect,
            commands::get_traffic,
            commands::get_subscriptions,
            commands::add_subscription,
            commands::remove_subscription,
            commands::update_subscription,
            commands::switch_core,
            commands::get_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

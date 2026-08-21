pub mod tools;
pub mod db;
pub mod parser;
pub mod scanner;
pub mod watcher;
pub mod commands;

use std::env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));

    let app_state = commands::AppState {
        db: None, // We would init this properly async in a setup hook
        home_dir: home,
        config_dir: config_home,
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::scan_all,
            commands::get_skills,
            commands::get_skill,
            commands::save_skill,
            commands::create_skill,
            commands::delete_skill,
            commands::get_skill_content,
            commands::start_watch,
            commands::stop_watch,
            commands::get_collections,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

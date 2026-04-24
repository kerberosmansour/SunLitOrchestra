#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod events;
mod provider;
mod state;

use tauri::Manager;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Load persisted settings on startup
            if let Ok(app_data_dir) = app.path().app_data_dir() {
                let settings = state::load_settings(&app_data_dir);
                let state = app.state::<AppState>();
                let mut current = state.settings.lock().expect("settings lock poisoned");
                *current = settings;
            }
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::plan::start_planning,
            commands::plan::read_runbook,
            commands::plan::save_runbook,
            commands::run::start_execution,
            commands::run::cancel_execution,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::get_available_providers,
            commands::settings::get_available_models,
            commands::voice::transcribe_audio,
            commands::voice::transcribe_audio_standalone,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    #[test]
    fn tauri_crate_depends_on_common() {
        // Given: sldo-tauri depends on sldo-common
        // When: version() is called
        let v = sldo_common::version();
        // Then: it returns a non-empty string
        assert!(!v.is_empty());
    }
}

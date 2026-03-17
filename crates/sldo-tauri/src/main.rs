#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod events;
mod state;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::plan::start_planning,
            commands::plan::read_runbook,
            commands::plan::save_runbook,
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

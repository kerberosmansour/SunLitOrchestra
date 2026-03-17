//! Settings Tauri commands — get and update application settings.

use tauri::{AppHandle, Manager};

use crate::provider;
use crate::state::{AppSettings, AppState};
/// Get the current application settings.
#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    let state = app.state::<AppState>();
    let settings = state
        .settings
        .lock()
        .map_err(|e| format!("Failed to lock settings: {}", e))?;
    Ok(settings.clone())
}

/// Update application settings, persisting to disk.
#[tauri::command]
pub fn update_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let state = app.state::<AppState>();

    // Persist to disk if app data dir is available
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        crate::state::save_settings(&app_data_dir, &settings)?;
    }

    // Update in-memory state
    let mut current = state
        .settings
        .lock()
        .map_err(|e| format!("Failed to lock settings: {}", e))?;
    *current = settings;
    Ok(())
}

/// Get list of available provider names.
#[tauri::command]
pub fn get_available_providers() -> Vec<String> {
    provider::available_providers()
}

/// Get list of available models for a given provider.
#[tauri::command]
pub fn get_available_models(provider_name: String) -> Vec<String> {
    match provider::get_provider(&provider_name) {
        Some(p) => p.available_models(),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Feature: Settings commands ──────────────────────────────────────

    #[test]
    fn get_available_providers_returns_copilot() {
        // Given/When: get_available_providers is called
        let providers = get_available_providers();
        // Then: Contains "copilot"
        assert!(providers.contains(&"copilot".to_string()));
    }

    #[test]
    fn get_available_models_for_copilot() {
        // Given: Provider name "copilot"
        // When: get_available_models is called
        let models = get_available_models("copilot".to_string());
        // Then: Returns non-empty list with known models
        assert!(!models.is_empty());
        assert!(models.contains(&"claude-opus-4.6".to_string()));
    }

    #[test]
    fn get_available_models_for_unknown_provider() {
        // Given: Unknown provider name
        // When: get_available_models is called
        let models = get_available_models("unknown".to_string());
        // Then: Returns empty list
        assert!(models.is_empty());
    }
}

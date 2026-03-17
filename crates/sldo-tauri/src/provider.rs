//! Provider trait abstraction for agent invocation.
//!
//! Defines a minimal `Provider` trait so the system can support multiple
//! coding agents beyond GitHub Copilot. Start with `CopilotProvider` that
//! wraps `CopilotInvocation::run_with_callback()`.

use std::path::Path;

use anyhow::Result;

use sldo_common::copilot::CopilotInvocation;
use sldo_common::logging::LogFile;

/// Minimal provider trait — abstracts agent invocation so adding new
/// providers (e.g., ClaudeCodeProvider) only requires a new struct + impl.
pub trait Provider: Send + Sync {
    /// Human-readable provider name (e.g., "copilot").
    fn name(&self) -> &str;

    /// List of models this provider supports.
    fn available_models(&self) -> Vec<String>;

    /// Invoke the agent with the given configuration.
    ///
    /// - `prompt`: The task prompt to send to the agent.
    /// - `model`: Which model to use.
    /// - `allow_flags`: Tool permission allow flags.
    /// - `deny_flags`: Tool permission deny flags.
    /// - `working_dir`: The working directory for the agent process.
    /// - `log_file`: Log file for capturing output.
    /// - `on_line`: Callback invoked for each line of output (line, stream_name).
    fn invoke(
        &self,
        prompt: &str,
        model: &str,
        allow_flags: &[String],
        deny_flags: &[String],
        working_dir: &Path,
        log_file: &LogFile,
        on_line: Box<dyn FnMut(&str, &str) + Send>,
    ) -> Result<i32>;
}

/// Provider backed by GitHub Copilot CLI.
pub struct CopilotProvider;

impl Provider for CopilotProvider {
    fn name(&self) -> &str {
        "copilot"
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "claude-opus-4.6".to_string(),
            "claude-sonnet-4.5".to_string(),
            "claude-sonnet-4".to_string(),
            "gpt-4o".to_string(),
            "o3".to_string(),
        ]
    }

    fn invoke(
        &self,
        prompt: &str,
        model: &str,
        allow_flags: &[String],
        deny_flags: &[String],
        working_dir: &Path,
        log_file: &LogFile,
        mut on_line: Box<dyn FnMut(&str, &str) + Send>,
    ) -> Result<i32> {
        let invocation = CopilotInvocation {
            prompt: prompt.to_string(),
            model: model.to_string(),
            allow_flags: allow_flags.to_vec(),
            deny_flags: deny_flags.to_vec(),
            working_dir: working_dir.to_path_buf(),
        };
        invocation.run_with_callback(log_file, |line, stream| {
            on_line(line, stream);
        })
    }
}

/// Look up a provider by name. Returns `None` if the name is unknown.
pub fn get_provider(name: &str) -> Option<Box<dyn Provider>> {
    match name {
        "copilot" => Some(Box::new(CopilotProvider)),
        _ => None,
    }
}

/// List all available provider names.
pub fn available_providers() -> Vec<String> {
    vec!["copilot".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Feature: Provider trait abstraction ──────────────────────────────

    #[test]
    fn copilot_provider_name_is_copilot() {
        // Given: A CopilotProvider
        let provider = CopilotProvider;
        // When: name() is called
        let name = provider.name();
        // Then: Returns "copilot"
        assert_eq!(name, "copilot");
    }

    #[test]
    fn copilot_provider_has_models() {
        // Given: A CopilotProvider
        let provider = CopilotProvider;
        // When: available_models() is called
        let models = provider.available_models();
        // Then: Returns a non-empty list containing expected models
        assert!(!models.is_empty());
        assert!(models.contains(&"claude-opus-4.6".to_string()));
    }

    #[test]
    fn get_provider_returns_copilot() {
        // Given: Provider name "copilot"
        // When: get_provider is called
        let provider = get_provider("copilot");
        // Then: Returns Some with name "copilot"
        assert!(provider.is_some());
        assert_eq!(provider.unwrap().name(), "copilot");
    }

    #[test]
    fn get_provider_returns_none_for_unknown() {
        // Given: Unknown provider name
        // When: get_provider is called
        let provider = get_provider("unknown-agent");
        // Then: Returns None
        assert!(provider.is_none());
    }

    #[test]
    fn available_providers_includes_copilot() {
        // Given/When: available_providers is called
        let providers = available_providers();
        // Then: Contains "copilot"
        assert!(providers.contains(&"copilot".to_string()));
    }
}

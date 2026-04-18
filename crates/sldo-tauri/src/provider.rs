//! Provider trait abstraction for agent invocation.
//!
//! Defines a minimal `Provider` trait so the system can support multiple
//! coding agents. Start with `ClaudeProvider` that wraps
//! `ClaudeInvocation::run_with_callback()`.

use std::path::Path;

use anyhow::Result;

use sldo_common::copilot::ClaudeInvocation;
use sldo_common::logging::LogFile;

/// Minimal provider trait — abstracts agent invocation so adding new
/// providers only requires a new struct + impl.
pub trait Provider: Send + Sync {
    /// Human-readable provider name (e.g., "claude").
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

/// Provider backed by Claude Code CLI.
pub struct ClaudeProvider;

impl Provider for ClaudeProvider {
    fn name(&self) -> &str {
        "claude"
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "claude-opus-4-7".to_string(),
            "claude-sonnet-4-6".to_string(),
            "claude-haiku-4-5".to_string(),
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
        let invocation = ClaudeInvocation {
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
        "claude" => Some(Box::new(ClaudeProvider)),
        _ => None,
    }
}

/// List all available provider names.
pub fn available_providers() -> Vec<String> {
    vec!["claude".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_provider_name_is_claude() {
        let provider = ClaudeProvider;
        assert_eq!(provider.name(), "claude");
    }

    #[test]
    fn claude_provider_has_models() {
        let provider = ClaudeProvider;
        let models = provider.available_models();
        assert!(!models.is_empty());
        assert!(models.contains(&"claude-sonnet-4-6".to_string()));
    }

    #[test]
    fn get_provider_returns_claude() {
        let provider = get_provider("claude");
        assert!(provider.is_some());
        assert_eq!(provider.unwrap().name(), "claude");
    }

    #[test]
    fn get_provider_returns_none_for_unknown() {
        let provider = get_provider("unknown-agent");
        assert!(provider.is_none());
    }

    #[test]
    fn available_providers_includes_claude() {
        let providers = available_providers();
        assert!(providers.contains(&"claude".to_string()));
    }
}

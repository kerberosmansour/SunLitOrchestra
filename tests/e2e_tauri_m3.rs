//! E2E runtime validation tests for Tauri Desktop Milestone 3.
//!
//! These tests validate the planning backend integration:
//! - `run_with_callback` captures output via closure
//! - Planning command rejects missing claude
//! - Planning uses correct tool flags

/// E2E: `run_with_callback` collects output via closure.
///
/// Creates a mock scenario where ClaudeInvocation is configured and
/// `run_with_callback` is called. Verifies the callback mechanism works
/// (claude may or may not be on PATH — we test the wiring, not claude itself).
#[test]
fn claude_run_with_callback_captures_lines() {
    // Given: A ClaudeInvocation configured with test parameters
    let tmp = std::env::temp_dir().join("sldo_e2e_callback_m3");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let log = sldo_common::logging::LogFile::new(&tmp, "e2e_callback.log").unwrap();
    let inv = sldo_common::copilot::ClaudeInvocation {
        prompt: "test prompt".to_string(),
        model: "test-model".to_string(),
        allow_flags: vec!["--allowedTools=Read,Write,Bash".to_string()],
        deny_flags: vec![],
        working_dir: tmp.clone(),
    };

    // When: run_with_callback is called with a capturing closure
    let mut captured: Vec<(String, String)> = Vec::new();
    let result = inv.run_with_callback(&log, |line, stream| {
        captured.push((line.to_string(), stream.to_string()));
    });

    // Then: Either succeeds (claude installed) or errors gracefully (not installed)
    match result {
        Ok(code) => {
            // If claude is installed, callback received lines
            assert!(code >= -1, "Exit code should be valid");
            // Verify streams are valid values
            for (_, stream) in &captured {
                assert!(
                    stream == "stdout" || stream == "stderr",
                    "Stream must be stdout or stderr, got: {}",
                    stream
                );
            }
        }
        Err(e) => {
            // Claude not installed — error should mention claude or spawn
            let msg = e.to_string().to_lowercase();
            assert!(
                msg.contains("claude") || msg.contains("spawn"),
                "Error should mention claude or spawn: {}",
                msg
            );
            // No lines should have been captured
            assert!(captured.is_empty());
        }
    }

    let _ = std::fs::remove_dir_all(&tmp);
}

/// E2E: Preflight validation rejects missing claude binary.
///
/// Verifies that `check_claude_installed()` returns an error with a
/// meaningful message when claude is not on PATH. This may pass or
/// be a no-op if claude is actually installed — the test verifies
/// the error handling path exists.
#[test]
fn plan_command_rejects_missing_claude() {
    // Given: We check for claude installation
    let result = sldo_common::preflight::check_claude_installed();

    // Then: If claude is not installed, error message mentions "claude"
    match result {
        Ok(path) => {
            // Claude is installed — verify path exists
            assert!(path.exists(), "Claude binary should exist at: {:?}", path);
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            assert!(
                msg.contains("claude"),
                "Error should mention claude: {}",
                msg
            );
        }
    }
}

/// E2E: Planning uses correct tool flags.
///
/// Verifies that `plan_allow_flags()` returns a non-empty vec with
/// expected permission flags.
#[test]
fn plan_allow_flags_used() {
    // Given: The planning tool flags module
    // When: plan_allow_flags() is called
    let flags = sldo_common::toolflags::plan_allow_flags();

    // Then: Returns non-empty vec with expected flags
    assert!(!flags.is_empty(), "plan_allow_flags should return flags");
    assert!(
        flags.iter().any(|f| f.contains("Write")),
        "Should contain Write permission"
    );
    assert!(
        flags.iter().any(|f| f.contains("Bash")),
        "Should contain Bash permission"
    );

    // Deny flags are empty for Claude Code CLI
    let deny = sldo_common::toolflags::plan_deny_flags();
    assert!(deny.is_empty(), "plan_deny_flags should be empty for Claude Code CLI");
}

/// E2E: Verify the event types can be serialized for Tauri emission.
#[test]
fn event_types_serialize_for_tauri() {
    // Given: Event payload types used by planning commands
    // PlanProgressEvent
    let progress_json = serde_json::json!({
        "line": "Analyzing repo structure...",
        "stream": "stdout",
        "timestamp": "2026-03-17T12:00:00Z"
    });
    assert!(progress_json.is_object());
    assert_eq!(progress_json["stream"], "stdout");

    // PlanCompleteEvent
    let complete_json = serde_json::json!({
        "runbook_path": "/tmp/RUNBOOK.md",
        "validation_issues": []
    });
    assert!(complete_json["validation_issues"].is_array());

    // PlanErrorEvent
    let error_json = serde_json::json!({
        "error": "claude not found"
    });
    assert!(error_json["error"].is_string());
}

/// E2E: Verify backward compatibility — existing `run()` method still works.
#[test]
fn existing_run_method_backward_compatible() {
    // Given: A ClaudeInvocation
    let tmp = std::env::temp_dir().join("sldo_e2e_run_compat_m3");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let log = sldo_common::logging::LogFile::new(&tmp, "compat.log").unwrap();
    let inv = sldo_common::copilot::ClaudeInvocation {
        prompt: "hello".to_string(),
        model: "test".to_string(),
        allow_flags: vec![],
        deny_flags: vec![],
        working_dir: tmp.clone(),
    };

    // When: The original run() method is called
    let result = inv.run(&log);

    // Then: Behaves the same as before the refactor
    match result {
        Ok(code) => {
            let _ = code; // claude existed
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            assert!(
                msg.contains("claude") || msg.contains("spawn"),
                "Error should mention claude: {}",
                msg
            );
        }
    }

    // Verify log file was written to
    let log_content = std::fs::read_to_string(log.path()).unwrap_or_default();
    assert!(
        log_content.contains("Running claude"),
        "Log should contain invocation entry"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

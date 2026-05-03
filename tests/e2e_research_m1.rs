//! E2E runtime validation tests for Milestone 1 — sldo-research CLI skeleton.
//!
//! These tests verify that the sldo-research binary parses arguments, rejects
//! invalid inputs, and runs preflight checks at runtime.

use std::process::Command;

fn binary() -> String {
    static RESEARCH_BIN: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    RESEARCH_BIN
        .get_or_init(|| {
            let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let bin = manifest_dir
                .join("target")
                .join("debug")
                .join(format!("sldo-research{}", std::env::consts::EXE_SUFFIX));

            if !bin.exists() {
                let status = Command::new("cargo")
                    .args(["build", "-p", "sldo-research"])
                    .current_dir(&manifest_dir)
                    .status()
                    .expect("failed to build sldo-research");
                assert!(status.success(), "cargo build -p sldo-research failed");
            }

            bin.to_string_lossy().into_owned()
        })
        .clone()
}

#[test]
fn test_help_flag() {
    // Given: The sldo-research binary is built
    // When: --help is run
    // Then: exit 0, stdout contains "research"
    let output = Command::new(binary())
        .arg("--help")
        .output()
        .expect("failed to execute sldo-research --help");

    assert!(
        output.status.success(),
        "sldo-research --help should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.to_lowercase().contains("research"),
        "sldo-research --help should mention 'research', got: {}",
        stdout
    );
}

#[test]
fn test_missing_prompt_fails() {
    // Given: no prompt file and no --prompt flag
    // When: sldo-research is invoked with no args
    // Then: exit non-zero
    let output = Command::new(binary())
        .output()
        .expect("failed to execute sldo-research");

    assert!(
        !output.status.success(),
        "sldo-research with no prompt should exit non-zero"
    );
}

#[test]
fn test_both_prompt_sources_fails() {
    // Given: both a prompt file path and --prompt are provided
    // When: sldo-research is invoked with both
    // Then: exit non-zero with error about conflicting sources
    let tmp = std::env::temp_dir().join("sldo_research_e2e_m1_both");
    std::fs::create_dir_all(&tmp).unwrap();
    let prompt_file = tmp.join("prompt.txt");
    std::fs::write(&prompt_file, "test topic").unwrap();

    let output = Command::new(binary())
        .arg(prompt_file.to_str().unwrap())
        .arg("--prompt")
        .arg("inline text")
        .output()
        .expect("failed to execute sldo-research");

    assert!(
        !output.status.success(),
        "sldo-research with both prompt sources should exit non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("both") || stderr.contains("not both") || stderr.contains("either"),
        "Expected conflict error, got: {}",
        stderr
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_prompt_file_accepted() {
    // Given: a valid prompt file
    // When: sldo-research is invoked with that file
    // Then: either exit 0 (all preflight passed) or a clear controlled error
    let tmp = std::env::temp_dir().join("sldo_research_e2e_m1_file");
    std::fs::create_dir_all(&tmp).unwrap();
    let prompt_file = tmp.join("prompt.txt");
    std::fs::write(&prompt_file, "research the best Rust async runtimes").unwrap();

    let output = Command::new(binary())
        .arg(prompt_file.to_str().unwrap())
        .env("PATH", "/sldo_research_nonexistent_path_for_m1")
        .output()
        .expect("failed to execute sldo-research");

    // Accept exit 0 (claude installed, safe branch) OR a non-zero exit with a
    // clear diagnostic (claude not on PATH, protected branch, etc.)
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.trim().is_empty(),
            "Expected a clear error message on failure, got empty stderr"
        );
    }

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_inline_prompt_accepted() {
    // Given: an inline --prompt argument
    // When: sldo-research --prompt "test topic" is run
    // Then: either exit 0 or a clear controlled error (same as file case)
    let output = Command::new(binary())
        .arg("--prompt")
        .arg("test topic for inline research")
        .env("PATH", "/sldo_research_nonexistent_path_for_m1")
        .output()
        .expect("failed to execute sldo-research");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.trim().is_empty(),
            "Expected a clear error message on failure, got empty stderr"
        );
    }
}

#[test]
fn test_invalid_repo_dir_fails() {
    // Given: --repo-dir pointing to a nonexistent path
    // When: sldo-research is invoked
    // Then: exit non-zero
    let output = Command::new(binary())
        .arg("--prompt")
        .arg("test topic")
        .arg("--repo-dir")
        .arg("/nonexistent/path/that/does/not/exist")
        .output()
        .expect("failed to execute sldo-research");

    assert!(
        !output.status.success(),
        "sldo-research with invalid repo dir should exit non-zero"
    );
}

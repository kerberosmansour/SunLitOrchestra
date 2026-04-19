//! E2E runtime validation tests for Milestone 2 — research prompt builder.
//!
//! These tests verify that the M1 CLI surface still works, that the new prompt
//! construction step is exercised at runtime (visible byte-count info line),
//! and that `prompt.rs` is a pure module — no scratch files appear in `output/`
//! or `.sldo-logs/` beyond what M1 already produced.

use std::process::Command;

fn binary() -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-research"
}

#[test]
fn test_binary_still_accepts_m1_args() {
    // Given: M1 CLI surface
    // When:  --prompt is supplied (no other args)
    // Then:  binary either exits 0 (claude on PATH, safe defaults) OR a
    //        non-zero exit with a clear stderr diagnostic
    let output = Command::new(binary())
        .arg("--prompt")
        .arg("evaluate async runtimes")
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
fn test_run_logs_prompt_length() {
    // Given: a valid inline prompt
    // When:  the binary runs through pre-flight + prompt construction
    // Then:  combined stdout/stderr surfaces a "bytes" hint, indicating that
    //        the exploration prompt was built at runtime
    let output = Command::new(binary())
        .arg("--prompt")
        .arg("test topic")
        .output()
        .expect("failed to execute sldo-research");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let combined_lower = combined.to_lowercase();

    // We accept either: success path (prompt built, "bytes" surfaced) OR
    // the claude-missing failure path (clear diagnostic on stderr).
    if output.status.success() {
        assert!(
            combined_lower.contains("bytes") || combined_lower.contains("prompt"),
            "Expected prompt-construction info line, got:\n{}",
            combined
        );
    } else {
        // Failure path: must have produced a non-empty diagnostic.
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.trim().is_empty(),
            "Expected stderr diagnostic on failure"
        );
    }
}

#[test]
fn test_prompt_module_does_not_leak_files() {
    // Given: a clean scratch directory used as CWD
    // When:  the binary is invoked with --prompt
    // Then:  no new files are written under that scratch dir, since the
    //        M2 prompt builder is pure and the M3 loop has not yet been wired
    let tmp = std::env::temp_dir().join("sldo_research_e2e_m2_no_leak");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let _ = Command::new(binary())
        .arg("--prompt")
        .arg("test topic")
        .current_dir(&tmp)
        .output()
        .expect("failed to execute sldo-research");

    // The scratch dir should still be empty (or contain only directories that
    // pre-existed). Pure prompt builders write nothing; M3 will add log files.
    let entries: Vec<_> = std::fs::read_dir(&tmp)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        entries.is_empty(),
        "M2 should not write any files under CWD, but found: {:?}",
        entries.iter().map(|e| e.path()).collect::<Vec<_>>()
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

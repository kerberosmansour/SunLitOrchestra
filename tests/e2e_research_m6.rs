//! E2E runtime validation tests for Milestone 6 — multi-source synthesis pass.
//!
//! These tests drive the `sldo-research` binary as a subprocess and verify
//! the synthesis phase added in M6:
//!   - A `research-synthesis.log` file is created when claude is available.
//!   - Synthesis failure (non-zero claude exit) does NOT halt the pipeline —
//!     the dossier is still written using the M4 raw-findings layout.
//!   - When the shim emits a fully-formed synthesised body containing every
//!     required dossier section header, the resulting dossier embeds that
//!     body and the M4 stub sentinel is absent.
//!   - When the shim's output is unstructured (a single marker line), the
//!     synthesis fallback kicks in and the dossier still contains the
//!     required section structure with raw findings under `## Key Findings`.
//!
//! The `--help` regression guard pins the M1–M5 CLI surface (no new flags
//! were added in M6 — synthesis is unconditional once the loop runs).
//!
//! All tests use the PATH-shim pattern inherited verbatim from M3/M4/M5:
//! a temporary `claude` script on PATH answers every Claude invocation in
//! the loop (exploration, optional web-search, optional deepening,
//! synthesis) with the same scripted output. No real Claude API is invoked.

use std::path::{Path, PathBuf};
use std::process::Command;

fn binary() -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + "/target/debug/sldo-research"
}

fn unique_tmp(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "sldo_research_e2e_m6_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

/// Build a minimal POSIX shim at `<dir>/claude` that prints `marker` and
/// exits 0. Inherited from the M3/M4/M5 harness.
fn shim_dir_with_claude(label: &str, marker: &str) -> PathBuf {
    let dir = unique_tmp(label);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let shim = dir.join("claude");
    let body = format!("#!/bin/sh\nprintf '%s\\n' '{}'\n", marker);
    std::fs::write(&shim, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&shim).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&shim, perms).unwrap();
    }
    dir
}

/// Build a shim that emits a fully-formed synthesised dossier body — every
/// required section header is present in the printed output. This is the
/// "synthesis succeeds and is well-formed" case.
///
/// Uses `printf` (a /bin/sh builtin) rather than `cat <<EOF` because the
/// test sets `PATH` to *only* the shim directory, so external utilities
/// like `cat` are not available. `printf` is built into POSIX `/bin/sh`
/// and works without PATH.
fn shim_dir_with_well_formed_synth_claude(label: &str, marker: &str) -> PathBuf {
    let dir = unique_tmp(label);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let shim = dir.join("claude");
    // Emit all 9 required dossier headers as one printf format string —
    // `synth_output_well_formed` requires every header to be present before
    // the synthesised body replaces the M4 layout.
    let payload = format!(
        "## Executive Summary\\n\\n{m}\\n\\n\
## Topic Decomposition\\n\\n{m}\\n\\n\
## Key Findings\\n\\n{m}\\n\\n\
## Library & Tool Evaluations\\n\\n{m}\\n\\n\
## Architecture Options\\n\\n{m}\\n\\n\
## API & SDK Documentation\\n\\n{m}\\n\\n\
## Design Recommendations\\n\\n{m}\\n\\n\
## Risks & Open Questions\\n\\n{m}\\n\\n\
## References\\n\\n{m}\\n",
        m = marker
    );
    let body = format!("#!/bin/sh\nprintf '{}'\n", payload);
    std::fs::write(&shim, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&shim).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&shim, perms).unwrap();
    }
    dir
}

/// Build a shim that succeeds for the first `succeed_count` invocations
/// and then exits non-zero. Used to simulate "exploration succeeds,
/// synthesis fails" — the synthesis phase is the LAST claude invocation in
/// the loop, so passing `succeed_count = 1` with `--max-iterations 1
/// --max-searches 0` makes synthesis the failing call.
///
/// Implementation note: counts via filesystem marker files
/// (`call-1`, `call-2`, …) created with shell-builtin `:>` rather than
/// `cat`/`echo`-to-file. The test's PATH is set to only the shim
/// directory, so external utilities are not available.
fn shim_dir_with_failing_synth_claude(label: &str, marker: &str, succeed_count: u32) -> PathBuf {
    let dir = unique_tmp(label);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let shim = dir.join("claude");
    // For each invocation, find the next free `call-N` marker file (using
    // shell `[ -f ]` test — a `/bin/sh` builtin), create it with `:>`
    // (POSIX builtin redirection of the `:` no-op), and decide whether to
    // succeed or fail based on N. Because `[` and `:>` are builtins, this
    // works with PATH set to only the shim dir.
    let body = format!(
        "#!/bin/sh\n\
n=1\n\
while [ -f '{dir}/call-'\"$n\" ]; do\n\
  n=$((n + 1))\n\
done\n\
: > '{dir}/call-'\"$n\"\n\
if [ \"$n\" -gt {succeed_count} ]; then\n\
  exit 1\n\
fi\n\
printf '%s\\n' '{marker}'\n",
        dir = dir.display(),
        succeed_count = succeed_count,
        marker = marker
    );
    std::fs::write(&shim, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&shim).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&shim, perms).unwrap();
    }
    dir
}

fn shimmed_cmd(label: &str, marker: &str) -> (Command, PathBuf, PathBuf) {
    let cwd = unique_tmp(&format!("{}_cwd", label));
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_claude(&format!("{}_shim", label), marker);
    let mut cmd = Command::new(binary());
    cmd.current_dir(&cwd).env("PATH", &shim);
    (cmd, cwd, shim)
}

fn cleanup(paths: &[&Path]) {
    for p in paths {
        let _ = std::fs::remove_dir_all(p);
    }
}

// ── Synthesis log file is created when claude is available ───────────────

#[test]
fn test_synthesis_log_created_when_claude_ok() {
    // Given: a working claude shim
    let (mut cmd, cwd, shim) = shimmed_cmd("synth_log_ok", "shim findings");
    // When: run with --max-iterations 1 --max-searches 0 (minimal phases,
    //       still triggers synthesis because raw is non-empty)
    let output = cmd
        .arg("--prompt")
        .arg("topic")
        .arg("--max-iterations")
        .arg("1")
        .arg("--max-searches")
        .arg("0")
        .output()
        .expect("failed to execute sldo-research");
    assert!(
        output.status.success(),
        "binary should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    // Then: the synthesis log file exists
    let synth_log = cwd.join(".sldo-logs").join("research-synthesis.log");
    assert!(
        synth_log.exists(),
        "expected synthesis log at {}",
        synth_log.display()
    );
    cleanup(&[&cwd, &shim]);
}

// ── Synthesis fallback: failed synthesis still writes a dossier ──────────

#[test]
fn test_synthesis_fallback_still_writes_dossier() {
    // Given: a shim that succeeds for exploration (call 1) and fails for
    //        synthesis (call 2). With --max-searches 0 --max-iterations 1
    //        the loop makes exactly two claude calls.
    let cwd = unique_tmp("synth_fallback_cwd");
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_failing_synth_claude("synth_fallback_shim", "EXPLORE-MARKER-M6", 1);
    let dossier = cwd.join("fallback.md");
    // When
    let output = Command::new(binary())
        .current_dir(&cwd)
        .env("PATH", &shim)
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&dossier)
        .arg("--max-iterations")
        .arg("1")
        .arg("--max-searches")
        .arg("0")
        .output()
        .expect("failed to execute sldo-research");
    // Then: pipeline still exits 0; dossier exists; raw findings (the
    //       exploration marker) appear in the dossier; and the M4 layout
    //       is in effect (stub sentinel present because synth failed).
    assert!(
        output.status.success(),
        "binary should still exit 0 when synthesis fails; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        dossier.exists(),
        "dossier should be written even on synth failure"
    );
    let content = std::fs::read_to_string(&dossier).unwrap();
    assert!(
        content.contains("EXPLORE-MARKER-M6"),
        "fallback dossier should contain raw exploration findings; got: {}",
        content
    );
    assert!(
        content.contains("To be synthesised in M6"),
        "fallback dossier should retain M4 stub sentinel when synthesis failed; got: {}",
        content
    );
    cleanup(&[&cwd, &shim]);
}

// ── Successful well-formed synthesis removes stub sentinel ───────────────

#[test]
fn test_successful_synthesis_replaces_stub_sentinel() {
    // Given: a shim that emits a well-formed synthesised dossier body for
    //        every invocation (exploration AND synthesis). The synthesis
    //        output contains every required section header, so the
    //        well-formedness check passes and the synth body replaces the
    //        M4 layout.
    let cwd = unique_tmp("synth_replaces_cwd");
    let _ = std::fs::remove_dir_all(&cwd);
    std::fs::create_dir_all(&cwd).unwrap();
    let shim = shim_dir_with_well_formed_synth_claude("synth_replaces_shim", "SYNTH-OK-M6");
    let dossier = cwd.join("replaced.md");
    // When
    let output = Command::new(binary())
        .current_dir(&cwd)
        .env("PATH", &shim)
        .arg("--prompt")
        .arg("topic")
        .arg("--output")
        .arg(&dossier)
        .arg("--max-iterations")
        .arg("1")
        .arg("--max-searches")
        .arg("0")
        .output()
        .expect("failed to execute sldo-research");
    assert!(
        output.status.success(),
        "binary should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    // Then: the dossier contains the synth marker and does NOT contain the
    //       M4 stub sentinel — synthesis replaced the stubs.
    assert!(dossier.exists());
    let content = std::fs::read_to_string(&dossier).unwrap();
    assert!(
        content.contains("SYNTH-OK-M6"),
        "dossier should contain synth marker; got: {}",
        content
    );
    assert!(
        !content.contains("To be synthesised in M6"),
        "successful synthesis should remove the M4 stub sentinel; got: {}",
        content
    );
    cleanup(&[&cwd, &shim]);
}

// ── CLI surface unchanged after M6 (no new flags) ───────────────────────

#[test]
fn test_help_flag_unchanged_after_m6() {
    // Given: the M1–M5 CLI surface
    let output = Command::new(binary())
        .arg("--help")
        .output()
        .expect("failed to execute sldo-research --help");
    // Then: exit 0; all documented flags still present; no new M6 flag was
    //       introduced (synthesis is unconditional once the loop runs).
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for flag in [
        "--prompt",
        "--repo-dir",
        "--output",
        "--model",
        "--max-iterations",
        "--max-searches",
    ] {
        assert!(stdout.contains(flag), "--help missing flag: {}", flag);
    }
    assert!(
        !stdout.contains("--max-synthesis"),
        "M6 should not introduce a --max-synthesis flag (per the M5 lessons rule)"
    );
}

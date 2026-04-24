//! End-to-end tests for `sldo-tla-sha`.
//!
//! The binary is exercised against fake `tools.toml` files under `tempfile`
//! directories. Network-requiring scenarios (network_failure,
//! redirect_to_foreign_host_aborts, oversize_response_aborts) are tested at
//! the library level in `src/lib.rs` unit tests, since spinning up a local
//! HTTP server in CI is brittle for the savings.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sldo-tla-sha"))
}

fn write_tools_toml(dir: &Path, content: &str) -> PathBuf {
    let p = dir.join("tools.toml");
    fs::write(&p, content).unwrap();
    p
}

#[test]
fn dry_run_prints_plan_no_fetch() {
    let tmp = TempDir::new().unwrap();
    let path = write_tools_toml(
        tmp.path(),
        r#"
[tlc]
version = "1.8.0"
url = "https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar"
sha256 = "UNSET"
"#,
    );
    let out = Command::new(binary_path())
        .arg("--tools-toml")
        .arg(&path)
        .arg("--dry-run")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "dry-run failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("would fetch"));
    assert!(stdout.contains("tlc"));
    assert!(stdout.contains("github.com"));
    assert!(stdout.contains("OK"), "host allow-list status should show OK");
}

#[test]
fn missing_file_errors_cleanly() {
    let out = Command::new(binary_path())
        .arg("--tools-toml")
        .arg("/nonexistent/path/to/tools.toml")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failed to read") || stderr.contains("No such file"),
        "stderr should name the missing file, got: {stderr}"
    );
}

#[test]
fn skips_already_populated_in_plan() {
    let tmp = TempDir::new().unwrap();
    let path = write_tools_toml(
        tmp.path(),
        r#"
[tlc]
version = "1.8.0"
url = "https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar"
sha256 = "UNSET"

[apalache]
version = "0.44.11"
url = "https://github.com/apalache-mc/apalache/releases/download/v0.44.11/apalache.tgz"
sha256 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
"#,
    );
    let out = Command::new(binary_path())
        .arg("--tools-toml")
        .arg(&path)
        .arg("--dry-run")
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("tlc"), "unset section must be planned");
    assert!(
        !stdout.contains("apalache"),
        "populated section must NOT be planned; got: {stdout}"
    );
}

#[test]
fn nothing_to_do_when_all_populated() {
    let tmp = TempDir::new().unwrap();
    let path = write_tools_toml(
        tmp.path(),
        r#"
[tlc]
version = "1.8.0"
url = "https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar"
sha256 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
"#,
    );
    let out = Command::new(binary_path())
        .arg("--tools-toml")
        .arg(&path)
        .output()
        .unwrap();
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("nothing to populate"),
        "stderr should say there is nothing to do; got: {stderr}"
    );
}

#[test]
fn verify_refuses_when_any_unset() {
    let tmp = TempDir::new().unwrap();
    let path = write_tools_toml(
        tmp.path(),
        r#"
[tlc]
version = "1.8.0"
url = "https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar"
sha256 = "UNSET"
"#,
    );
    let out = Command::new(binary_path())
        .arg("--tools-toml")
        .arg(&path)
        .arg("--verify")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("UNSET"));
    assert!(stderr.contains("without --verify"));
}

#[test]
fn help_exits_clean() {
    let out = Command::new(binary_path()).arg("--help").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("sldo-tla-sha"));
    assert!(stdout.contains("tools-toml"));
    assert!(stdout.contains("verify"));
    assert!(stdout.contains("dry-run"));
}

#[test]
fn malformed_toml_errors_cleanly() {
    let tmp = TempDir::new().unwrap();
    let path = write_tools_toml(tmp.path(), "this is not valid toml [[[");
    let out = Command::new(binary_path())
        .arg("--tools-toml")
        .arg(&path)
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("parse") || stderr.contains("failed"));
}

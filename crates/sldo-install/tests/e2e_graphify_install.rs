//! End-to-end tests for the Graphify doctor/install-plan CLI.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sldo-install"))
}

fn sldo_install_graphify(cwd: &Path, path: &Path, extra: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(binary_path());
    cmd.current_dir(cwd).env("PATH", path);
    for arg in extra {
        cmd.arg(arg);
    }
    cmd.output().expect("failed to exec sldo-install")
}

#[cfg(unix)]
fn make_fake_graphify(bin_dir: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let script = bin_dir.join("graphify");
    fs::write(&script, "#!/bin/sh\necho graphify 9.9.9\n").unwrap();
    let mut permissions = fs::metadata(&script).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&script, permissions).unwrap();
}

#[cfg(windows)]
fn make_fake_graphify(bin_dir: &Path) {
    fs::write(
        bin_dir.join("graphify.cmd"),
        "@echo off\r\necho graphify 9.9.9\r\n",
    )
    .unwrap();
}

fn make_graphify_source_checkout(root: &Path) {
    let package = root.join("graphify");
    fs::create_dir_all(&package).unwrap();
    fs::write(package.join("__init__.py"), "").unwrap();
    fs::write(package.join("build.py"), "").unwrap();
}

#[test]
fn graphify_install_plan_does_not_require_skills_dir() {
    let cwd = TempDir::new().unwrap();
    let path = TempDir::new().unwrap();

    let out = sldo_install_graphify(cwd.path(), path.path(), &["graphify", "--install-plan"]);

    assert!(
        out.status.success(),
        "install plan failed: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Graphify readiness"));
    assert!(stdout.contains("uv tool install graphifyy"));
}

#[test]
fn graphify_doctor_fails_closed_when_graphify_is_missing() {
    let cwd = TempDir::new().unwrap();
    let path = TempDir::new().unwrap();

    let out = sldo_install_graphify(cwd.path(), path.path(), &["graphify"]);

    assert!(
        !out.status.success(),
        "doctor unexpectedly succeeded: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stdout.contains("status: not ready"));
    assert!(stderr.contains("Graphify is not ready"));
}

#[test]
fn graphify_doctor_accepts_cli_on_path() {
    let cwd = TempDir::new().unwrap();
    let path = TempDir::new().unwrap();
    make_fake_graphify(path.path());

    let out = sldo_install_graphify(cwd.path(), path.path(), &["--host", "codex", "graphify"]);

    assert!(
        out.status.success(),
        "doctor failed: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("status: ready"));
    assert!(stdout.contains("graphify CLI: found (graphify 9.9.9)"));
    assert!(stdout.contains("graphify install --platform codex"));
}

#[test]
fn graphify_doctor_accepts_source_checkout() {
    let cwd = TempDir::new().unwrap();
    let path = TempDir::new().unwrap();
    let source = TempDir::new().unwrap();
    make_graphify_source_checkout(source.path());

    let source_arg = source.path().to_string_lossy().to_string();
    let out = sldo_install_graphify(
        cwd.path(),
        path.path(),
        &["graphify", "--graphify-path", &source_arg],
    );

    assert!(
        out.status.success(),
        "doctor failed: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("status: ready"));
    assert!(stdout.contains("source checkout: found"));
}

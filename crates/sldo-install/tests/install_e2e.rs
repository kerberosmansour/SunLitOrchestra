//! End-to-end tests for `sldo-install`.
//!
//! Each test builds a fake source skills directory and a fake HOME inside a
//! `tempfile::TempDir`, so no user state is touched. Tests invoke the
//! installer's library-exposed logic via integration test — that means this
//! crate re-runs install/uninstall against those fakes and asserts
//! filesystem state.
//!
//! We intentionally shell out to the compiled binary where CLI behavior
//! matters (help text, exit codes), and call the library directly where
//! we want to inspect plan state.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Find the path to the sldo-install binary produced by the current test build.
fn binary_path() -> PathBuf {
    // cargo provides CARGO_BIN_EXE_<name> at test time.
    PathBuf::from(env!("CARGO_BIN_EXE_sldo-install"))
}

fn make_skill(skills_dir: &Path, name: &str) {
    let dir = skills_dir.join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("SKILL.md"),
        format!("---\nname: {}\ndescription: test\n---\nbody", name),
    )
    .unwrap();
}

fn assert_managed_link_to(link: &Path, source: &Path) {
    assert!(
        link.exists() || link.is_symlink(),
        "expected managed link at {}",
        link.display()
    );
    let resolved = fs::canonicalize(link).unwrap();
    let expected = fs::canonicalize(source).unwrap();
    assert_eq!(resolved, expected);
}

#[cfg(unix)]
fn create_test_dir_link(source: &Path, target: &Path) {
    std::os::unix::fs::symlink(source, target).unwrap();
}

#[cfg(windows)]
fn create_test_dir_link(source: &Path, target: &Path) {
    if std::os::windows::fs::symlink_dir(source, target).is_ok() {
        return;
    }

    let output = Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(target)
        .arg(source)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "failed to create Windows test junction: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
fn remove_test_link(target: &Path) {
    fs::remove_file(target).unwrap();
}

#[cfg(windows)]
fn remove_test_link(target: &Path) {
    fs::remove_dir(target)
        .or_else(|_| fs::remove_file(target))
        .unwrap();
}

fn sldo_install(home: &Path, skills_dir: &Path, extra: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(binary_path());
    cmd.env("HOME", home).arg("--skills-dir").arg(skills_dir);
    for a in extra {
        cmd.arg(a);
    }
    cmd.output().expect("failed to exec sldo-install")
}

#[test]
fn test_full_install_uninstall_cycle() {
    // Given: a fake home and a skills dir with two skills
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");
    make_skill(src.path(), "get-api-docs");

    // When: install runs
    let out = sldo_install(home.path(), src.path(), &["install"]);
    assert!(
        out.status.success(),
        "install failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Then: symlinks exist and manifest records them
    let target_root = home.path().join(".claude").join("skills");
    assert_managed_link_to(
        &target_root.join("slo-ideate"),
        &src.path().join("slo-ideate"),
    );
    assert_managed_link_to(
        &target_root.join("get-api-docs"),
        &src.path().join("get-api-docs"),
    );

    let manifest_path = home.path().join(".sldo").join("install.toml");
    assert!(manifest_path.exists(), "manifest not written");
    let manifest_content = fs::read_to_string(&manifest_path).unwrap();
    assert!(manifest_content.contains("slo-ideate"));
    assert!(manifest_content.contains("get-api-docs"));

    // When: uninstall runs
    let out = sldo_install(home.path(), src.path(), &["uninstall"]);
    assert!(
        out.status.success(),
        "uninstall failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Then: symlinks and manifest are gone
    assert!(!target_root.join("slo-ideate").exists());
    assert!(!target_root.join("get-api-docs").exists());
    assert!(
        !manifest_path.exists(),
        "manifest still present after uninstall"
    );
}

#[test]
fn test_idempotent_double_install() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-alpha");

    let out1 = sldo_install(home.path(), src.path(), &["install"]);
    assert!(out1.status.success());
    let out2 = sldo_install(home.path(), src.path(), &["install"]);
    assert!(
        out2.status.success(),
        "second run failed: {}",
        String::from_utf8_lossy(&out2.stderr)
    );

    let stdout2 = String::from_utf8_lossy(&out2.stdout);
    assert!(
        stdout2.contains("already installed") || stdout2.contains("no changes"),
        "expected idempotent output, got: {stdout2}"
    );
}

#[test]
fn test_force_overwrites_existing_symlink() {
    let home = TempDir::new().unwrap();
    let src1 = TempDir::new().unwrap();
    let src2 = TempDir::new().unwrap();
    make_skill(src1.path(), "slo-clash");
    make_skill(src2.path(), "slo-clash");

    let out1 = sldo_install(home.path(), src1.path(), &["install"]);
    assert!(out1.status.success());

    // Second install from a different source — without --force, should fail.
    let out2 = sldo_install(home.path(), src2.path(), &["install"]);
    assert!(
        !out2.status.success(),
        "expected non-zero exit without --force"
    );

    // With --force, should replace.
    let out3 = sldo_install(home.path(), src2.path(), &["install", "--force"]);
    assert!(
        out3.status.success(),
        "force install failed: {}",
        String::from_utf8_lossy(&out3.stderr)
    );

    let link = home.path().join(".claude").join("skills").join("slo-clash");
    let resolved = fs::canonicalize(&link).unwrap();
    let expected = fs::canonicalize(src2.path().join("slo-clash")).unwrap();
    assert_eq!(resolved, expected);
}

#[test]
fn test_dry_run_writes_nothing() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-dry");

    let out = sldo_install(home.path(), src.path(), &["install", "--dry-run"]);
    assert!(out.status.success());

    let target = home.path().join(".claude").join("skills").join("slo-dry");
    assert!(!target.exists(), "--dry-run created a symlink");
    let manifest = home.path().join(".sldo").join("install.toml");
    assert!(!manifest.exists(), "--dry-run wrote a manifest");
}

#[test]
fn test_local_install_into_project() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-local");

    let project = TempDir::new().unwrap();
    let mut cmd = Command::new(binary_path());
    let out = cmd
        .current_dir(project.path())
        .env("HOME", home.path())
        .arg("--skills-dir")
        .arg(src.path())
        .arg("--local")
        .arg("install")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "local install failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let link = project
        .path()
        .join(".claude")
        .join("skills")
        .join("slo-local");
    assert_managed_link_to(&link, &src.path().join("slo-local"));

    // Global manifest should not exist.
    let global_manifest = home.path().join(".sldo").join("install.toml");
    assert!(
        !global_manifest.exists(),
        "local install wrote global manifest"
    );

    let local_manifest = project.path().join(".claude").join("slo-install.toml");
    assert!(local_manifest.exists(), "local manifest not written");
}

#[test]
fn test_missing_skills_dir_fails_loud() {
    let home = TempDir::new().unwrap();
    let bogus = home.path().join("nope-not-here");
    let out = sldo_install(home.path(), &bogus, &["install"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("does not exist") || stderr.contains("not"),
        "expected missing-dir error, got stderr: {stderr}"
    );
}

#[test]
fn test_empty_skills_dir_ok() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    // Empty, no SKILL.md anywhere.
    let out = sldo_install(home.path(), src.path(), &["install"]);
    assert!(out.status.success(), "empty dir should succeed cleanly");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("no changes")
            || stdout.contains("no skills")
            || stdout.contains("installed 0")
    );
}

#[test]
fn test_skill_without_skill_md_is_skipped() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    fs::create_dir_all(src.path().join("not-a-skill")).unwrap();
    make_skill(src.path(), "slo-real");

    let out = sldo_install(home.path(), src.path(), &["install"]);
    assert!(out.status.success());

    let target_root = home.path().join(".claude").join("skills");
    assert_managed_link_to(&target_root.join("slo-real"), &src.path().join("slo-real"));
    assert!(!target_root.join("not-a-skill").exists());
}

#[test]
fn test_help_flag_exits_clean() {
    let out = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("failed to exec");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("sldo-install"));
    assert!(stdout.contains("skills-dir"));
}

#[test]
fn test_uninstall_preserves_user_modified_symlink() {
    // If the user replaces our symlink with one pointing elsewhere, uninstall
    // should leave it alone and warn — not clobber their customization.
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-keep");

    let out = sldo_install(home.path(), src.path(), &["install"]);
    assert!(out.status.success());

    // Replace the symlink with a different source.
    let other = TempDir::new().unwrap();
    let other_target = other.path().join("whatever");
    fs::create_dir_all(&other_target).unwrap();
    let link = home.path().join(".claude").join("skills").join("slo-keep");
    remove_test_link(&link);
    create_test_dir_link(&other_target, &link);

    let out = sldo_install(home.path(), src.path(), &["uninstall"]);
    assert!(out.status.success());

    // Link should still exist (user's customization preserved).
    assert_managed_link_to(&link, &other_target);
}

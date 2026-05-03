use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn binary_path() -> PathBuf {
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

fn sldo_install(home: &Path, skills_dir: &Path, extra: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(binary_path());
    cmd.env("HOME", home).arg("--skills-dir").arg(skills_dir);
    for arg in extra {
        cmd.arg(arg);
    }
    cmd.output().expect("failed to exec sldo-install")
}

fn manifest_path(home: &Path) -> PathBuf {
    home.join(".sldo/install.toml")
}

#[test]
fn test_claude_copilot_and_codex_roots_are_distinct() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    let claude = sldo_install(home.path(), src.path(), &["install"]);
    assert!(
        claude.status.success(),
        "claude install failed: {}",
        String::from_utf8_lossy(&claude.stderr)
    );

    let copilot = sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "install"],
    );
    assert!(
        copilot.status.success(),
        "copilot install failed: {}",
        String::from_utf8_lossy(&copilot.stderr)
    );

    let codex = sldo_install(home.path(), src.path(), &["--host", "codex", "install"]);
    assert!(
        codex.status.success(),
        "codex install failed: {}",
        String::from_utf8_lossy(&codex.stderr)
    );

    assert!(home.path().join(".claude/skills/slo-ideate").is_symlink());
    assert!(home.path().join(".copilot/skills/slo-ideate").is_symlink());
    assert!(home.path().join(".codex/skills/slo-ideate").is_symlink());

    let manifest = fs::read_to_string(home.path().join(".sldo/install.toml")).unwrap();
    assert!(manifest.contains("host = \"claude-code\""));
    assert!(manifest.contains("host = \"github-copilot\""));
    assert!(manifest.contains("host = \"codex\""));
}

#[test]
fn test_uninstall_only_removes_selected_host_entries() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    assert!(sldo_install(home.path(), src.path(), &["install"])
        .status
        .success());
    assert!(sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "install"],
    )
    .status
    .success());
    assert!(
        sldo_install(home.path(), src.path(), &["--host", "codex", "install"])
            .status
            .success()
    );

    let uninstall = sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "uninstall"],
    );
    assert!(
        uninstall.status.success(),
        "copilot uninstall failed: {}",
        String::from_utf8_lossy(&uninstall.stderr)
    );

    assert!(home.path().join(".claude/skills/slo-ideate").is_symlink());
    assert!(!home.path().join(".copilot/skills/slo-ideate").exists());
    assert!(home.path().join(".codex/skills/slo-ideate").is_symlink());

    let manifest_path = home.path().join(".sldo/install.toml");
    let manifest = fs::read_to_string(&manifest_path).expect("manifest should keep claude entries");
    assert!(manifest.contains("host = \"claude-code\""));
    assert!(!manifest.contains("host = \"github-copilot\""));
    assert!(manifest.contains("host = \"codex\""));
}

#[test]
fn test_status_and_verify_report_selected_host() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    let install = sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "install"],
    );
    assert!(install.status.success());

    let status = sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "status"],
    );
    assert!(status.status.success());
    let status_stdout = String::from_utf8_lossy(&status.stdout);
    assert!(status_stdout.contains("github-copilot"));

    let verify = sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "verify"],
    );
    assert!(verify.status.success());
    let verify_stdout = String::from_utf8_lossy(&verify.stdout);
    assert!(verify_stdout.contains("github-copilot"));
}

#[test]
fn test_codex_status_and_verify_report_selected_host() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    let install = sldo_install(home.path(), src.path(), &["--host", "codex", "install"]);
    assert!(install.status.success());

    let status = sldo_install(home.path(), src.path(), &["--host", "codex", "status"]);
    assert!(status.status.success());
    let status_stdout = String::from_utf8_lossy(&status.stdout);
    assert!(status_stdout.contains("codex"));

    let verify = sldo_install(home.path(), src.path(), &["--host", "codex", "verify"]);
    assert!(verify.status.success());
    let verify_stdout = String::from_utf8_lossy(&verify.stdout);
    assert!(verify_stdout.contains("codex"));
}

#[test]
fn test_github_copilot_local_install_uses_repo_state() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-local");

    let project = TempDir::new().unwrap();
    let out = Command::new(binary_path())
        .current_dir(project.path())
        .env("HOME", home.path())
        .arg("--skills-dir")
        .arg(src.path())
        .arg("--host")
        .arg("github-copilot")
        .arg("--local")
        .arg("install")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "local copilot install failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(project
        .path()
        .join(".copilot/skills/slo-local")
        .is_symlink());
    assert!(project.path().join(".copilot/slo-install.toml").exists());
    assert!(!home.path().join(".copilot/skills/slo-local").exists());
}

#[test]
fn test_codex_local_install_uses_repo_state() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-local");

    let project = TempDir::new().unwrap();
    let out = Command::new(binary_path())
        .current_dir(project.path())
        .env("HOME", home.path())
        .arg("--skills-dir")
        .arg(src.path())
        .arg("--host")
        .arg("codex")
        .arg("--local")
        .arg("install")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "local codex install failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(project.path().join(".codex/skills/slo-local").is_symlink());
    assert!(project.path().join(".codex/slo-install.toml").exists());
    assert!(!home.path().join(".codex/skills/slo-local").exists());
}

#[test]
fn test_unknown_host_fails_loud() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    let out = sldo_install(
        home.path(),
        src.path(),
        &["--host", "not-a-host", "install"],
    );
    assert!(!out.status.success(), "unknown host should fail");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("claude-code"));
    assert!(stderr.contains("github-copilot"));
    assert!(stderr.contains("codex"));
}

#[test]
fn test_legacy_manifest_upgrades_without_breaking_uninstall() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    let install = sldo_install(home.path(), src.path(), &["install"]);
    assert!(install.status.success());

    let manifest_path = manifest_path(home.path());
    let legacy_manifest = fs::read_to_string(&manifest_path)
        .unwrap()
        .replace("schema_version = 2", "schema_version = 1")
        .replace("host = \"claude-code\"\n", "");
    fs::write(&manifest_path, legacy_manifest).unwrap();

    let uninstall = sldo_install(home.path(), src.path(), &["uninstall"]);
    assert!(
        uninstall.status.success(),
        "legacy uninstall failed: {}",
        String::from_utf8_lossy(&uninstall.stderr)
    );

    assert!(!home.path().join(".claude/skills/slo-ideate").exists());
    assert!(!manifest_path.exists());
}

#[test]
fn test_install_root_escape_refused_on_verify() {
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    make_skill(src.path(), "slo-ideate");

    let install = sldo_install(home.path(), src.path(), &["install"]);
    assert!(install.status.success());

    let manifest_path = manifest_path(home.path());
    let mismatched_manifest = fs::read_to_string(&manifest_path)
        .unwrap()
        .replace("host = \"claude-code\"", "host = \"github-copilot\"");
    fs::write(&manifest_path, mismatched_manifest).unwrap();

    let verify = sldo_install(
        home.path(),
        src.path(),
        &["--host", "github-copilot", "verify"],
    );
    assert!(
        !verify.status.success(),
        "verify should reject escaped roots"
    );

    let stderr = String::from_utf8_lossy(&verify.stderr);
    assert!(stderr.contains("outside the selected host root"));
    assert!(home.path().join(".claude/skills/slo-ideate").is_symlink());
}

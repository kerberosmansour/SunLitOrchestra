//! M2 E2E: verifies that /slo-ideate and /slo-retro are well-formed and
//! discoverable by the installer.
//!
//! These tests do not exercise the runtime behavior of the skills (that
//! happens when Claude Code actually runs them). They verify the static
//! contract: the SKILL.md file has valid YAML frontmatter with the required
//! fields, and the installer picks both up and installs them as symlinks.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points at crates/sldo-install; go up two levels.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sldo-install"))
}

/// Minimal frontmatter + body check. We don't pull in a YAML parser — we just
/// validate the shape so a malformed SKILL.md doesn't slip through.
fn assert_valid_skill_md(path: &Path) {
    let body = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    assert!(
        body.starts_with("---\n"),
        "SKILL.md must start with `---\\n`: {}",
        path.display()
    );
    // Find the closing `---` on its own line after the first one.
    let after = &body[4..];
    let end_idx = after
        .find("\n---\n")
        .or_else(|| after.find("\n---"))
        .unwrap_or_else(|| panic!("no closing `---` in {}", path.display()));
    let frontmatter = &after[..end_idx];
    assert!(
        frontmatter.contains("name:"),
        "frontmatter missing `name:`: {}",
        path.display()
    );
    assert!(
        frontmatter.contains("description:"),
        "frontmatter missing `description:`: {}",
        path.display()
    );
    let rest = &after[end_idx..];
    assert!(
        rest.trim().len() > 20,
        "skill body suspiciously short in {}",
        path.display()
    );
}

#[test]
fn test_slo_ideate_skill_is_valid() {
    let path = repo_root().join("skills").join("slo-ideate").join("SKILL.md");
    assert!(path.exists(), "slo-ideate SKILL.md missing");
    assert_valid_skill_md(&path);
}

#[test]
fn test_slo_retro_skill_is_valid() {
    let path = repo_root().join("skills").join("slo-retro").join("SKILL.md");
    assert!(path.exists(), "slo-retro SKILL.md missing");
    assert_valid_skill_md(&path);
}

#[test]
fn test_installer_picks_up_ideate_and_retro() {
    // Build a scratch skills dir with copies of just these two skills, so the
    // test is hermetic from whatever else exists in the repo now or later.
    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();

    for name in ["slo-ideate", "slo-retro"] {
        let source_file = repo_root().join("skills").join(name).join("SKILL.md");
        let dest_dir = src.path().join(name);
        fs::create_dir_all(&dest_dir).unwrap();
        fs::copy(&source_file, dest_dir.join("SKILL.md")).unwrap();
    }

    let out = Command::new(binary_path())
        .env("HOME", home.path())
        .arg("--skills-dir")
        .arg(src.path())
        .arg("install")
        .output()
        .expect("exec sldo-install");
    assert!(
        out.status.success(),
        "install failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let skills_root = home.path().join(".claude").join("skills");
    for name in ["slo-ideate", "slo-retro"] {
        let link = skills_root.join(name);
        assert!(
            link.is_symlink(),
            "expected symlink at {}",
            link.display()
        );
    }
}

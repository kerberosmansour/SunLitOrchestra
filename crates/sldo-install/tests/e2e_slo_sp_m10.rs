//! M10 E2E: get-api-docs third-party skill vendored and installable.

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn skill_dir() -> PathBuf {
    repo_root().join("skills").join("get-api-docs")
}

#[test]
fn get_api_docs_skill_md_exists_and_is_valid() {
    let body =
        fs::read_to_string(skill_dir().join("SKILL.md")).expect("get-api-docs SKILL.md missing");
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: get-api-docs"));
    assert!(body.contains("description:"));
    assert!(body.to_lowercase().contains("chub"));
}

#[test]
fn upstream_md_present_with_source_and_commit() {
    let body = fs::read_to_string(skill_dir().join("UPSTREAM.md"))
        .expect("get-api-docs UPSTREAM.md missing");
    assert!(
        body.contains("andrewyng/context-hub"),
        "must cite upstream repo"
    );
    assert!(
        body.contains("Commit") || body.contains("commit"),
        "must record the source commit"
    );
    // 40-char hex is the minimum; substring match of a known fetched hash.
    assert!(
        body.contains("596506ebb4d53cfbc6ae458b731e0b1a18790f9e") || body.contains("Fetched on"),
        "must record the specific commit and fetch date"
    );
    assert!(body.contains("MIT"), "must cite the upstream license");
}

#[test]
fn get_api_docs_installs_through_generic_pickup() {
    use std::process::Command;
    use tempfile::TempDir;

    let home = TempDir::new().unwrap();
    let src = TempDir::new().unwrap();
    let skill_src = src.path().join("get-api-docs");
    fs::create_dir_all(&skill_src).unwrap();
    fs::copy(skill_dir().join("SKILL.md"), skill_src.join("SKILL.md")).unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_sldo-install"))
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

    let link = home
        .path()
        .join(".claude")
        .join("skills")
        .join("get-api-docs");
    assert!(
        link.is_symlink(),
        "get-api-docs should install via the generic symlink path — no special case"
    );
}

#[test]
fn claude_md_lists_get_api_docs() {
    let claude_md = fs::read_to_string(repo_root().join("CLAUDE.md")).expect("CLAUDE.md missing");
    assert!(
        claude_md.to_lowercase().contains("get-api-docs"),
        "CLAUDE.md must list the get-api-docs skill so sessions discover it"
    );
    assert!(
        claude_md.contains("@aisuite/chub") || claude_md.contains("chub"),
        "CLAUDE.md must name the chub prerequisite"
    );
}

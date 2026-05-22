//! M5 structural-contract test (measurement-loop runbook).
//!
//! Asserts the Feature-performance loop is catalogued:
//!
//! - `docs/LOOPS-ENGINEERING.md` carries a `## Feature-performance loop` entry
//!   in the standard loop-entry format (User-visible outcome / Trigger / Steps
//!   / Exit condition / Artifacts / Skills involved).
//! - `docs/LOOPS-BUSINESS.md` carries a CROSS-REFERENCE to it (not a full
//!   duplicate) — single canonical home in the engineering doc.
//! - Existing loop entries in both docs are preserved (no restructure).

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    let p = workspace_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("failed to read {}: {}", p.display(), e))
}

fn eng() -> String {
    read("docs/LOOPS-ENGINEERING.md")
}
fn biz() -> String {
    read("docs/LOOPS-BUSINESS.md")
}

const HEADING: &str = "## Feature-performance loop";

/// The text of the `## Feature-performance loop` section (up to the next `## `).
fn loop_section(content: &str) -> String {
    let start = content
        .find(HEADING)
        .expect("LOOPS-ENGINEERING.md must contain a `## Feature-performance loop` heading");
    let after = &content[start + HEADING.len()..];
    let end = after.find("\n## ").unwrap_or(after.len());
    after[..end].to_string()
}

#[test]
fn feature_performance_loop_entry_present() {
    let section = loop_section(&eng());
    for label in [
        "User-visible outcome",
        "Trigger",
        "Steps",
        "Exit condition",
        "Artifacts",
        "Skills involved",
    ] {
        assert!(
            section.contains(label),
            "Feature-performance loop must include the standard `{label}` sub-heading"
        );
    }
}

#[test]
fn feature_performance_loop_cross_ref_present() {
    assert!(
        biz().contains("Feature-performance loop"),
        "LOOPS-BUSINESS.md must cross-reference the Feature-performance loop"
    );
}

#[test]
fn feature_performance_loop_single_home_no_duplication() {
    // The full loop lives once (engineering doc); the business doc carries a
    // cross-reference, NOT a duplicated `## Feature-performance loop` section.
    assert!(
        !biz().contains(HEADING),
        "LOOPS-BUSINESS.md must NOT duplicate the full `## Feature-performance loop` section — cross-reference only"
    );
}

#[test]
fn existing_loops_preserved() {
    let e = eng();
    for h in [
        "## Sprint loop",
        "## Lessons loop",
        "## Library-feedback loop",
    ] {
        assert!(
            e.contains(h),
            "existing engineering loop `{h}` must be preserved"
        );
    }
    let b = biz();
    for h in ["## GTM loop", "## Pricing loop", "## User-interview loop"] {
        assert!(
            b.contains(h),
            "existing business loop `{h}` must be preserved"
        );
    }
}

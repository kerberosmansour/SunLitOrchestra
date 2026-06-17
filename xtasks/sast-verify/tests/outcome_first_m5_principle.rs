//! M5 structural-contract test (outcome-first runbook).
//!
//! The Outcome First Engineering principle is binding + discoverable:
//! - `references/agent/operating-contract.md` carries the host-neutral principle.
//! - `docs/skill-pack-catalog.md` names the Outcome Validation gate (skill-count
//!   line untouched).
//! - `docs/LOOPS-ENGINEERING.md` documents the Sprint-loop Outcome-First overlay
//!   + the inverted-authority pyramid (Secure Value overlay preserved).
//!
//! Docs-only milestone — no SKILL.md edits, so no SHA pin; marker assertions.

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

const CONTRACT: &str = "references/agent/operating-contract.md";
const CATALOG: &str = "docs/skill-pack-catalog.md";
const LOOPS: &str = "docs/LOOPS-ENGINEERING.md";

#[test]
fn operating_contract_has_outcome_first_principle() {
    let c = read(CONTRACT);
    let lc = c.to_lowercase();
    assert!(
        c.contains("Outcome First Engineering"),
        "operating-contract.md must add the Outcome First Engineering principle"
    );
    assert!(
        lc.contains("code completion alone is insufficient"),
        "the principle must state that code completion alone is insufficient"
    );
    assert!(
        lc.contains("existing important outcomes still exist"),
        "the principle must require existing important outcomes to still exist"
    );
}

#[test]
fn operating_contract_principle_host_neutral() {
    // Scope the host-neutral check to the PRINCIPLE section only — the file
    // elsewhere legitimately names hosts in "Keep Host Boundaries Honest".
    let c = read(CONTRACT);
    let start = c
        .find("## Outcome First Engineering")
        .expect("principle section present");
    let after = &c[start + "## Outcome First Engineering".len()..];
    let end = after.find("\n## ").unwrap_or(after.len());
    let section = &after[..end];
    assert!(
        !section.contains("Playwright") && !section.contains("Claude Code"),
        "the Outcome First principle must be host-neutral (no Playwright / Claude Code in the section)"
    );
}

#[test]
fn catalog_names_outcome_gate() {
    let c = read(CATALOG);
    assert!(
        c.contains("Outcome Validation") || c.contains("Outcome First"),
        "the catalog must name the Outcome Validation gate / Outcome First principle"
    );
}

#[test]
fn catalog_skill_count_preserved() {
    let c = read(CATALOG);
    assert!(
        c.contains("Shipped skills at HEAD: 49"),
        "M5 must NOT change the catalog skill-count line"
    );
}

#[test]
fn loops_has_outcome_first_overlay_and_pyramid() {
    let c = read(LOOPS);
    let lc = c.to_lowercase();
    assert!(
        c.contains("Outcome-First overlay"),
        "LOOPS-ENGINEERING.md must add the Sprint-loop Outcome-First overlay"
    );
    assert!(
        lc.contains("highest authority") && c.contains("OUTCOME"),
        "the Outcome-First overlay must document the inverted-authority pyramid (OUTCOME = highest authority)"
    );
}

#[test]
fn loops_secure_value_overlay_preserved() {
    assert!(
        read(LOOPS).contains("### Secure Value Loop overlay"),
        "the existing Secure Value Loop overlay must be preserved (additive insertion)"
    );
}

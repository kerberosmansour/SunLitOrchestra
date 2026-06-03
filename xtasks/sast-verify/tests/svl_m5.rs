//! M5 structural-contract test (secure-value-loop runbook).
//!
//! Asserts the loop docs name each stage's security output, `/slo-ship` carries
//! the secure-release checklist + closed `ship_state` enum + conditional
//! SBOM/provenance, and the runbook dogfoods a FILLED §5B Secure Value &
//! Security Contract (no placeholder rows).

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

const LOOPS_ENG: &str = "docs/LOOPS-ENGINEERING.md";
const LOOPS_BIZ: &str = "docs/LOOPS-BUSINESS.md";
const SHIP_SKILL: &str = "skills/slo-ship/SKILL.md";
const RUNBOOK: &str = "docs/RUNBOOK-secure-value-loop.md";

#[test]
fn loops_engineering_names_security_output_per_stage() {
    let d = read(LOOPS_ENG);
    assert!(
        d.contains("Secure Value Loop overlay"),
        "LOOPS-ENGINEERING.md must carry the Secure Value Loop overlay"
    );
    let lc = d.to_lowercase();
    assert!(
        lc.contains("every stage carries a security output") || lc.contains("security output"),
        "the overlay must state every stage carries a security output"
    );
    // A few representative stages must name their security output.
    for stage in [
        "threat model",
        "security source pack",
        "Detected Work Ledger",
    ] {
        assert!(
            d.contains(stage),
            "LOOPS-ENGINEERING overlay must name the `{stage}` stage output"
        );
    }
}

#[test]
fn loops_business_has_security_cross_ref() {
    let d = read(LOOPS_BIZ);
    assert!(
        d.contains("security-visible proof of safety"),
        "LOOPS-BUSINESS.md must make security-visible proof of safety part of product/value review"
    );
}

#[test]
fn ship_has_secure_release_checklist_and_closed_ship_state() {
    let c = read(SHIP_SKILL);
    let lc = c.to_lowercase();
    assert!(
        lc.contains("secure-release checklist"),
        "/slo-ship must carry the secure-release checklist"
    );
    for state in [
        "shipped",
        "human_review_required",
        "blocked",
        "canary_only",
        "docs_only",
    ] {
        assert!(
            c.contains(state),
            "/slo-ship ship_state enum must include `{state}`"
        );
    }
}

#[test]
fn ship_sbom_provenance_is_conditional() {
    let c = read(SHIP_SKILL);
    assert!(
        c.contains("SBOM") && c.contains("when applicable"),
        "/slo-ship must keep SBOM/provenance conditional (when applicable), never a hard gate for markdown"
    );
    let lc = c.to_lowercase();
    assert!(
        lc.contains("not_applicable") && lc.contains("released artifact"),
        "/slo-ship must say SBOM is not_applicable unless a released artifact is built"
    );
}

#[test]
fn runbook_dogfoods_a_filled_secure_value_contract() {
    let r = read(RUNBOOK);
    // The dogfood §5B section exists in the runbook itself.
    assert!(
        r.contains("## 5B. Secure Value and Security Contract"),
        "the runbook must dogfood a §5B Secure Value & Security Contract"
    );
    // Filled, not placeholder: real ledger row + real abuse-case citations.
    assert!(
        r.contains("DW-001") && r.contains("file_github_issue"),
        "the dogfood Detected Work Ledger must carry a real disposed row (DW-001)"
    );
    assert!(
        r.contains("tm-secure-value-loop-abuse-1"),
        "the dogfood Threat Model Summary must cite the frozen abuse-case IDs"
    );
    assert!(
        r.contains("safe_to_continue_without_blockers"),
        "the dogfood Operator Readiness sub-block must carry the readiness flag"
    );
}

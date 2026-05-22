//! M5 structural-contract test (kani-verification runbook).
//!
//! Asserts the TLA+↔Kani pairing + the local deep-verification workflow, and
//! that NO Kani CI automation was added (v1 decision: deep proofs run locally).
//!
//! - The pairing doc carries the refinement map (action → fn → harness) and the
//!   boundary invariant ("Kani never claims what TLA+ owns").
//! - The local-deep-verification reference defines a quick tier and a deep tier,
//!   uses the pinned toolchain, and states deep-before-release.
//! - No `.github/workflows/*` file mentions Kani (no CI automation in v1).
//! - `/slo-tla` carries the reciprocal `/slo-kani` handoff note.
//!
//! Prose-gate substrings are matched case-insensitively (kani-m2 lesson).

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

fn read_lc(rel: &str) -> String {
    std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {rel}: {e}"))
        .to_lowercase()
}

#[test]
fn pairing_doc_has_refinement_map_and_invariant() {
    let t = read_lc("docs/slo/design/kani-verification-kani-pairing.md");
    assert!(
        t.contains("action") && t.contains("harness") && t.contains("→"),
        "pairing doc must carry the refinement map (action → fn → harness)"
    );
    assert!(
        t.contains("kani never claims what tla+ owns"),
        "pairing doc must carry the boundary invariant (`Kani never claims what TLA+ owns`) — tm-kani-verification-abuse-2"
    );
    assert!(
        t.contains("check_gcd_contract") || t.contains("check_zero_prefix"),
        "pairing doc must reference a worked example using an M4 demo harness"
    );
}

#[test]
fn local_deep_verification_documents_quick_and_deep_tiers() {
    let t = read_lc("skills/slo-kani/references/local-deep-verification.md");
    assert!(
        t.contains("quick") && t.contains("deep"),
        "local-deep-verification.md must define both a quick and a deep tier"
    );
    assert!(
        t.contains("before") && t.contains("release"),
        "local-deep-verification.md must state the deep tier runs green before a release"
    );
}

#[test]
fn local_workflow_uses_pinned_toolchain() {
    let doc = read_lc("skills/slo-kani/references/local-deep-verification.md");
    let toml = read_lc("skills/slo-kani/tools.toml");
    // Extract the pinned version (digit-leading) from tools.toml.
    let pin = toml
        .lines()
        .find_map(|l| {
            let l = l.trim();
            l.strip_prefix("version")
                .and_then(|r| r.split('"').nth(1))
                .filter(|v| {
                    v.chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                })
                .map(|v| v.to_string())
        })
        .expect("tools.toml must pin a concrete version");
    assert!(
        doc.contains("tools.toml") && doc.contains(&pin),
        "local-deep-verification.md must use the pinned tools.toml version ({pin}), not `latest`"
    );
}

#[test]
fn no_kani_ci_workflow_added() {
    // v1 decision: deep proofs run locally; no Kani CI automation.
    let dir = workspace_root().join(".github/workflows");
    if !dir.exists() {
        return;
    }
    let mut offenders: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("yml")
            && path.extension().and_then(|e| e.to_str()) != Some("yaml")
        {
            continue;
        }
        let content = std::fs::read_to_string(&path)
            .unwrap_or_default()
            .to_lowercase();
        if content.contains("cargo kani") || content.contains("kani-verifier") {
            offenders.push(path.file_name().unwrap().to_string_lossy().to_string());
        }
    }
    assert!(
        offenders.is_empty(),
        "no Kani CI workflow may be added in v1 (deep proofs run locally) — found: {offenders:?}"
    );
}

#[test]
fn slo_tla_carries_reciprocal_kani_note() {
    let t = read_lc("skills/slo-tla/SKILL.md");
    assert!(
        t.contains("slo-kani"),
        "slo-tla/SKILL.md must carry the reciprocal /slo-kani handoff note (Rust-kernel verification)"
    );
}

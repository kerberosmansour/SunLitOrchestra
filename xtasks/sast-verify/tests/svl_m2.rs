//! M2 structural-contract test (secure-value-loop runbook).
//!
//! Asserts `/slo-plan` requires the §5B Secure Value & Security Contract for
//! value-bearing OR security-relevant milestones, forward-looking (never
//! retroactively invalidating legacy runbooks), with the security-relevant
//! trigger list and the inert-window note (F-ENG-3).

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

const PLAN_SKILL: &str = "skills/slo-plan/SKILL.md";

#[test]
fn plan_requires_secure_value_contract() {
    let c = read(PLAN_SKILL);
    let lc = c.to_lowercase();
    assert!(
        c.contains("Secure Value") && (c.contains("5B") || lc.contains("security contract")),
        "/slo-plan must require the §5B Secure Value & Security Contract"
    );
    assert!(
        lc.contains("value-bearing") && lc.contains("security-relevant"),
        "/slo-plan must require §5B for value-bearing OR security-relevant milestones"
    );
}

#[test]
fn requirement_is_forward_looking_not_retroactive() {
    let lc = read(PLAN_SKILL).to_lowercase();
    assert!(
        lc.contains("flag") && lc.contains("legacy"),
        "/slo-plan must flag a missing §5B without invalidating legacy runbooks (forward-looking)"
    );
}

#[test]
fn security_relevant_triggers_listed() {
    // The deterministic trigger list (proposal §9 #9 / interfaces).
    let lc = read(PLAN_SKILL).to_lowercase();
    let triggers = [
        "identity",
        "secrets",
        "pii",
        "cloud",
        "ai",
        "public",
        "ci/cd",
        "infrastructure",
    ];
    let present = triggers.iter().filter(|t| lc.contains(**t)).count();
    assert!(
        present >= 6,
        "/slo-plan must enumerate the security-relevant trigger list (found {present}/8 of: identity, secrets, PII, cloud, AI, public, CI/CD, infrastructure)"
    );
}

#[test]
fn inert_window_note_documented() {
    // F-ENG-3: the M2 mandate lands before the M3 consumer; the generated §5B
    // must say the gate is enforced from the M3 release onward.
    let c = read(PLAN_SKILL);
    let lc = c.to_lowercase();
    assert!(
        lc.contains("enforced") && lc.contains("m3"),
        "/slo-plan must document the inert window (Operator Readiness Gate enforced from the M3 release onward) — F-ENG-3"
    );
}

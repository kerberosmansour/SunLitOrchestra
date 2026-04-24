//! M8 E2E: power tools contracts.

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

fn skill(name: &str) -> String {
    fs::read_to_string(repo_root().join("skills").join(name).join("SKILL.md"))
        .unwrap_or_else(|e| panic!("{} SKILL.md missing: {e}", name))
}

#[test]
fn all_power_tools_present_and_valid() {
    for name in &["slo-second-opinion", "slo-freeze", "slo-resume", "slo-ship"] {
        let body = skill(name);
        assert!(body.starts_with("---\n"), "{name} frontmatter broken");
        assert!(body.contains(&format!("name: {name}")), "{name} frontmatter missing name");
    }
}

#[test]
fn second_opinion_handles_missing_providers() {
    let body = skill("slo-second-opinion");
    let lower = body.to_lowercase();
    assert!(lower.contains("which codex") || lower.contains("codex cli"));
    assert!(lower.contains("gemini"));
    // Clean exit if neither is present.
    assert!(
        lower.contains("no second-opinion provider") || lower.contains("not found on path"),
        "must surface missing-provider case cleanly"
    );
    // No silent fallback.
    assert!(
        lower.contains("do not silently fall back") || lower.contains("not silently fall back")
            || lower.contains("defeats the purpose"),
        "must forbid silent fallback to Claude-pretending"
    );
}

#[test]
fn second_opinion_is_disagreement_finder_not_arbitrator() {
    let body = skill("slo-second-opinion");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("disagreement") || lower.contains("diff of findings"),
        "skill must frame as disagreement surfacer, not arbitrator"
    );
    assert!(
        lower.contains("not a vote") || lower.contains("not an arbitrator")
            || lower.contains("not a verdict"),
        "skill must explicitly reject vote/arbitration framing"
    );
}

#[test]
fn freeze_refuses_root() {
    let body = skill("slo-freeze");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("refuse to freeze") || lower.contains("not a freeze"),
        "freeze must refuse root / full-session freeze"
    );
}

#[test]
fn resume_reads_tracker_but_doesnt_execute() {
    let body = skill("slo-resume");
    let lower = body.to_lowercase();
    assert!(lower.contains("milestone tracker"));
    assert!(
        lower.contains("do not start") || lower.contains("does not") && lower.contains("execute")
            || lower.contains("orientation, not execution"),
        "resume must orient, not execute"
    );
}

#[test]
fn ship_refuses_on_main() {
    let body = skill("slo-ship");
    let lower = body.to_lowercase();
    assert!(lower.contains("main") && lower.contains("master"));
    assert!(
        lower.contains("refuse") && (lower.contains("master") || lower.contains("main")),
        "ship must refuse to run on main/master"
    );
}

#[test]
fn ship_refuses_non_done_tracker() {
    let body = skill("slo-ship");
    let lower = body.to_lowercase();
    assert!(lower.contains("tracker"));
    assert!(
        lower.contains("not done") || lower.contains("non-done") || lower.contains("not `done`"),
        "ship must refuse when any milestone is not done"
    );
}

#[test]
fn ship_does_not_force_push_or_skip_hooks() {
    let body = skill("slo-ship");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("do not force-push") || lower.contains("never automatic")
            || lower.contains("never force")
            || lower.contains("do not `--force-push`"),
        "ship must prohibit automatic force-push"
    );
    assert!(
        lower.contains("--no-verify") || lower.contains("skip hooks") || lower.contains("hooks exist"),
        "ship must address --no-verify / hook-skipping"
    );
}

#[test]
fn ship_composes_pr_body_from_completions() {
    let body = skill("slo-ship");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("completion") && lower.contains("milestone"),
        "ship must build PR body from milestone completion summaries"
    );
    // Not a diff-stat PR description.
    assert!(
        lower.contains("not a diff-stat") || lower.contains("not a diff stat")
            || lower.contains("diff-stat") && lower.contains("anti-pattern"),
        "ship must reject diff-stat PR descriptions"
    );
}

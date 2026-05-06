//! M4 structural-contract tests for `/slo-sec-libs`.
//!
//! These tests assert the explicit upstream filing gate and per-session cap.
//! Live upstream issue creation remains manually gated by user confirmation.

use std::fs;
use std::path::{Path, PathBuf};

use sldo_common::toolflags;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_path() -> PathBuf {
    repo_root().join("skills/slo-sec-libs")
}

fn skill() -> String {
    read(&skill_path().join("SKILL.md"))
}

fn discipline() -> String {
    read(&skill_path().join("references/upstream-filing-discipline.md"))
}

fn cap_decision(filed_this_session: u8) -> &'static str {
    if filed_this_session < 40 {
        "allow-upstream"
    } else {
        "spilled-cap"
    }
}

#[test]
fn file_upstream_flag_documented() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("--file-upstream --upstream-dir <path>"));
    assert!(skill.contains("M4 upstream filing mode"));
    assert!(discipline.contains("`--file-upstream` is an explicit opt-in gate"));
    assert!(discipline.contains("Without it, every valid gap follows the default SLO-intake path"));
}

#[test]
fn default_destination_unchanged_without_file_upstream() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("--file-gaps <m2-output.json> --intake-dir <path>"));
    assert!(skill.contains("M3 default filing mode"));
    assert!(discipline.contains("Default destination: `kerberosmansour/slo-security-intake`"));
    assert!(discipline.contains("default SLO-intake path"));
}

#[test]
fn upstream_owner_mapping_locked() {
    let discipline = discipline();
    for expected in [
        "`hulumi`",
        "`kerberosmansour/hulumi`",
        "`SunLitSecurityLibraries`",
        "`kerberosmansour/SunLitSecurityLibraries`",
        "`unknown` | none | upstream filing refused",
    ] {
        assert!(
            discipline.contains(expected),
            "missing mapping `{expected}`"
        );
    }
    assert!(discipline
        .contains("Ambiguous, unknown, legacy, or mismatched owner values MUST NOT file upstream"));
}

#[test]
fn upstream_confirmation_and_fallback_documented() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("require a yes/no confirmation for upstream filing"));
    assert!(skill.contains("separate confirmation before any intake fallback"));
    assert!(discipline.contains(
        "If the user declines upstream filing, offer the SLO-intake fallback and ask again"
    ));
    assert!(discipline.contains("Resolved intake origin URL"));
    assert!(discipline.contains("Cap counter (`filed_this_session` / 40)"));
}

#[test]
fn forty_per_hour_cap_documented() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("40 issues per session per hour cap"));
    assert!(skill.contains("filed_this_session"));
    assert!(discipline.contains("40 issues per session per hour"));
    assert!(discipline.contains("filed_this_session = 0"));
    assert!(discipline.contains("Allow upstream filings while `filed_this_session < 40`"));
    assert!(
        discipline.contains("41st and later upstream candidates MUST NOT call `gh issue create`")
    );
}

#[test]
fn cap_simulation_allows_first_40_and_spills_41st() {
    for filed in 0..40 {
        assert_eq!(cap_decision(filed), "allow-upstream");
    }
    assert_eq!(cap_decision(40), "spilled-cap");
    assert_eq!(cap_decision(41), "spilled-cap");
}

#[test]
fn cross_session_state_not_persisted() {
    let skill = skill();
    let discipline = discipline();
    let combined = format!("{skill}\n{discipline}");
    assert!(combined.contains("never persist cap state across sessions"));
    assert!(discipline.contains("MUST NOT persist cap state to disk, environment variables, git config, temp files, or any global cache"));
    assert!(discipline.contains("The cap resets when the invocation/session ends"));
    assert!(!combined.to_lowercase().contains("saved counter"));
    assert!(!combined.to_lowercase().contains("global counter"));
}

#[test]
fn lessons_backlog_spillover() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("spill the 41st and later candidates to `LESSONS-BACKLOG.md` with `disposition: spilled-cap`"));
    assert!(discipline.contains("LESSONS-BACKLOG.md"));
    assert!(discipline.contains("capability-gap-schema.md"));
    assert!(discipline.contains("disposition` | `spilled-cap`"));
    assert!(discipline.contains("spillover_reason` | `upstream-cap-40-per-session`"));
}

#[test]
fn no_repo_and_no_merge_flags_still_forbidden() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("Do not use `--repo`"));
    assert!(discipline.contains("NO `--repo` flag"));
    assert!(!discipline.contains("gh issue create --repo"));
    assert!(!discipline.contains("gh issue list --repo"));
    assert!(discipline.contains("Running `gh pr merge`"));
    for flag in ["--auto", "--merge", "--squash", "--rebase", "--admin"] {
        assert!(
            discipline.contains(flag),
            "missing forbidden merge flag `{flag}`"
        );
    }
}

#[test]
fn no_gh_auth_login_from_skill() {
    let skill = skill();
    let discipline = discipline();
    assert!(skill.contains("never run `gh auth login`"));
    assert!(discipline.contains("Never run `gh auth login` from this skill"));
    assert!(discipline.contains("gh auth status"));
}

#[test]
fn m1_m2_m3_contracts_still_present() {
    let root = skill_path();
    let skill = skill();
    assert!(skill.contains("--read-declarations <path>"));
    assert!(skill.contains("--match <runbook.md> --catalog <catalog.json>"));
    assert!(skill.contains("--file-gaps <m2-output.json> --intake-dir <path>"));
    assert!(root.join("scripts/read-declarations.py").is_file());
    assert!(root.join("references/methodology-m1-reader.md").is_file());
    assert!(root.join("references/methodology-m2-matcher.md").is_file());
    assert!(root.join("references/capability-gap-schema.md").is_file());
}

#[test]
fn sec_libs_tool_deny_flags_unchanged() {
    let flags = toolflags::sec_libs_deny_flags();
    let combined = flags.join(",");
    assert!(combined.contains("WebFetch"));
    assert!(combined.contains("WebSearch"));
}

//! M3 structural-contract tests for `/slo-sec-libs`.
//!
//! These tests assert the default SLO-intake filing contract. Live GitHub issue
//! creation remains manually gated because M3 requires per-issue user
//! confirmation before any side effect.

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

fn schema() -> String {
    read(&skill_path().join("references/capability-gap-schema.md"))
}

fn discipline() -> String {
    read(&skill_path().join("references/upstream-filing-discipline.md"))
}

#[test]
fn capability_gap_schema_exists_with_frontmatter() {
    let body = schema();
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-sec-libs-capability-gap-schema"));
    assert!(body.contains("milestone: M3"));
    assert!(body.contains("# /slo-sec-libs M3 Capability-Gap Schema"));
}

#[test]
fn capability_gap_schema_regex_validated() {
    let body = schema();
    for field in [
        "source_repository",
        "source_ref_sha",
        "runbook_path",
        "milestone",
        "proactive_control_row",
        "desired_capability",
        "data_classification",
        "expected_library_owner",
        "match_status",
        "evidence_url_or_path",
        "impact_class",
        "exploitability",
        "alternatives_tried",
        "parametric_requirements",
        "target_repo_context",
        "user_confirmed",
        "body_sha256",
    ] {
        assert!(body.contains(field), "missing schema field `{field}`");
    }

    for rule in [
        "^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$",
        "^[0-9a-f]{40}$",
        "^docs/slo/(future|current|completed)/RUNBOOK-[A-Z0-9][A-Z0-9-]*\\.md$",
        "^M[0-9]+$",
        "^pc-[0-9]{3}$",
        "^[a-z0-9][a-z0-9-]{2,79}$",
        "SunLitSecurityLibraries",
        "unmatched",
        "ambiguous",
        "low-confidence",
        "^(OWASP-C[0-9]+|ASVS-[0-9]+(\\.[0-9]+){1,2}|unknown)$",
        "low`, `medium`, `high`, `unknown",
        "^[0-9a-f]{12}$",
    ] {
        assert!(body.contains(rule), "missing validation rule `{rule}`");
    }
}

#[test]
fn schema_rejects_untrusted_prose_and_unicode_tricks() {
    let body = schema();
    for guard in [
        "Unicode NFKC",
        "zero-width characters U+200B, U+200C, U+200D, and U+FEFF",
        "RTL/LTR override characters U+202E and U+202D",
        "angle brackets `<` and `>`",
        "pipe `|`",
        "<script>",
        "Reject raw target-repo prose",
        "Do not emit legacy `SunLitSecureLibraries`",
    ] {
        assert!(body.contains(guard), "missing guard `{guard}`");
    }
}

#[test]
fn upstream_filing_discipline_argv_list() {
    let body = discipline();
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-sec-libs-upstream-filing-discipline"));
    assert!(body.contains("argv-list"));
    assert!(body.contains(
        "gh issue create --title \"<title>\" --body-file \"<tmpfile>\" --label capability-gap"
    ));
    assert!(body.contains("[\"gh\", \"issue\", \"create\", \"--title\", title, \"--body-file\", tmpfile, \"--label\", \"capability-gap\"]"));
    assert!(body.contains("--body-file"));
    assert!(body.contains("--label capability-gap"));
}

#[test]
fn no_repo_flag_documented() {
    let body = discipline();
    assert!(body.contains("NO `--repo` flag"));
    assert!(body.contains("MUST NOT pass `--repo`"));
    assert!(body.contains("Destination comes from the local intake checkout origin"));
    assert!(!body.contains("gh issue create --repo"));
    assert!(!body.contains("gh issue list --repo"));
}

#[test]
fn no_merge_flags() {
    let skill = read(&skill_path().join("SKILL.md"));
    let body = discipline();
    assert!(skill
        .contains("Using merge flags, auto-merge flags, or `gh pr merge` anywhere in this skill."));
    assert!(body.contains("Running `gh pr merge`"));
    for flag in ["--auto", "--merge", "--squash", "--rebase", "--admin"] {
        assert!(body.contains(flag), "missing forbidden merge flag `{flag}`");
    }
}

#[test]
fn no_gh_auth_login_from_skill() {
    let skill = read(&skill_path().join("SKILL.md"));
    let body = discipline();
    assert!(skill.contains("`gh auth status`"));
    assert!(skill.contains("never run `gh auth login`"));
    assert!(body.contains("gh auth status"));
    assert!(body.contains("Never run `gh auth login` from this skill"));
}

#[test]
fn skill_dispatch_documents_file_gaps_mode() {
    let skill = read(&skill_path().join("SKILL.md"));
    assert!(skill.contains("--file-gaps <m2-output.json> --intake-dir <path>"));
    assert!(skill.contains("capability-gap-schema.md"));
    assert!(skill.contains("upstream-filing-discipline.md"));
    assert!(skill.contains("M3 SLO-Intake Filer"));
    assert!(skill.contains("Do not use `--repo`"));
}

#[test]
fn user_confirmation_required() {
    let skill = read(&skill_path().join("SKILL.md"));
    let body = discipline();
    assert!(skill.contains("per-issue user confirmation"));
    assert!(skill.contains("show the user the resolved intake origin URL, title, body preview, and validation result before each filing"));
    assert!(body.contains("Every candidate filing MUST surface a per-issue confirmation prompt"));
    assert!(body.contains("Never auto-file"));
}

#[test]
fn intake_repo_template_dependency_documented() {
    let body = discipline();
    assert!(body.contains("kerberosmansour/slo-security-intake"));
    assert!(body.contains(".github/ISSUE_TEMPLATE/capability-gap-record.md"));
    assert!(body.contains("test -f .github/ISSUE_TEMPLATE/capability-gap-record.md"));
}

#[test]
fn m1_m2_contracts_still_present() {
    let root = skill_path();
    let skill = read(&root.join("SKILL.md"));
    assert!(skill.contains("--read-declarations <path>"));
    assert!(skill.contains("--match <runbook.md> --catalog <catalog.json>"));
    assert!(root.join("scripts/read-declarations.py").is_file());
    assert!(root.join("references/methodology-m1-reader.md").is_file());
    assert!(root.join("references/methodology-m2-matcher.md").is_file());
}

#[test]
fn sec_libs_tool_deny_flags_unchanged() {
    let flags = toolflags::sec_libs_deny_flags();
    let combined = flags.join(",");
    assert!(combined.contains("WebFetch"));
    assert!(combined.contains("WebSearch"));
}

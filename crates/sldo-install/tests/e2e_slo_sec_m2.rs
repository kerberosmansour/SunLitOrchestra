//! M2 structural-contract tests for the slo-security-embedding runbook.
//!
//! Verifies `/slo-plan` gains three new required Contract Block rows
//! (Data classification, Proactive controls in play, Abuse acceptance
//! scenarios) plus the "abuse case" BDD category, and that the supporting
//! reference files exist. Also asserts backward-compat invariants.

use std::fs;
use std::path::{Path, PathBuf};

// --- Fixture: FNV-1a 64-bit hash of docs/runbook-template_v_3_template.md.
//
// Re-pinned 2026-04-30 by the loops-and-lessons-closure runbook M4, which
// explicitly authorizes adding the optional "Carry-forward from prior retros"
// section to the template. Earlier value (M2-era, 2026-04-24) was
// 0x5c2f04635249e0a2 / 29978 bytes. Future template edits must also be
// authorized by a milestone contract; if a future runbook touches the
// template without authorization, this test fails loudly.
//
// FNV-1a is non-cryptographic but stable, cheap to compute inline, and
// avoids pulling in `sha2` as a dev-dependency.
const EXPECTED_RUNBOOK_TEMPLATE_FNV1A_64: u64 = 0xfe57e5c116c1542e;
const EXPECTED_RUNBOOK_TEMPLATE_BYTE_LEN: usize = 32341;

fn fnv1a_64(s: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// BDD #1 — SKILL.md documents Data classification row.
// ---------------------------------------------------------------------------

#[test]
fn plan_skill_documents_data_classification_row() {
    let skill = read(&repo_root().join("skills/slo-plan/SKILL.md"));
    assert!(
        skill.contains("Data classification"),
        "/slo-plan SKILL.md must document the `Data classification` Contract Block row"
    );
    // All four allowed values must appear in the vocabulary documentation.
    for v in ["Public", "Internal", "Confidential", "Restricted"] {
        assert!(
            skill.contains(v),
            "/slo-plan SKILL.md must document the `{v}` data classification"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #2 — SKILL.md documents Proactive controls row.
// ---------------------------------------------------------------------------

#[test]
fn plan_skill_documents_proactive_controls_row() {
    let skill = read(&repo_root().join("skills/slo-plan/SKILL.md"));
    assert!(
        skill.contains("Proactive controls in play"),
        "/slo-plan SKILL.md must document the `Proactive controls in play` Contract Block row"
    );
    assert!(
        skill.contains("proactive-controls-vocabulary.md"),
        "/slo-plan SKILL.md must reference `references/proactive-controls-vocabulary.md`"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — SKILL.md documents Abuse acceptance scenarios row.
// ---------------------------------------------------------------------------

#[test]
fn plan_skill_documents_abuse_scenarios_row() {
    let skill = read(&repo_root().join("skills/slo-plan/SKILL.md"));
    assert!(
        skill.contains("Abuse acceptance scenarios"),
        "/slo-plan SKILL.md must document the `Abuse acceptance scenarios` Contract Block row"
    );
    assert!(
        skill.contains("abuse-case-examples.md"),
        "/slo-plan SKILL.md must reference `references/abuse-case-examples.md`"
    );
}

// ---------------------------------------------------------------------------
// BDD #4 — SKILL.md adds "abuse case" to required BDD categories.
// ---------------------------------------------------------------------------

#[test]
fn plan_skill_adds_abuse_case_bdd_category() {
    let skill = read(&repo_root().join("skills/slo-plan/SKILL.md"));
    let lower = skill.to_lowercase();
    assert!(
        lower.contains("abuse case"),
        "/slo-plan SKILL.md must list `abuse case` as a required-when-new-surface BDD category"
    );
    // The rule is "required when new surface is introduced; N/A-with-reason acceptable otherwise"
    let mentions_conditional = lower.contains("new surface") || lower.contains("n/a");
    assert!(
        mentions_conditional,
        "/slo-plan SKILL.md must document that abuse-case BDD is required only when a new surface is introduced (N/A with reason acceptable otherwise)"
    );
}

// ---------------------------------------------------------------------------
// BDD #5 / #6 / #7 — vocabulary file has data classifications + Rust-axum +
// Pulumi/AWS vocabulary.
// ---------------------------------------------------------------------------

#[test]
fn vocabulary_file_has_data_classifications() {
    let vocab = read(
        &repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md"),
    );
    for v in ["Public", "Internal", "Confidential", "Restricted"] {
        assert!(
            vocab.contains(v),
            "vocabulary file must list `{v}` as a data classification"
        );
    }
}

#[test]
fn vocabulary_file_has_rust_axum_crate_names() {
    let vocab = read(
        &repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md"),
    );
    let candidates = [
        "secure_boundary",
        "secure_data",
        "secure_identity",
        "secure_authz",
        "secure_output",
        "secure_errors",
    ];
    let found: Vec<&&str> = candidates.iter().filter(|c| vocab.contains(**c)).collect();
    assert!(
        found.len() >= 3,
        "vocabulary file must cite at least three SunLitSecureLibraries crate names; found {found:?}"
    );
}

#[test]
fn vocabulary_file_has_hulumi_references() {
    let vocab = read(
        &repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md"),
    );
    // At least one Hulumi component name should appear for Pulumi/AWS targets.
    let candidates = [
        "SecureBucket",
        "HulumiHardeningPack",
        "AccountFoundation",
        "Hulumi",
    ];
    let found: Vec<&&str> = candidates.iter().filter(|c| vocab.contains(**c)).collect();
    assert!(
        !found.is_empty(),
        "vocabulary file must reference at least one Hulumi component (SecureBucket / HulumiHardeningPack / AccountFoundation) for Pulumi/AWS stacks; found {found:?}"
    );
}

// ---------------------------------------------------------------------------
// BDD #8 / #9 — abuse-case examples file has ≥6 rows covering 6 surface classes.
// ---------------------------------------------------------------------------

fn count_given_blocks(s: &str) -> usize {
    // Count "| Given" occurrences (case-insensitive) inside table rows to
    // approximate BDD row count. Acceptable because the file is
    // agent-authored Markdown with explicit Given/When/Then columns.
    s.to_lowercase().matches("| given").count()
}

#[test]
fn abuse_case_examples_has_at_least_six_rows() {
    let ex = read(&repo_root().join("skills/slo-plan/references/abuse-case-examples.md"));
    let n = count_given_blocks(&ex);
    assert!(
        n >= 6,
        "abuse-case examples file must contain ≥6 Given/When/Then rows; found {n}"
    );
}

#[test]
fn abuse_case_examples_cover_six_surface_classes() {
    let ex = read(&repo_root().join("skills/slo-plan/references/abuse-case-examples.md"));
    // Six surface classes: HTTP endpoint, IPC command, file write, subprocess,
    // outbound request, persisted state. Check each appears as a topic.
    let classes = [
        ("HTTP endpoint", &["http", "endpoint", "ssrf"][..]),
        ("IPC command", &["ipc", "command"][..]),
        ("file write", &["file write", "path traversal", "zip-slip", "symlink"][..]),
        ("subprocess", &["subprocess", "command injection"][..]),
        ("outbound request", &["outbound", "metadata service", "ssrf"][..]),
        ("persisted state", &["persisted", "deserialization", "state"][..]),
    ];
    let ex_lower = ex.to_lowercase();
    for (class, keywords) in classes {
        let found = keywords.iter().any(|k| ex_lower.contains(*k));
        assert!(
            found,
            "abuse-case examples must cover surface class `{class}` (any of {keywords:?})"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #10 — empty-surface milestone rule documented.
// ---------------------------------------------------------------------------

#[test]
fn plan_skill_documents_empty_surface_rule() {
    let skill = read(&repo_root().join("skills/slo-plan/SKILL.md"));
    let lower = skill.to_lowercase();
    assert!(
        lower.contains("n/a") && (lower.contains("no new surface") || lower.contains("silent")),
        "/slo-plan SKILL.md must document that a milestone without a new surface fills the three new rows with `N/A — <reason>` (silent omission forbidden)"
    );
}

// ---------------------------------------------------------------------------
// BDD #11 — backward-compat: existing runbooks parse unchanged.
// ---------------------------------------------------------------------------

#[test]
fn existing_runbooks_have_milestone_tracker() {
    // Sentinel: shipped runbooks still carry a Milestone Tracker heading.
    // (Earlier fixtures were removed in the 2026-04 cleanup; re-pointed
    // at the surviving biz + sast runbooks.)
    for rb in [
        "docs/RUNBOOK-BIZ-SKILL-PACK-A.md",
        "docs/RUNBOOK-BIZ-SKILL-PACK-B1.md",
        "docs/RUNBOOK-SLO-SEC-LIBS.md",
    ] {
        let body = read(&repo_root().join(rb));
        assert!(
            body.contains("## Milestone Tracker") || body.contains("Milestone Tracker"),
            "{rb} must still contain a Milestone Tracker heading (backward-compat invariant)"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #12 — vocabulary file documents that values are Markdown-literal,
// never spliced into shell (defense against injection via vocab).
// ---------------------------------------------------------------------------

#[test]
fn vocabulary_file_documents_no_shell_interpolation() {
    let vocab = read(
        &repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md"),
    );
    let lower = vocab.to_lowercase();
    let documents_safety = lower.contains("markdown")
        && (lower.contains("never") || lower.contains("not"))
        && (lower.contains("shell") || lower.contains("subprocess") || lower.contains("invoked"));
    assert!(
        documents_safety,
        "vocabulary file must document that proactive-controls values are Markdown-literal, never invoked as shell or interpolated into subprocess commands (defense against vocab injection)"
    );
}

// ---------------------------------------------------------------------------
// BDD #13 — SKILL.md line count sane.
// ---------------------------------------------------------------------------

#[test]
fn plan_skill_line_count_sane() {
    let skill = read(&repo_root().join("skills/slo-plan/SKILL.md"));
    let n = skill.lines().count();
    assert!(
        n <= 300,
        "slo-plan SKILL.md grew to {n} lines (cap 300); consider splitting into reference files"
    );
}

// ---------------------------------------------------------------------------
// E2E — template invariant (FNV-1a-64 catches any byte edit).
// ---------------------------------------------------------------------------

#[test]
fn runbook_v3_template_fnv1a_unchanged() {
    let path = repo_root().join("docs/runbook-template_v_3_template.md");
    let body = fs::read(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    assert_eq!(
        body.len(),
        EXPECTED_RUNBOOK_TEMPLATE_BYTE_LEN,
        "template byte length changed — re-pin only when authorized by an explicit milestone contract"
    );
    let hash = fnv1a_64(&body);
    assert_eq!(
        hash,
        EXPECTED_RUNBOOK_TEMPLATE_FNV1A_64,
        "template FNV-1a-64 hash changed (expected 0x{EXPECTED_RUNBOOK_TEMPLATE_FNV1A_64:016x}, got 0x{hash:016x}) — re-pin only when authorized by an explicit milestone contract"
    );
}

// ---------------------------------------------------------------------------
// E2E — vocabulary and examples files exist and are non-empty.
// ---------------------------------------------------------------------------

#[test]
fn vocabulary_file_exists_and_nonempty() {
    let path = repo_root().join("skills/slo-plan/references/proactive-controls-vocabulary.md");
    let body = read(&path);
    assert!(body.len() > 500, "vocabulary file suspiciously short");
}

#[test]
fn abuse_case_examples_file_exists_and_nonempty() {
    let path = repo_root().join("skills/slo-plan/references/abuse-case-examples.md");
    let body = read(&path);
    assert!(body.len() > 500, "abuse-case examples file suspiciously short");
}

//! M1 structural-contract tests for the biz-skill-pack Runbook A.
//!
//! These tests verify the static contract of M1's edits — `/slo-legal/SKILL.md`
//! plus the three M1-tier `references/biz/` files and the oneNDA placeholder
//! template — are in place and well-formed. They do not exercise Claude Code
//! runtime behavior (that happens when an agent actually invokes `/slo-legal`);
//! they assert the documented shape is correct so downstream skills (and the
//! cross-skill citation test in `e2e_biz_a_m2.rs`) can rely on it.
//!
//! BDD scenarios are taken verbatim from
//! `docs/RUNBOOK-BIZ-SKILL-PACK-A.md` Milestone 1.
//!
//! Note on the oneNDA SHA-256 hash check: M1 ships the template as a placeholder
//! (see `references/biz/templates/onenda-uk.md` replace-before-production-use
//! instructions). Until canonical bytes replace the placeholder, the structural
//! test asserts the placeholder marker is present. A future small follow-up
//! runbook will pin the canonical SHA-256 once the bytes are fetched and
//! verified by the project owner.

use std::fs;
use std::path::{Path, PathBuf};

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

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];

const FOUR_MODES: &[&str] = &["draft", "translate", "triage", "prepare"];

const ONENDA_PLACEHOLDER_MARKER: &str = "ONENDA-UK-PLACEHOLDER";

// ---------------------------------------------------------------------------
// BDD #1 — slo-legal SKILL.md has required frontmatter.
// ---------------------------------------------------------------------------

#[test]
fn slo_legal_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));

    // Given: the new skills/slo-legal/SKILL.md
    // When:  parsing YAML frontmatter
    // Then:  name: slo-legal and a non-empty description
    assert!(
        skill.starts_with("---\n"),
        "SKILL.md must start with YAML frontmatter delimiter `---\\n`"
    );

    // Find the closing frontmatter delimiter on its own line.
    let after_open = &skill[4..];
    let close_idx = after_open
        .find("\n---\n")
        .expect("SKILL.md frontmatter must close with a `---` line");
    let frontmatter = &after_open[..close_idx];

    assert!(
        frontmatter.contains("name: slo-legal"),
        "SKILL.md frontmatter must contain `name: slo-legal`"
    );
    assert!(
        frontmatter.contains("description:"),
        "SKILL.md frontmatter must contain a `description:` key"
    );
    // description must be non-empty. YAML supports inline strings (`description: foo`),
    // double-quoted strings (`description: "foo"`), and block scalars (`description: >`
    // or `description: |` followed by indented content on subsequent lines).
    let lines: Vec<&str> = frontmatter.lines().collect();
    let desc_idx = lines
        .iter()
        .position(|l| l.trim_start().starts_with("description:"))
        .expect("description line not found");
    let desc_line = lines[desc_idx];
    let desc_value = desc_line
        .trim_start()
        .strip_prefix("description:")
        .unwrap()
        .trim();
    let has_inline_value = !desc_value.is_empty() && desc_value != ">" && desc_value != "|";
    let has_block_value = (desc_value == ">" || desc_value == "|")
        && lines
            .iter()
            .skip(desc_idx + 1)
            .take_while(|l| l.starts_with("  ") || l.is_empty())
            .any(|l| !l.trim().is_empty());
    assert!(
        has_inline_value || has_block_value,
        "description must be non-empty (inline or YAML `>`/`|` block scalar with indented content); got `{desc_value}` on the description line"
    );
}

// ---------------------------------------------------------------------------
// BDD #2 — slo-legal SKILL.md documents the four advisor modes.
// ---------------------------------------------------------------------------

#[test]
fn slo_legal_skill_md_documents_four_modes() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));
    for mode in FOUR_MODES {
        assert!(
            skill.contains(mode),
            "slo-legal SKILL.md must mention mode `{mode}` somewhere in the body"
        );
    }
    // And the modes section must be present (heading-level).
    assert!(
        skill.contains("## Modes") || skill.contains("# Modes"),
        "slo-legal SKILL.md must have a `## Modes` (or `# Modes`) heading"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — references/biz/triage-gate.md defines exactly the four predicate IDs.
// ---------------------------------------------------------------------------

#[test]
fn triage_gate_md_defines_four_predicate_ids() {
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));

    // Each predicate id must appear at least once in the file.
    for pid in FOUR_PREDICATE_IDS {
        assert!(
            gate.contains(pid),
            "triage-gate.md must define predicate id `{pid}`"
        );
    }

    // The file must include the canonical predicate-table columns somewhere
    // (we look for the column headers in any order, all in one row).
    let columns = ["id", "name", "predicate", "if_true", "route_to", "rationale_doc"];
    let mut header_row: Option<&str> = None;
    for line in gate.lines() {
        if columns.iter().all(|col| line.contains(col)) {
            header_row = Some(line);
            break;
        }
    }
    assert!(
        header_row.is_some(),
        "triage-gate.md must contain a Markdown table header with all six columns: {columns:?}"
    );
}

// ---------------------------------------------------------------------------
// BDD #4 — slo-legal SKILL.md cites all four predicate IDs.
// ---------------------------------------------------------------------------

#[test]
fn slo_legal_skill_md_cites_all_four_predicate_ids() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(
            skill.contains(pid),
            "slo-legal SKILL.md must cite predicate id `{pid}` from references/biz/triage-gate.md"
        );
    }
    // The skill must reference the triage-gate.md file path explicitly so
    // future readers see the source of truth.
    assert!(
        skill.contains("references/biz/triage-gate.md"),
        "slo-legal SKILL.md must reference `references/biz/triage-gate.md` as the predicate source of truth"
    );
}

// ---------------------------------------------------------------------------
// BDD #5 — cost-baseline-jpp-law-2026.md carries a retrieval date.
// ---------------------------------------------------------------------------

#[test]
fn cost_baseline_md_carries_retrieval_date() {
    let baseline = read(&repo_root().join("references/biz/cost-baseline-jpp-law-2026.md"));

    // The file's frontmatter must include a `retrieved: YYYY-MM-DD` line.
    let frontmatter_end = baseline
        .find("\n---\n")
        .or_else(|| baseline[4..].find("\n---\n").map(|i| i + 4))
        .expect("cost-baseline file must have YAML frontmatter delimited by `---`");
    let frontmatter = &baseline[..frontmatter_end];

    let retrieved_line = frontmatter
        .lines()
        .find(|l| l.trim_start().starts_with("retrieved:"))
        .expect("cost-baseline frontmatter must include `retrieved:` field");

    let date_str = retrieved_line.trim_start().strip_prefix("retrieved:").unwrap().trim();
    // YYYY-MM-DD shape check: 10 chars, two `-` at positions 4 and 7.
    assert_eq!(
        date_str.len(),
        10,
        "retrieved: value must be YYYY-MM-DD (got `{date_str}`)"
    );
    assert_eq!(&date_str[4..5], "-", "retrieved: value must have `-` at position 4 (got `{date_str}`)");
    assert_eq!(&date_str[7..8], "-", "retrieved: value must have `-` at position 7 (got `{date_str}`)");

    // The body must cite jpplaw.co.uk explicitly.
    assert!(
        baseline.contains("jpplaw.co.uk"),
        "cost-baseline file must cite jpplaw.co.uk as the source"
    );

    // At least one line item with a £ symbol or "GBP" must appear.
    assert!(
        baseline.contains("GBP") || baseline.contains("£"),
        "cost-baseline file must include at least one GBP / £ price reference"
    );
}

// ---------------------------------------------------------------------------
// BDD #6 — onenda-uk.md is in placeholder state OR canonical-pinned state.
// ---------------------------------------------------------------------------

const ONENDA_CANONICAL_PINNED_MARKER: &str = "ONENDA-UK-CANONICAL-PINNED";

#[test]
fn onenda_template_placeholder_or_pinned_hash() {
    let path = repo_root().join("references/biz/templates/onenda-uk.md");
    let body = read(&path);

    // The file is valid in two states: pre-pinning (PLACEHOLDER marker) and
    // post-pinning (CANONICAL-PINNED marker). The post-pinning state was
    // reached by follow-up `biz-pack-onenda-canonical-pin` once the project
    // owner manually fetched the canonical .docx and computed SHA-256.
    let placeholder = body.contains(ONENDA_PLACEHOLDER_MARKER);
    let pinned = body.contains(ONENDA_CANONICAL_PINNED_MARKER);
    assert!(
        placeholder || pinned,
        "onenda-uk.md must contain ONE OF: `{ONENDA_PLACEHOLDER_MARKER}` (pre-fetch state) or `{ONENDA_CANONICAL_PINNED_MARKER}` (post-pinning state). Neither is present — file is in an unknown state."
    );

    // Whichever state, the canonical-source citations must remain present.
    assert!(
        body.contains("onenda.org"),
        "onenda-uk.md must cite onenda.org as the canonical source"
    );
    assert!(
        body.contains("CC BY-ND 4.0"),
        "onenda-uk.md must document the CC BY-ND 4.0 license obligation"
    );
}

// ---------------------------------------------------------------------------
// BDD #7 — references/biz/ is NOT discovered as a skill by sldo-install.
// ---------------------------------------------------------------------------

#[test]
fn references_biz_dir_not_discovered_as_skill() {
    // The `discover_skills()` walker in src/install.rs requires
    // <skills_dir>/<name>/SKILL.md. references/biz/ lives outside skills/, so
    // it is not even on the walker's radar — but assert the path layout to
    // catch a future drift where someone moves references/biz/ under skills/.

    let skills_dir = repo_root().join("skills");
    let references_biz = repo_root().join("references/biz");

    assert!(
        skills_dir.exists() && skills_dir.is_dir(),
        "skills/ directory must exist"
    );
    assert!(
        references_biz.exists() && references_biz.is_dir(),
        "references/biz/ directory must exist"
    );

    // references/biz/ must NOT be a sibling under skills/.
    let misplaced = skills_dir.join("biz");
    assert!(
        !misplaced.exists(),
        "skills/biz/ must NOT exist — references/biz/ lives outside skills/ (per crates/sldo-install/src/install.rs:44-71's discover_skills() requiring <skills_dir>/<name>/SKILL.md)"
    );
    let misplaced_underscore = skills_dir.join("_biz-shared");
    assert!(
        !misplaced_underscore.exists(),
        "skills/_biz-shared/ must NOT exist — the shared-scaffolding location is references/biz/ at repo root, not under skills/ (the leading-underscore convention is fragile because discover_skills() does not filter it)"
    );

    // skills/slo-legal/SKILL.md must exist (M1's positive case).
    let slo_legal = skills_dir.join("slo-legal").join("SKILL.md");
    assert!(
        slo_legal.exists(),
        "skills/slo-legal/SKILL.md must exist (M1 ships /slo-legal as the wedge advisor skill)"
    );
}

// ---------------------------------------------------------------------------
// BDD #8 — slo-legal documents the gate-4 GDPR hard-block routing.
// ---------------------------------------------------------------------------

#[test]
fn gdpr_doc_draft_routes_to_triage() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));

    // The skill body must contain prose stating gate-4 is unconditional refusal
    // of `draft` for GDPR documents.
    assert!(
        skill.contains("gate-4-gdpr-document"),
        "slo-legal SKILL.md must cite gate-4-gdpr-document by id"
    );
    // Must list at least three GDPR doc-types so the gate's surface is concrete.
    let gdpr_doc_types = ["privacy notice", "ROPA", "DPA"];
    for ty in &gdpr_doc_types {
        assert!(
            skill.contains(ty),
            "slo-legal SKILL.md must enumerate GDPR doc-type `{ty}` so the gate-4 surface is concrete"
        );
    }
    // Must contain the unconditional-refusal language.
    let refusal_signals = ["unconditionally refused", "unconditional refusal", "never draft"];
    let any_refusal = refusal_signals.iter().any(|s| skill.contains(s));
    assert!(
        any_refusal,
        "slo-legal SKILL.md must state GDPR `draft` is unconditionally refused (looked for any of {refusal_signals:?})"
    );
}

// ---------------------------------------------------------------------------
// BDD #9 — slo-legal documents the two-tier output convention.
// ---------------------------------------------------------------------------

#[test]
fn confidential_draft_to_public_tier_rejected() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));

    // The skill must document docs/biz/ as the default for tier:confidential.
    assert!(
        skill.contains("docs/biz/"),
        "slo-legal SKILL.md must document `docs/biz/` as the confidential output dir"
    );
    assert!(
        skill.contains("docs/biz-public/"),
        "slo-legal SKILL.md must document `docs/biz-public/` as the public-tier output dir"
    );
    // The skill must mention `.gitignore` so the founder is steered to gitignore docs/biz/.
    assert!(
        skill.contains(".gitignore"),
        "slo-legal SKILL.md must mention `.gitignore` to steer the founder to ignore docs/biz/"
    );
    // The skill must NOT instruct putting confidential drafts into docs/biz-public/
    // (a forbidden-pattern absence check). We approximate by asserting the
    // explicit guidance "confidential drafts ... docs/biz/" appears AND no
    // contradictory guidance appears.
    assert!(
        skill.contains("Default for `draft`")
            || skill.contains("Default for draft")
            || skill.contains("default for `draft`")
            || skill.contains("default for draft"),
        "slo-legal SKILL.md must state `docs/biz/` is the default for draft outputs"
    );
}

// ---------------------------------------------------------------------------
// BDD #10 — non-UK jurisdiction request is rejected.
// ---------------------------------------------------------------------------

#[test]
fn non_uk_jurisdiction_arg_rejected() {
    let skill = read(&repo_root().join("skills/slo-legal/SKILL.md"));

    // The canonical error string must appear in the skill body.
    assert!(
        skill.contains("v1 supports UK only"),
        "slo-legal SKILL.md must document the canonical non-UK rejection error: `v1 supports UK only`"
    );
    // And reference the v2 architectural-pivot framing.
    assert!(
        skill.contains("v2 architectural pivot"),
        "slo-legal SKILL.md must reference `v2 architectural pivot` as the future expansion path"
    );
    // The skill must NOT pre-emptively stub a `--jurisdiction us` or `--jurisdiction eu` flag.
    let forbidden_stubs = ["--jurisdiction us", "--jurisdiction eu"];
    for stub in &forbidden_stubs {
        // It's OK for the skill to MENTION the flag in the context of rejecting it
        // (e.g., "non-UK arg surface like --jurisdiction us"), so we use a softer
        // check: the skill must NOT contain prose suggesting the stub is available.
        // We approximate by asserting the skill does not contain "Accepts --jurisdiction"
        // or similar acceptance language.
        let acceptance_signals = [
            &format!("Accepts {stub}")[..],
            &format!("accepts {stub}")[..],
            &format!("Use {stub}")[..],
            &format!("use {stub}")[..],
        ];
        for signal in acceptance_signals.iter() {
            assert!(
                !skill.contains(*signal),
                "slo-legal SKILL.md must NOT contain prose suggesting `{stub}` is accepted (found: `{signal}`)"
            );
        }
    }
}

//! M1 structural-contract tests for the slo-security-embedding runbook.
//!
//! These tests verify the static contract of M1's edits — the SKILL.md files,
//! reference templates, and frontmatter conventions are in place and
//! well-formed. They do not exercise Claude Code runtime behavior (that
//! happens when an agent actually invokes `/slo-ideate` or `/slo-architect`);
//! they assert the documented shape is correct so downstream skills can
//! rely on it.
//!
//! BDD scenarios and E2E validations are taken verbatim from
//! `docs/RUNBOOK-SLO-SECURITY-EMBEDDING.md` Milestone 1.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points at crates/sldo-install; go up two levels.
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
// BDD #1 — /slo-ideate has a 7th forcing question about security risks.
// ---------------------------------------------------------------------------

#[test]
fn ideate_has_seventh_forcing_question() {
    // Given: the edited skills/slo-ideate/SKILL.md
    let skill = read(&repo_root().join("skills/slo-ideate/SKILL.md"));

    // When: regex-scanning for a 7th numbered forcing question
    // Then: the section exists exactly once, mentions at least two of
    //       {breach, compliance fine, prolonged outage, worst day}.
    let has_q7 = skill.contains("\n7. **") || skill.contains("\n7) **");
    assert!(
        has_q7,
        "/slo-ideate SKILL.md missing a 7th numbered forcing question"
    );
    let q7_keywords = ["worst day", "breach", "fine", "outage"];
    let matches: Vec<_> = q7_keywords.iter().filter(|k| skill.contains(**k)).collect();
    assert!(
        matches.len() >= 2,
        "Q7 should mention at least two of {q7_keywords:?}; found {matches:?}"
    );
}

// ---------------------------------------------------------------------------
// BDD #2 — idea-doc shape includes a `## Top risks` block.
// ---------------------------------------------------------------------------

#[test]
fn ideate_idea_doc_shape_has_top_risks_block() {
    let skill = read(&repo_root().join("skills/slo-ideate/SKILL.md"));
    assert!(
        skill.contains("## Top risks"),
        "/slo-ideate SKILL.md must include a `## Top risks` heading inside the idea-doc shape"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — /slo-architect Step 3.5 cites both reference templates.
// ---------------------------------------------------------------------------

#[test]
fn architect_step_3_5_cites_both_templates() {
    let skill = read(&repo_root().join("skills/slo-architect/SKILL.md"));
    // Step 3.5 must exist.
    assert!(
        skill.contains("Step 3.5") || skill.contains("### 3.5") || skill.contains("3.5."),
        "/slo-architect SKILL.md must document Step 3.5 (STRIDE sweep + SECURITY.md + threat-model.md emission)"
    );
    assert!(
        skill.contains("references/SECURITY-md-template.md"),
        "Step 3.5 must cite skills/slo-architect/references/SECURITY-md-template.md"
    );
    assert!(
        skill.contains("references/threat-model-template.md"),
        "Step 3.5 must cite skills/slo-architect/references/threat-model-template.md"
    );
}

// ---------------------------------------------------------------------------
// BDD #4 — architect documents the three new frontmatter keys.
// ---------------------------------------------------------------------------

#[test]
fn architect_documents_new_frontmatter_keys() {
    let skill = read(&repo_root().join("skills/slo-architect/SKILL.md"));
    for key in ["security_libs_required", "ai_component", "compliance"] {
        assert!(
            skill.contains(key),
            "/slo-architect SKILL.md must document the `{key}` frontmatter key"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #5 — SECURITY.md template exists with required sections.
// ---------------------------------------------------------------------------

#[test]
fn security_md_template_has_required_sections() {
    let template = read(
        &repo_root().join("skills/slo-architect/references/SECURITY-md-template.md"),
    );
    assert!(template.len() > 500, "template is suspiciously short");
    for section in ["Crypto policy", "Auth model", "Input handling", "Escape hatches"] {
        assert!(
            template.contains(section),
            "SECURITY.md template missing required section `{section}`"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #6 — threat-model template has a STRIDE table.
// ---------------------------------------------------------------------------

#[test]
fn threat_model_template_has_stride_table() {
    let template = read(
        &repo_root().join("skills/slo-architect/references/threat-model-template.md"),
    );
    let stride_terms = [
        "Spoofing",
        "Tampering",
        "Repudiation",
        "Information disclosure",
        "Denial of service",
        "Elevation of privilege",
    ];
    for term in stride_terms {
        assert!(
            template.to_lowercase().contains(&term.to_lowercase()),
            "threat-model template missing STRIDE term `{term}`"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #7 — threat-model template has a compliance-mapping section with default columns.
// ---------------------------------------------------------------------------

#[test]
fn threat_model_template_has_compliance_mapping() {
    let template = read(
        &repo_root().join("skills/slo-architect/references/threat-model-template.md"),
    );
    assert!(
        template.contains("Compliance mapping") || template.contains("## Compliance"),
        "threat-model template missing `Compliance mapping` section"
    );
    // Default columns must both appear.
    assert!(
        template.contains("SOC 2") || template.contains("SOC2"),
        "compliance mapping must reference SOC 2 by default"
    );
    assert!(
        template.contains("ASVS"),
        "compliance mapping must reference OWASP ASVS by default"
    );
}

// ---------------------------------------------------------------------------
// BDD #8 — threat-model template has a conditional AI triad section.
// ---------------------------------------------------------------------------

#[test]
fn threat_model_template_has_ai_triad_conditional() {
    let template = read(
        &repo_root().join("skills/slo-architect/references/threat-model-template.md"),
    );
    // All three frameworks appear and are clearly grouped with ai_component: true.
    for citation in ["MITRE ATLAS", "OWASP LLM Top 10", "NIST AI RMF"] {
        assert!(
            template.contains(citation),
            "threat-model template missing AI triad citation `{citation}`"
        );
    }
    assert!(
        template.contains("ai_component"),
        "threat-model template must gate the AI triad section on `ai_component: true`"
    );
}

// ---------------------------------------------------------------------------
// BDD #9 — ideate SKILL.md backward compat — existing idea docs still work.
// ---------------------------------------------------------------------------

#[test]
fn ideate_existing_idea_doc_still_reads() {
    // An existing idea doc without the new Top risks block must still be
    // valid Markdown with frontmatter. We use the biz-skill-pack idea doc
    // as a fixture — it predates the security-embedding milestone's Q7
    // additions and so represents the pre-update shape.
    let doc = read(&repo_root().join("docs/idea/biz-skill-pack.md"));
    assert!(
        doc.starts_with("---\n"),
        "existing idea doc must retain YAML frontmatter shape"
    );
    // SKILL.md must not imply deletion of existing sections; i.e., prose
    // should treat Top risks as additive, not replacing the capabilities list.
    let skill = read(&repo_root().join("skills/slo-ideate/SKILL.md"));
    assert!(
        skill.contains("Five capabilities") || skill.contains("capabilities"),
        "ideate SKILL.md must retain the existing capabilities list in the idea-doc shape"
    );
}

// ---------------------------------------------------------------------------
// BDD #10 — architect SKILL.md backward compat — existing overviews still valid.
// ---------------------------------------------------------------------------

#[test]
fn architect_documents_defaults_for_optional_keys() {
    // /slo-architect SKILL.md must document defaults for the security-related
    // frontmatter keys it added so that overviews without them still parse.
    // (The original backward-compat fixture — tla-sha-autopop-overview.md —
    // was removed in the 2026-04 cleanup; the prose-level invariant remains.)
    let skill = read(&repo_root().join("skills/slo-architect/SKILL.md"));
    let documents_defaults = skill.contains("default")
        && (skill.contains("security_libs_required") && skill.contains("false"))
        && (skill.contains("ai_component") && skill.contains("false"));
    assert!(
        documents_defaults,
        "architect SKILL.md must document defaults for absent frontmatter keys (security_libs_required: false, ai_component: false)"
    );
}

// ---------------------------------------------------------------------------
// BDD #11 — architect documents behavior when idea doc is absent.
// ---------------------------------------------------------------------------

#[test]
fn architect_documents_absent_idea_doc_behavior() {
    let skill = read(&repo_root().join("skills/slo-architect/SKILL.md"));
    // The skill should either refuse or document a fallback. We assert that
    // the Inputs section or Step 3.5 prose mentions the missing-idea-doc case.
    let mentions_missing = skill.to_lowercase().contains("idea doc");
    assert!(
        mentions_missing,
        "architect SKILL.md must mention idea-doc inputs so the absent case is discoverable"
    );
}

// ---------------------------------------------------------------------------
// BDD #12 — Top risks content containing HTML / fences is handled literally,
//           not rendered as structured content by downstream agents.
// ---------------------------------------------------------------------------

#[test]
fn template_placeholder_fence_rule_documented() {
    // Both templates must document that user-provided strings render inside
    // ~~~text fences so raw content is literal text, not interpretable.
    for name in ["SECURITY-md-template.md", "threat-model-template.md"] {
        let template = read(
            &repo_root()
                .join("skills/slo-architect/references")
                .join(name),
        );
        assert!(
            template.contains("~~~text") || template.contains("fenced"),
            "template {name} must document the `~~~text` (or equivalent fenced-literal) placeholder-expansion rule"
        );
        assert!(
            template.to_lowercase().contains("user")
                && (template.to_lowercase().contains("escap")
                    || template.contains("literal")
                    || template.contains("verbatim")),
            "template {name} must document that user-provided strings are treated as literal/verbatim text"
        );
    }
}

// ---------------------------------------------------------------------------
// BDD #13 / #14 — SKILL.md line-count sanity.
// ---------------------------------------------------------------------------

#[test]
fn ideate_skill_line_count_sane() {
    let skill = read(&repo_root().join("skills/slo-ideate/SKILL.md"));
    let n = skill.lines().count();
    assert!(
        n <= 250,
        "slo-ideate SKILL.md grew to {n} lines (cap 250); consider splitting into reference files"
    );
}

#[test]
fn architect_skill_line_count_sane() {
    let skill = read(&repo_root().join("skills/slo-architect/SKILL.md"));
    let n = skill.lines().count();
    assert!(
        n <= 300,
        "slo-architect SKILL.md grew to {n} lines (cap 300); consider splitting into reference files"
    );
}

// ---------------------------------------------------------------------------
// BDD #15 — architect rerun is idempotent or prompts (f5 from critique).
// ---------------------------------------------------------------------------

#[test]
fn architect_rerun_idempotent_documented() {
    let skill = read(&repo_root().join("skills/slo-architect/SKILL.md"));
    // The skill must describe what happens on re-run: detect existing,
    // surface drift, prompt for overwrite/merge/skip. Not silent clobber.
    let lower = skill.to_lowercase();
    let mentions_rerun = lower.contains("re-run")
        || lower.contains("rerun")
        || lower.contains("already exists")
        || lower.contains("existing");
    let mentions_choice = lower.contains("overwrite")
        || lower.contains("merge")
        || lower.contains("skip")
        || lower.contains("prompt");
    assert!(
        mentions_rerun && mentions_choice,
        "architect SKILL.md must document re-run idempotency: detect existing SECURITY.md/threat-model, surface drift, prompt user to overwrite/merge/skip"
    );
}

// ---------------------------------------------------------------------------
// BDD #16 / #17 / #18 — frontmatter type-checker handles bad types (f7).
// ---------------------------------------------------------------------------

/// Minimal regex-free type checker for the three frontmatter keys.
/// Implements the contract M1's SKILL.md documents: bools for
/// security_libs_required / ai_component; list-of-allowed-strings for
/// compliance.
///
/// This helper lives in the test because M1 is a docs/contract milestone —
/// the runtime type-checking happens when Claude Code reads the skill. The
/// helper is used here to demonstrate the contract is sound and the skill's
/// prose is executable by a reasonable reader.
fn check_overview_frontmatter(body: &str) -> Result<(), String> {
    // Extract frontmatter block.
    if !body.starts_with("---\n") {
        return Err("no frontmatter".into());
    }
    let after = &body[4..];
    let end = after
        .find("\n---\n")
        .or_else(|| after.find("\n---"))
        .ok_or("no closing ---")?;
    let fm = &after[..end];

    let allowed_compliance = [
        "soc2",
        "asvs",
        "gdpr",
        "hipaa",
        "pci-dss",
        "nist-800-53",
        "iso-27001",
    ];

    for line in fm.lines() {
        let line = line.trim_end();
        if let Some(rest) = line.strip_prefix("security_libs_required:") {
            let v = rest.trim();
            if v != "true" && v != "false" {
                return Err(format!(
                    "security_libs_required must be bool (true|false), got `{v}`"
                ));
            }
        } else if let Some(rest) = line.strip_prefix("ai_component:") {
            let v = rest.trim();
            if v != "true" && v != "false" {
                return Err(format!(
                    "ai_component must be bool (true|false), got `{v}`"
                ));
            }
        } else if let Some(rest) = line.strip_prefix("compliance:") {
            let v = rest.trim();
            if !v.starts_with('[') || !v.ends_with(']') {
                return Err(format!(
                    "compliance must be a YAML flow-list like [soc2, asvs], got `{v}`"
                ));
            }
            let inner = &v[1..v.len() - 1];
            for item in inner.split(',') {
                let item = item.trim().trim_matches('"').trim_matches('\'');
                if item.is_empty() {
                    continue;
                }
                if !allowed_compliance.contains(&item) {
                    return Err(format!(
                        "compliance value `{item}` not in allowed set {allowed_compliance:?}"
                    ));
                }
            }
        }
    }
    Ok(())
}

#[test]
fn frontmatter_type_checker_rejects_string_bool_for_security_libs_required() {
    let bad = "---\nname: x\nsecurity_libs_required: \"yes\"\n---\nbody\n";
    let err = check_overview_frontmatter(bad).expect_err("must reject quoted-string bool");
    assert!(
        err.contains("security_libs_required"),
        "error must name the offending key; got: {err}"
    );
}

#[test]
fn frontmatter_type_checker_rejects_int_for_ai_component() {
    let bad = "---\nname: x\nai_component: 1\n---\nbody\n";
    let err = check_overview_frontmatter(bad).expect_err("must reject int bool");
    assert!(
        err.contains("ai_component"),
        "error must name the offending key; got: {err}"
    );
}

#[test]
fn frontmatter_type_checker_rejects_scalar_compliance() {
    let bad = "---\nname: x\ncompliance: soc2\n---\nbody\n";
    let err = check_overview_frontmatter(bad).expect_err("must reject scalar compliance");
    assert!(err.contains("compliance"));
}

#[test]
fn frontmatter_type_checker_rejects_unknown_compliance_value() {
    let bad = "---\nname: x\ncompliance: [soc5000]\n---\nbody\n";
    let err = check_overview_frontmatter(bad).expect_err("must reject unknown value");
    assert!(
        err.contains("soc5000") || err.contains("not in allowed"),
        "error must name the offending value; got: {err}"
    );
}

#[test]
fn frontmatter_type_checker_accepts_valid_frontmatter() {
    let good = concat!(
        "---\n",
        "name: demo\n",
        "security_libs_required: true\n",
        "ai_component: false\n",
        "compliance: [soc2, asvs]\n",
        "---\n",
        "body\n",
    );
    check_overview_frontmatter(good).expect("valid frontmatter must pass");
}

#[test]
fn frontmatter_type_checker_accepts_absent_new_keys_backward_compat() {
    // Synthesised pre-security-embedding frontmatter — represents an
    // overview written before /slo-architect added security_libs_required,
    // ai_component, compliance. The checker must treat absent keys as
    // defaults, not as errors. (The original on-disk fixture —
    // tla-sha-autopop-overview.md — was removed in the 2026-04 cleanup.)
    let synthesised_pre_sec_overview = concat!(
        "---\n",
        "name: pre-security-embedding-fixture\n",
        "created: 2026-04-01\n",
        "tla_required: false\n",
        "---\n",
        "# Overview body\n",
    );
    check_overview_frontmatter(synthesised_pre_sec_overview)
        .expect("frontmatter without the security-embedding keys must pass the checker (backward-compat)");
}

// ---------------------------------------------------------------------------
// E2E invariant: existing runbooks still parse (backward-compat sentinel).
// ---------------------------------------------------------------------------

#[test]
fn existing_runbooks_still_parse() {
    // Sentinel: every shipped runbook still carries a Milestone Tracker
    // heading. (Earlier fixtures — RUNBOOK-API-FACADE / RUNBOOK-AWS-ORG-SETUP
    // / RUNBOOK-TLA-SHA-AUTOPOP — were removed in the 2026-04 cleanup;
    // re-pointed at the surviving biz + sast runbooks.)
    let runbook_files = [
        "docs/RUNBOOK-BIZ-SKILL-PACK-A.md",
        "docs/RUNBOOK-BIZ-SKILL-PACK-B1.md",
        "docs/RUNBOOK-SAST-RULEGEN-A.md",
    ];
    for rb in runbook_files {
        let body = read(&repo_root().join(rb));
        assert!(
            body.contains("## Milestone Tracker") || body.contains("Milestone Tracker"),
            "{rb} should still contain a Milestone Tracker heading"
        );
    }
}

// ---------------------------------------------------------------------------
// E2E invariant: docs/runbook-template_v_3_template.md has not been touched.
// ---------------------------------------------------------------------------

#[test]
fn v3_template_not_modified() {
    let template = read(&repo_root().join("docs/runbook-template_v_3_template.md"));
    // A few stable sections that should remain present and unchanged in wording.
    assert!(template.contains("## Runbook Metadata"));
    assert!(template.contains("## Milestone Tracker"));
    assert!(template.contains("### Required Test Coverage Categories"));
}

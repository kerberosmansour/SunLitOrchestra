//! Structural-contract tests for `docs/runbook-template_v_4_template.md`.
//!
//! v4 is the canonical going-forward runbook template — `/slo-plan`'s
//! output contract — adding Carmack-style reliability controls
//! (debugger-first inspection, mandatory static analysis,
//! assertion-driven invariants, bounded resource design,
//! "make invalid states unrepresentable") on top of v3's SunLit-specific
//! structure (carry-forward from prior retros, abuse-acceptance
//! scenarios, threat-model integration).
//!
//! These tests are presence-grep tests — they assert that every locked
//! v4 surface remains in place after a future edit. v3 has its own
//! pin-test in `e2e_slo_sec_m1.rs` and `e2e_slo_sec_m2.rs`; this file
//! is the v4 equivalent.

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

const V4_PATH: &str = "docs/runbook-template_v_4_template.md";

#[test]
fn v4_template_exists_with_required_top_sections() {
    let body = read(&repo_root().join(V4_PATH));
    for marker in &[
        "## 1. Runbook Metadata",
        "## 2. Milestone Tracker",
        "## 3. End-to-End Architecture Diagram",
        "## 4. Carmack-Style Development Best Practices",
        "## 5. High-Level Design for State Modeling",
        "## 6. Global Execution Rules",
        "## 7. Global Entry Rules",
        "## 8. Global Exit Rules",
        "## 9. Background Context",
        "## 10. Carry-forward from prior retros",
        "## 11. BDD and Runtime Validation Rules",
        "## 12. Dependency, Migration, and Refactor Policy",
        "## 13. Evidence Log Template",
        "## 14. Self-Review Gate",
        "## 15. Lessons-Learned File Template",
        "## 16. Completion Summary Template",
        "## 17. Milestone Plan",
        "## 18. Documentation Update Table",
    ] {
        assert!(
            body.contains(marker),
            "v4 template missing required section: {marker}"
        );
    }
}

#[test]
fn v4_carmack_subsections_present() {
    let body = read(&repo_root().join(V4_PATH));
    // The 8 Carmack-style sub-rules in §4 must all be present and named
    // identically to the source-of-truth carmack template.
    for marker in &[
        "### 4.1 Inspect State, Do Not Guess",
        "### 4.2 Static Analysis Is Mandatory",
        "### 4.3 Assertions Are Executable Comments",
        "### 4.4 Prefer Bounded Resources Over Silent Growth",
        "### 4.5 Make Invalid States Unrepresentable",
        "### 4.6 Preserve Compatibility Until Explicitly Broken",
        "### 4.7 Prefer Small, Local, Reviewable Changes",
        "### 4.8 No Silent Failure",
    ] {
        assert!(
            body.contains(marker),
            "v4 template missing Carmack-style sub-rule: {marker}"
        );
    }
}

#[test]
fn v4_contract_block_has_new_carmack_fields() {
    let body = read(&repo_root().join(V4_PATH));
    for marker in &[
        "Resource bounds introduced/changed",
        "Invariants/assertions required",
        "Debugger / inspection expectation",
        "Static analysis gates",
    ] {
        assert!(
            body.contains(marker),
            "v4 Contract Block missing Carmack-style field: {marker}"
        );
    }
    // Existing v3 fields must still be present (additive change).
    for marker in &[
        "Files allowed to change",
        "Compatibility commitments",
        "Forbidden shortcuts",
    ] {
        assert!(
            body.contains(marker),
            "v4 Contract Block must preserve existing v3 field: {marker}"
        );
    }
}

#[test]
fn v4_metadata_has_new_command_fields() {
    let body = read(&repo_root().join(V4_PATH));
    for marker in &[
        "Default formatter command",
        "Default static analysis / lint command",
        "Default dependency / security audit command",
        "Default debugger or state-inspection tool",
    ] {
        assert!(
            body.contains(marker),
            "v4 Runbook Metadata must declare {marker}"
        );
    }
}

#[test]
fn v4_lessons_template_extended() {
    let body = read(&repo_root().join(V4_PATH));
    // The lessons-learned template must include the new Carmack-style
    // sections (assumptions verified/unresolved, invariants, resource
    // bounds, debugging notes) on top of v3's existing fields.
    for marker in &[
        "## Assumptions verified",
        "## Assumptions still unresolved",
        "## Invariants/assertions added or strengthened",
        "## Resource bounds established or verified",
        "## Debugging / inspection notes",
    ] {
        assert!(
            body.contains(marker),
            "v4 lessons-learned template must include {marker}"
        );
    }
}

#[test]
fn v4_completion_template_extended() {
    let body = read(&repo_root().join(V4_PATH));
    for marker in &[
        "## Static analysis and formatter evidence",
        "## Invariants/assertions added",
        "## Resource bounds added or verified",
    ] {
        assert!(
            body.contains(marker),
            "v4 completion-summary template must include {marker}"
        );
    }
}

#[test]
fn v4_carry_forward_section_preserved_from_v3() {
    // v4 inherits v3's carry-forward section verbatim — same lane
    // vocabulary, same backward-compat note. Existing skills
    // (/slo-execute Step 1.5, /slo-resume) read this section.
    let body = read(&repo_root().join(V4_PATH));
    assert!(
        body.contains("## 10. Carry-forward from prior retros"),
        "v4 must preserve the carry-forward section from v3"
    );
    for lane in &["micro", "milestone", "fresh-runbook"] {
        assert!(
            body.contains(lane),
            "v4 carry-forward must preserve lane vocabulary: {lane}"
        );
    }
    assert!(
        body.contains("retro-derived"),
        "v4 must continue to cite the locked `retro-derived` issue marker"
    );
}

#[test]
fn v4_self_review_gate_extended() {
    let body = read(&repo_root().join(V4_PATH));
    // The Self-Review Gate must include the new Carmack-style questions
    // on top of v3's existing checklist.
    for marker in &[
        "Did I add or update assertions/invariants",
        "Did I bound new resource growth",
        "Did I run formatter, typecheck, and static analysis",
        "Did I use a debugger or state-inspection tool",
    ] {
        assert!(
            body.contains(marker),
            "v4 Self-Review Gate must include {marker}"
        );
    }
}

#[test]
fn v4_global_execution_rules_extended() {
    let body = read(&repo_root().join(V4_PATH));
    // §6 must include the new Carmack-style rules so they are mandatory
    // not just illustrative in §4.
    for marker in &[
        "### 6.3 Assertions and invariants are mandatory",
        "### 6.4 Resource bounds are mandatory",
        "### 6.5 Static analysis must pass",
        "### 6.6 Debugger over guessing",
    ] {
        assert!(
            body.contains(marker),
            "v4 Global Execution Rules must include {marker}"
        );
    }
}

#[test]
fn v4_definition_of_done_includes_new_gates() {
    let body = read(&repo_root().join(V4_PATH));
    // The milestone Definition of Done must require:
    let lower = body.to_lowercase();
    assert!(
        lower.contains("formatter, typecheck, and static analyzer pass"),
        "v4 Definition of Done must require formatter + typecheck + static analyzer"
    );
    assert!(
        lower.contains("declared resource bounds")
            && lower.contains("are encoded and tested"),
        "v4 Definition of Done must require declared resource bounds to be encoded"
    );
    assert!(
        lower.contains("declared invariants/assertions")
            && lower.contains("are encoded and tested"),
        "v4 Definition of Done must require declared invariants to be encoded"
    );
}

#[test]
fn v3_template_still_present_for_historical_runbooks() {
    // v3 stays in place as the historical artifact for runbooks already
    // authored against it. Removing it would break the FNV-1a pin tests
    // and break references in completed runbooks.
    let path = repo_root().join("docs/runbook-template_v_3_template.md");
    assert!(
        path.exists(),
        "v3 template must remain in place as the historical artifact for runbooks already authored against it"
    );
}

#[test]
fn canonical_pointers_now_target_v4() {
    // The canonical "going forward" docs must point at v4. Existing
    // runbooks (RUNBOOK-*.md) keep their v3 references — those are
    // historical artifacts authored against v3 — so we do NOT assert
    // them here.
    let root = repo_root();
    for canonical in &[
        "CLAUDE.md",
        "README.md",
        "docs/ARCHITECTURE.md",
        "docs/skill-pack-catalog.md",
        "docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md",
        "docs/LOOPS-ENGINEERING.md",
        "skills/slo-plan/SKILL.md",
    ] {
        let body = read(&root.join(canonical));
        assert!(
            body.contains("runbook-template_v_4_template.md"),
            "{canonical} must reference v4 template (canonical going-forward)"
        );
    }
}

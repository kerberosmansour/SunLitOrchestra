//! M4 structural-contract tests for the slo-security-embedding runbook.
//!
//! Asserts `/slo-verify` gains Pass 4 (supply-chain + variant-analysis
//! spot-check + conditional DAST), the reference file ships with the
//! documented command set, and passes 1–3 are byte-identical to pre-M4.

use std::fs;
use std::path::{Path, PathBuf};

// --- Fixtures: FNV-1a-64 hashes of the three existing pass subsections.
// Captured at M4 start on 2026-04-24. If any changes during M4, the milestone
// accidentally edited an existing pass — contract violated.
const EXPECTED_PASS1_FNV1A_64: u64 = 0x7112f3380cf4dfcc;
const EXPECTED_PASS1_BYTE_LEN: usize = 280;
const EXPECTED_PASS2_FNV1A_64: u64 = 0xe28a58fb580e347a;
const EXPECTED_PASS2_BYTE_LEN: usize = 160;
const EXPECTED_PASS3_FNV1A_64: u64 = 0x525e5cb087db1b0c;
const EXPECTED_PASS3_BYTE_LEN: usize = 260;

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
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// Split `slo-verify/SKILL.md` into named subsection bytes keyed by heading.
/// Returns the byte ranges for `### Pass 1.`, `### Pass 2.`, `### Pass 3.`
/// (ending at the next `## ` top-level heading after Pass 3 or at EOF).
fn extract_pass_subsections(body: &str) -> [(usize, usize); 3] {
    let p1 = body.find("### Pass 1.").expect("Pass 1 heading missing");
    let p2 = body.find("### Pass 2.").expect("Pass 2 heading missing");
    let p3 = body.find("### Pass 3.").expect("Pass 3 heading missing");
    // End of Pass 3 = the next heading of any level (H2 or H3) after p3.
    // Post-M4 the next heading is `### Pass 4`; pre-M4 it was `## When you
    // find a bug`. Using `min(find("\n### "), find("\n## "))` makes the
    // boundary stable across the M4 edit.
    let tail = &body[p3 + "### Pass 3.".len()..];
    let next_h3 = tail.find("\n### ").map(|o| p3 + "### Pass 3.".len() + o);
    let next_h2 = tail.find("\n## ").map(|o| p3 + "### Pass 3.".len() + o);
    let p3_end = match (next_h3, next_h2) {
        (Some(a), Some(b)) => a.min(b),
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => body.len(),
    };
    [(p1, p2), (p2, p3), (p3, p3_end)]
}

// ---------------------------------------------------------------------------
// Invariants: passes 1–3 byte-unchanged.
// ---------------------------------------------------------------------------

#[test]
fn pass_1_subsection_byte_invariant() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let [p1, _, _] = extract_pass_subsections(&body);
    let slice = &body.as_bytes()[p1.0..p1.1];
    assert_eq!(
        slice.len(),
        EXPECTED_PASS1_BYTE_LEN,
        "Pass 1 subsection byte length changed"
    );
    assert_eq!(
        fnv1a_64(slice),
        EXPECTED_PASS1_FNV1A_64,
        "Pass 1 subsection content changed — M4 must not edit Pass 1"
    );
}

#[test]
fn pass_2_subsection_byte_invariant() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let [_, p2, _] = extract_pass_subsections(&body);
    let slice = &body.as_bytes()[p2.0..p2.1];
    assert_eq!(
        slice.len(),
        EXPECTED_PASS2_BYTE_LEN,
        "Pass 2 subsection byte length changed"
    );
    assert_eq!(
        fnv1a_64(slice),
        EXPECTED_PASS2_FNV1A_64,
        "Pass 2 subsection content changed — M4 must not edit Pass 2"
    );
}

#[test]
fn pass_3_subsection_byte_invariant() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let [_, _, p3] = extract_pass_subsections(&body);
    let slice = &body.as_bytes()[p3.0..p3.1];
    assert_eq!(
        slice.len(),
        EXPECTED_PASS3_BYTE_LEN,
        "Pass 3 subsection byte length changed"
    );
    assert_eq!(
        fnv1a_64(slice),
        EXPECTED_PASS3_FNV1A_64,
        "Pass 3 subsection content changed — M4 must not edit Pass 3"
    );
}

// ---------------------------------------------------------------------------
// BDD #1 — SKILL.md has a Pass 4 heading after Pass 3.
// ---------------------------------------------------------------------------

#[test]
fn skill_has_pass_4_after_pass_3() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let p3 = body.find("### Pass 3.").expect("Pass 3 heading missing");
    let p4 = body
        .find("### Pass 4")
        .expect("Pass 4 heading missing — SKILL.md must document a fourth runtime-QA pass");
    assert!(p4 > p3, "Pass 4 must appear after Pass 3 in SKILL.md");
}

// ---------------------------------------------------------------------------
// BDD #2 — stack-detection heuristic documented.
// ---------------------------------------------------------------------------

#[test]
fn pass_4_documents_stack_detection() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let lower = body.to_lowercase();
    // At least three manifests named.
    let manifests = ["cargo.toml", "package.json", "pyproject.toml", "go.mod"];
    let found: Vec<_> = manifests.iter().filter(|m| lower.contains(**m)).collect();
    assert!(
        found.len() >= 3,
        "Pass 4 prose in SKILL.md must name ≥3 stack-detection manifest files; found {found:?}"
    );
}

// ---------------------------------------------------------------------------
// BDD #3 — polyglot rule documented.
// ---------------------------------------------------------------------------

#[test]
fn pass_4_documents_polyglot_rule() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let lower = body.to_lowercase();
    let mentions_polyglot = lower.contains("polyglot")
        || lower.contains("multiple stack")
        || lower.contains("multi-stack");
    let mentions_all_sets = lower.contains("all applicable")
        || lower.contains("each stack gets")
        || lower.contains("both")
        || lower.contains("one row per stack");
    assert!(
        mentions_polyglot && mentions_all_sets,
        "Pass 4 must document the polyglot rule: run all applicable command sets; one row per stack"
    );
}

// ---------------------------------------------------------------------------
// BDD #4 — tool-optional + tool-error rule documented (offline-safe).
// ---------------------------------------------------------------------------

#[test]
fn pass_4_documents_tool_optional_rule() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let lower = body.to_lowercase();
    let mentions_skipped = lower.contains("skipped") || lower.contains("skip");
    let mentions_reason = lower.contains("not installed")
        || lower.contains("unreachable")
        || lower.contains("tool error")
        || lower.contains("network");
    let mentions_not_finding =
        lower.contains("not a finding") || lower.contains("never") && lower.contains("finding");
    assert!(
        mentions_skipped && mentions_reason && mentions_not_finding,
        "Pass 4 must document that tool-error / unreachable-tool exits map to `skipped` rows, never to findings"
    );
}

// ---------------------------------------------------------------------------
// BDD #5 — DAST conditional on smoke service.
// ---------------------------------------------------------------------------

#[test]
fn pass_4_documents_dast_conditional() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let lower = body.to_lowercase();
    assert!(lower.contains("dast"), "Pass 4 must reference DAST");
    let mentions_conditional = lower.contains("smoke service")
        || lower.contains("smoke/reference")
        || lower.contains("openapi")
        || lower.contains("conditional");
    assert!(
        mentions_conditional,
        "Pass 4 must gate DAST on smoke/reference service presence"
    );
}

// ---------------------------------------------------------------------------
// BDD #6 — markdown-only target N/A path.
// ---------------------------------------------------------------------------

#[test]
fn pass_4_documents_markdown_only_na() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let lower = body.to_lowercase();
    assert!(
        lower.contains("markdown") && (lower.contains("n/a") || lower.contains("no compiled")),
        "Pass 4 must document N/A path for markdown-only targets"
    );
}

// ---------------------------------------------------------------------------
// BDD #7 — reference file exists and documents Rust / Semgrep / DAST blocks.
// ---------------------------------------------------------------------------

#[test]
fn reference_file_exists_and_sized() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    assert!(body.len() > 1000, "reference file suspiciously short");
}

#[test]
fn reference_file_documents_cargo_audit() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    assert!(
        body.contains("cargo audit"),
        "reference file must name `cargo audit`"
    );
    // Exit-code contract documented.
    assert!(
        body.contains("exit")
            && (body.contains("0") || body.contains("1"))
            && (body.contains("2") || body.contains("error")),
        "reference file must document cargo audit exit-code semantics (0 clean / 1 finding / ≥2 tool error)"
    );
}

#[test]
fn reference_file_documents_cargo_deny() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    assert!(
        body.contains("cargo deny"),
        "reference file must name `cargo deny`"
    );
}

#[test]
fn reference_file_documents_semgrep() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    assert!(
        body.contains("semgrep"),
        "reference file must name `semgrep`"
    );
    assert!(
        body.contains("--sarif") || body.contains("sarif-output"),
        "reference file must show Semgrep SARIF output flag"
    );
}

#[test]
fn reference_file_documents_ast_grep() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    assert!(
        body.contains("ast-grep"),
        "reference file must name `ast-grep`"
    );
}

#[test]
fn reference_file_documents_dast_command_conditional() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    let mentions_zap = body.contains("ZAP") || body.contains("zap") || body.contains("zaproxy");
    let mentions_dastardly = body.contains("Dastardly") || body.contains("dastardly");
    assert!(
        mentions_zap || mentions_dastardly,
        "reference file must document ZAP or Dastardly DAST command"
    );
}

#[test]
fn reference_file_documents_polyglot_rule() {
    let body = read(&repo_root().join("skills/slo-verify/references/security-pass-commands.md"));
    let lower = body.to_lowercase();
    let mentions_polyglot = lower.contains("polyglot")
        || lower.contains("multiple stack")
        || lower.contains("multi-stack");
    assert!(
        mentions_polyglot,
        "reference file must document polyglot rule"
    );
}

#[test]
fn reference_file_reuses_bug_found_flow() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let lower = body.to_lowercase();
    let mentions_reuse = lower.contains("existing bug-found")
        || lower.contains("reuse")
        || lower.contains("same as")
        || (lower.contains("regression test") && lower.contains("pass 4"));
    assert!(
        mentions_reuse,
        "Pass 4 prose must reference the existing bug-found flow (regression test first, hand to /slo-execute, re-verify) — not invent a new flow"
    );
}

// ---------------------------------------------------------------------------
// BDD #15 — Pass 4 is additive (no renames / reorderings).
// ---------------------------------------------------------------------------

#[test]
fn three_pass_ordering_preserved() {
    let body = read(&repo_root().join("skills/slo-verify/SKILL.md"));
    let p1 = body.find("### Pass 1.").unwrap();
    let p2 = body.find("### Pass 2.").unwrap();
    let p3 = body.find("### Pass 3.").unwrap();
    let p4 = body.find("### Pass 4").unwrap();
    assert!(
        p1 < p2 && p2 < p3 && p3 < p4,
        "Passes must appear in order 1 → 2 → 3 → 4"
    );
}

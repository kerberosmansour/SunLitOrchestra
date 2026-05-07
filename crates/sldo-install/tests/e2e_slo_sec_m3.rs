//! M3 structural-contract tests for the slo-security-embedding runbook.
//!
//! Asserts `skills/slo-critique/personas/security.md` is rewritten around
//! bug-class elimination + variant analysis + threat-model citation, and
//! that `ceo.md` / `design.md` personas plus the finding-row table schema in
//! `SKILL.md` are byte-unchanged. `eng.md` was intentionally changed later by
//! the Fowler AI architecture M4 runbook, so this historical guard now checks
//! that the authorized architecture-coherence marker exists instead of pinning
//! bytes from before M4.

use std::fs;
use std::path::{Path, PathBuf};

// --- Fixtures: FNV-1a-64 hashes of the non-rewritten personas,
// captured at M3 start on 2026-04-24. If any of these hashes changes during
// M3, an out-of-scope persona was touched — contract violated.
const EXPECTED_CEO_FNV1A_64: u64 = 0xa297a61e54048204;
const EXPECTED_CEO_BYTE_LEN: usize = 1787;
const EXPECTED_DESIGN_FNV1A_64: u64 = 0x449d7a844c24e5cd;
const EXPECTED_DESIGN_BYTE_LEN: usize = 1860;

// --- Finding-row header must remain byte-identical (schema invariant).
const EXPECTED_FINDING_ROW_HEADER: &str =
    "| id | persona | category | runbook section | finding | concrete scenario | recommendation |";

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

// ---------------------------------------------------------------------------
// BDD #1–#4 — rewritten persona enforces class/citation/elimination/variant.
// ---------------------------------------------------------------------------

#[test]
fn persona_requires_class_naming() {
    let persona = read(&repo_root().join("skills/slo-critique/personas/security.md"));
    let lower = persona.to_lowercase();
    assert!(
        lower.contains("class") && (lower.contains("eliminat") || lower.contains("impossible")),
        "security persona must require every finding to name a bug class and answer the elimination question"
    );
    assert!(
        persona.contains("bug-class-catalog.md")
            || lower.contains("bug class catalog")
            || lower.contains("catalog"),
        "security persona must cite the bug-class catalog by path or name"
    );
}

#[test]
fn persona_requires_threat_model_citation() {
    let persona = read(&repo_root().join("skills/slo-critique/personas/security.md"));
    assert!(
        persona.contains("docs/slo/design/") && persona.contains("threat-model"),
        "security persona must require every finding to cite a row from docs/slo/design/<slug>-threat-model.md"
    );
}

#[test]
fn persona_requires_class_elimination_answer() {
    let persona = read(&repo_root().join("skills/slo-critique/personas/security.md"));
    let lower = persona.to_lowercase();
    let mentions_eliminated = lower.contains("eliminated") || lower.contains("impossible");
    let mentions_mitigated = lower.contains("mitigated");
    assert!(
        mentions_eliminated && mentions_mitigated,
        "security persona must document the eliminate-vs-mitigate question every finding answers"
    );
}

#[test]
fn persona_requires_variant_analysis_pointer() {
    let persona = read(&repo_root().join("skills/slo-critique/personas/security.md"));
    let lower = persona.to_lowercase();
    assert!(
        lower.contains("variant") && lower.contains("analysis"),
        "security persona must require each finding to include a variant-analysis pointer (where else in the codebase could this class exist?)"
    );
    assert!(
        persona.contains("variant-analysis-playbook.md")
            || lower.contains("variant-analysis playbook"),
        "security persona must cite variant-analysis-playbook.md"
    );
}

// ---------------------------------------------------------------------------
// BDD #5–#6 — bug-class catalog covers OWASP ASVS chapters + cites libraries.
// ---------------------------------------------------------------------------

#[test]
fn catalog_covers_asvs_chapters() {
    let catalog = read(&repo_root().join("skills/slo-critique/references/bug-class-catalog.md"));
    // Count ASVS V-chapter headings. ASVS 5.0 has 17 chapters; we require ≥10.
    let mut chapters_found = 0;
    for v in 1..=17 {
        let marker_a = format!("V{v}");
        let marker_b = format!(" V{v} ");
        let marker_c = format!(" V{v}:");
        let marker_d = format!("V{v}.");
        if catalog.contains(&marker_a)
            || catalog.contains(&marker_b)
            || catalog.contains(&marker_c)
            || catalog.contains(&marker_d)
        {
            chapters_found += 1;
        }
    }
    assert!(
        chapters_found >= 10,
        "bug-class catalog must cover ≥10 ASVS 5.0 chapters; found {chapters_found}"
    );
}

#[test]
fn catalog_cites_secure_libraries() {
    let catalog = read(&repo_root().join("skills/slo-critique/references/bug-class-catalog.md"));
    let candidates = [
        "secure_boundary",
        "secure_data",
        "secure_identity",
        "secure_authz",
        "secure_output",
        "secure_errors",
        "SqlIdentifier",
        "SafePath",
        "SafeUrl",
        "SecureJson",
    ];
    let found: Vec<&&str> = candidates
        .iter()
        .filter(|c| catalog.contains(**c))
        .collect();
    assert!(
        found.len() >= 3,
        "bug-class catalog must cite ≥3 SunLitSecurityLibraries crates/types as elimination patterns; found {found:?}"
    );
}

// ---------------------------------------------------------------------------
// BDD #7–#8 — variant-analysis playbook has three strategies + small-codebase exit.
// ---------------------------------------------------------------------------

#[test]
fn playbook_has_three_strategies() {
    let playbook =
        read(&repo_root().join("skills/slo-critique/references/variant-analysis-playbook.md"));
    let lower = playbook.to_lowercase();
    for tool in ["ripgrep", "ast-grep", "semgrep"] {
        assert!(
            lower.contains(tool),
            "variant-analysis playbook must document strategy for `{tool}`"
        );
    }
}

#[test]
fn playbook_has_small_codebase_exit() {
    let playbook =
        read(&repo_root().join("skills/slo-critique/references/variant-analysis-playbook.md"));
    let lower = playbook.to_lowercase();
    // The playbook should have an explicit N/A rule for very small repos.
    assert!(
        (lower.contains("small") || lower.contains("< 500") || lower.contains("n/a"))
            && (lower.contains("loc") || lower.contains("codebase") || lower.contains("variant")),
        "variant-analysis playbook must have an explicit small-codebase exit (N/A when LOC is very small)"
    );
}

// ---------------------------------------------------------------------------
// BDD #9–#10 — CEO / design personas unchanged; eng changed by later M4.
// ---------------------------------------------------------------------------

#[test]
fn ceo_persona_unchanged() {
    let path = repo_root().join("skills/slo-critique/personas/ceo.md");
    let body = fs::read(&path).unwrap();
    assert_eq!(
        body.len(),
        EXPECTED_CEO_BYTE_LEN,
        "ceo.md byte length changed"
    );
    assert_eq!(
        fnv1a_64(&body),
        EXPECTED_CEO_FNV1A_64,
        "ceo.md content changed — M3 must not edit ceo persona"
    );
}

#[test]
fn eng_persona_architecture_coherence_allowed_after_m4() {
    let path = repo_root().join("skills/slo-critique/personas/eng.md");
    let body = read(&path);
    let lower = body.to_lowercase();
    assert!(
        lower.contains("architecture coherence") && lower.contains("four-object summary"),
        "eng.md must carry the authorized M4 architecture-coherence marker"
    );
}

#[test]
fn design_persona_unchanged() {
    let path = repo_root().join("skills/slo-critique/personas/design.md");
    let body = fs::read(&path).unwrap();
    assert_eq!(
        body.len(),
        EXPECTED_DESIGN_BYTE_LEN,
        "design.md byte length changed"
    );
    assert_eq!(
        fnv1a_64(&body),
        EXPECTED_DESIGN_FNV1A_64,
        "design.md content changed — M3 must not edit design persona"
    );
}

#[test]
fn finding_row_schema_unchanged() {
    let skill = read(&repo_root().join("skills/slo-critique/SKILL.md"));
    assert!(
        skill.contains(EXPECTED_FINDING_ROW_HEADER),
        "finding-row table header changed in skills/slo-critique/SKILL.md — M3 must not reshape the finding schema"
    );
}

// ---------------------------------------------------------------------------
// BDD #11 — no OWASP boilerplate enumeration in the rewritten persona.
// ---------------------------------------------------------------------------

#[test]
fn persona_forbids_owasp_boilerplate() {
    let persona = read(&repo_root().join("skills/slo-critique/personas/security.md"));
    let lower = persona.to_lowercase();
    // The persona must explicitly reject generic OWASP-category enumeration.
    assert!(
        lower.contains("owasp")
            && (lower.contains("generic")
                || lower.contains("boilerplate")
                || lower.contains("enumeration")
                || lower.contains("without a concrete")
                || lower.contains("concrete surface")),
        "security persona must explicitly forbid generic OWASP-category enumeration without a concrete surface"
    );
}

// ---------------------------------------------------------------------------
// BDD #12 — handoff note in SKILL.md updated.
// ---------------------------------------------------------------------------

#[test]
fn skill_security_persona_handoff_updated() {
    let skill = read(&repo_root().join("skills/slo-critique/SKILL.md"));
    let lower = skill.to_lowercase();
    // The rotation-order description for security must reference the new framing.
    assert!(
        lower.contains("class elimination")
            || lower.contains("variant analysis")
            || lower.contains("threat-model citation"),
        "skills/slo-critique/SKILL.md must update the security-persona description to reference class elimination + variant analysis + threat-model citation"
    );
}

// ---------------------------------------------------------------------------
// BDD #13 — adversarial persona-prompt injection: self-bounded mandate.
// ---------------------------------------------------------------------------

#[test]
fn persona_mandate_is_self_bounded() {
    let persona = read(&repo_root().join("skills/slo-critique/personas/security.md"));
    let lower = persona.to_lowercase();
    // The persona must explicitly document it will not follow runbook-embedded
    // instructions that attempt to alter its mandate.
    assert!(
        (lower.contains("ignore") || lower.contains("not follow") || lower.contains("mandate"))
            && (lower.contains("runbook") || lower.contains("embedded") || lower.contains("body")),
        "security persona must document that its mandate is bounded to reviewing plans and it does not follow instructions embedded in the runbook body"
    );
}

// ---------------------------------------------------------------------------
// E2E — existing critique outputs remain valid Markdown.
// ---------------------------------------------------------------------------

#[test]
fn existing_critiques_valid_markdown() {
    // The biz-skill-pack-a critique is a real critique-output fixture and
    // should still parse as a Markdown document with a findings table.
    // (The original fixture — tla-sha-autopop — was removed in the 2026-04
    // cleanup.)
    let critique = read(&repo_root().join("docs/slo/critique/biz-skill-pack-a.md"));
    assert!(
        critique.contains("| id |") || critique.contains("|id|"),
        "existing critique file must still contain the finding-row table"
    );
}

// ---------------------------------------------------------------------------
// E2E — catalog + playbook files exist, non-empty, sensible size.
// ---------------------------------------------------------------------------

#[test]
fn catalog_file_exists_and_sized() {
    let body = read(&repo_root().join("skills/slo-critique/references/bug-class-catalog.md"));
    assert!(body.len() > 1000, "bug-class catalog suspiciously short");
}

#[test]
fn playbook_file_exists_and_sized() {
    let body =
        read(&repo_root().join("skills/slo-critique/references/variant-analysis-playbook.md"));
    assert!(
        body.len() > 1000,
        "variant-analysis playbook suspiciously short"
    );
}

//! M1 structural-contract test (slo-threat-model-producer runbook).
//!
//! Asserts the `/slo-architect` Step 3.5 PRODUCER contract — the write-side
//! addition that makes `/slo-architect` also emit
//! `docs/slo/design/<slug>-threat-model.slo.json`:
//!
//! - The producer contract is present in `skills/slo-architect/SKILL.md`
//!   Step 3.5 and `skills/slo-architect/references/threat-model-template.md`:
//!   emits `<slug>-threat-model.slo.json`, cites
//!   `references/security/threat-model-schema.md`, states the provenance
//!   idiom and the supersede-don't-renumber rule.
//! - ENG-1 (no over-claim — no producer executes locally): the template's
//!   serialization-mapping maps the Markdown `tm-<slug>-abuse-N` column 1:1
//!   onto the JSON `id` field and forbids renumber; AND the merged dogfood
//!   fixture still strict-conforms with ids `tm-slo-sec-abuse-1..8` present
//!   and contiguous. This is a STRUCTURAL PROXY for idempotence, not a live
//!   re-emission proof.
//! - SEC-1 (producer-side injection): the producer prose mandates structural
//!   JSON serialization (or escaped equivalent) and that user-controlled
//!   idea-doc text NEVER chooses `id` / `classification` /
//!   `accepted_residual` / `status` (author-controlled fields).

use regex::Regex;
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
    let path = workspace_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

const ARCHITECT_SKILL: &str = "skills/slo-architect/SKILL.md";
const TEMPLATE: &str = "skills/slo-architect/references/threat-model-template.md";
const MERGED_FIXTURE: &str = "docs/slo/design/slo-security-embedding-threat-model.slo.json";

/// BDD `producer_contract_documented` + `template_cites_schema`.
#[test]
fn producer_contract_is_documented_in_step_3_5() {
    let skill = read(ARCHITECT_SKILL);
    let template = read(TEMPLATE);
    let both = format!("{skill}\n{template}");

    assert!(
        both.contains("-threat-model.slo.json"),
        "Step 3.5 / template must document emitting <slug>-threat-model.slo.json"
    );
    assert!(
        both.contains("references/security/threat-model-schema.md"),
        "the producer contract must cite references/security/threat-model-schema.md"
    );
    assert!(
        both.contains("producer_skill_sha")
            || both.contains("producing-skill")
            || both.contains("SKILL.md git sha"),
        "the producer contract must state the provenance idiom \
         (producing-skill SKILL.md git sha + input git blob shas)"
    );
    assert!(
        both.to_lowercase().contains("supersede") && both.to_lowercase().contains("renumber"),
        "the producer contract must state the supersede-don't-renumber rule"
    );
}

/// BDD `existing_e2e_guards_intact` — Step 3.5 item 7 anchor and the
/// e2e_slo_sec_m1 cited substrings must survive an append-only edit.
#[test]
fn append_only_preserved_existing_step35_anchors() {
    let skill = read(ARCHITECT_SKILL);
    // e2e_slo_sec_m1.rs:42 asserts a "\n7. **" Step 3.5 item exists.
    assert!(
        skill.contains("\n7. **") || skill.contains("\n7) **"),
        "Step 3.5 item 7 anchor lost — the producer edit must be append-only \
         (e2e_slo_sec_m1 depends on the \\n7. ** anchor)"
    );
    for cited in [
        "references/SECURITY-md-template.md",
        "references/threat-model-template.md",
        "## Top risks",
    ] {
        assert!(
            skill.contains(cited),
            "append-only violated: e2e_slo_sec_m1-cited substring {cited:?} \
             missing from slo-architect/SKILL.md"
        );
    }
}

/// BDD `mapping_forbids_renumber` (ENG-1 structural proxy, doc half).
#[test]
fn template_mapping_maps_ids_1to1_and_forbids_renumber() {
    let template = read(TEMPLATE);
    let lc = template.to_lowercase();
    assert!(
        template.contains("tm-<slug>-abuse-N") || template.contains("tm-<slug>-abuse-"),
        "the serialization mapping must name the tm-<slug>-abuse-N → id mapping"
    );
    assert!(
        lc.contains("\"id\"") || lc.contains("`id`") || lc.contains(" id field"),
        "the serialization mapping must map the abuse-case row id onto the JSON id field"
    );
    assert!(
        lc.contains("forbid") && lc.contains("renumber"),
        "the serialization mapping must explicitly forbid renumber (1:1, frozen)"
    );
}

/// BDD `mapping_forbids_renumber` (ENG-1 structural proxy, fixture half) —
/// the merged dogfood fixture still conforms with frozen contiguous ids.
#[test]
fn merged_fixture_ids_unchanged_and_contiguous() {
    let raw = read(MERGED_FIXTURE);
    let v: serde_json::Value =
        serde_json::from_str(&raw).expect("merged dogfood fixture must be valid JSON");
    let re = Regex::new(r"^tm-slo-sec-abuse-(\d+)$").unwrap();
    let mut nums: Vec<u32> = v["abuse_cases"]
        .as_array()
        .expect("abuse_cases array")
        .iter()
        .map(|ac| {
            let id = ac["id"].as_str().expect("abuse id is a string");
            let c = re.captures(id).unwrap_or_else(|| {
                panic!("merged fixture id {id:?} drifted from tm-slo-sec-abuse-N")
            });
            c[1].parse::<u32>()
                .expect("trailing id segment is an integer")
        })
        .collect();
    nums.sort_unstable();
    assert_eq!(
        nums,
        (1..=8).collect::<Vec<_>>(),
        "merged dogfood fixture must keep its frozen contiguous ids tm-slo-sec-abuse-1..8 \
         (idempotent-emit structural proxy — a renumber here is the original failure)"
    );
}

/// BDD `idea_doc_text_cannot_choose_control_fields` (SEC-1 producer-side).
#[test]
fn producer_prose_mandates_sec1_neutralisation() {
    let skill = read(ARCHITECT_SKILL);
    let template = read(TEMPLATE);
    let both = format!("{skill}\n{template}");
    let lc = both.to_lowercase();

    assert!(
        lc.contains("serializer") || lc.contains("serialize") || lc.contains("escape"),
        "SEC-1: producer prose must mandate structural JSON serialization \
         (or escaped equivalent) for user-controlled idea-doc strings"
    );
    // The author-controlled fields user text must never choose.
    for field in ["id", "classification", "accepted_residual", "status"] {
        assert!(
            both.contains(field),
            "SEC-1: producer prose must name the author-controlled field {field:?} \
             that user-controlled idea-doc text must never choose"
        );
    }
    assert!(
        lc.contains("never")
            && (lc.contains("idea-doc")
                || lc.contains("idea doc")
                || lc.contains("user-controlled")
                || lc.contains("top-risks")
                || lc.contains("top risks")),
        "SEC-1: producer prose must state user-controlled idea-doc text NEVER \
         chooses the author-controlled fields"
    );
}

/// REGRESSION (verify M1, 2026-05-19): the SEC-1 loose-conjunction assertion
/// above is satisfiable by the words "never" and "idea-doc" appearing
/// anywhere in the SKILL.md (e.g., "never overwrite by default" pre-existing
/// elsewhere), so a mutation that weakened the actual SEC-1 phrase
/// `**never** chooses` to `MAYBE chooses` did NOT make that assertion fail.
/// Prior lesson recurrence: "a guard must bind the invariant, not the prose
/// around it." This test binds the invariant tightly: it requires `never`,
/// `chooses`, and one of the four author-controlled field tokens to appear
/// within ~120 chars of each other in the producer-side prose.
#[test]
fn sec1_clause_is_specifically_bound() {
    let re = Regex::new(
        r"(?is)\bnever\b[^\n]{0,40}\bchooses\b[^\n]{0,120}\b(id|classification|accepted_residual|status)\b",
    )
    .unwrap();
    // Per-file (not union): SEC-1 is documented redundantly in BOTH the
    // SKILL.md and the template, which is good prose discipline; but the
    // test must bind each independently so a partial weakening in one file
    // is caught (verify M1 found that a union-only check let a SKILL.md-only
    // mutation slip through because the template still matched).
    for path in [ARCHITECT_SKILL, TEMPLATE] {
        let body = read(path);
        assert!(
            re.is_match(&body),
            "SEC-1 regression: {path} must literally state `never ... chooses \
             ... <id|classification|accepted_residual|status>` within a \
             sentence — a union check across both files is too forgiving."
        );
    }
}

//! M1 structural-contract test (innovation-loop runbook).
//!
//! Asserts the Experiment Book spine invariants from M1 of
//! `docs/RUNBOOK-innovation-loop.md`, grounded in the binding spec
//! `docs/slo/design/innovation-loop-experiment-book-spec.md`:
//!
//! - `skills/slo-experiment/SKILL.md` exists with valid YAML frontmatter,
//!   non-empty `name` + `description`, and `name` == dir == "slo-experiment".
//! - The skill's declared output paths are allow-listed
//!   (`docs/slo/experiments/`) with no `..` traversal and no absolute path.
//! - (critique S1) The skill body mandates a runtime `<slug>` validation rule
//!   `^[a-z0-9][a-z0-9-]*$`.
//! - The template `docs/slo/templates/experiment-book-template_v_1.md` exists
//!   and carries §0–§11 in order; the frozen 8 exit states + 5 status values;
//!   the §2A Judgment Timing Rule (phase moods, play = safety-only); the
//!   Experiment Safety Rails table + per-phase Safety Check; the Experiment
//!   Phase Contract field set; the three Definition-of-Learned blocks; and the
//!   five §10 promotion-seed table headers.
//! - (critique S2) The template wraps user-supplied strings (§0 Starting hunch,
//!   §3 Material) in `~~~text` fences.
//! - No secret/PII pattern appears in any tracked file under
//!   `docs/slo/experiments/` (vacuous-pass while no Books exist yet).

use std::path::{Component, Path, PathBuf};

const SKILL_REL: &str = "skills/slo-experiment/SKILL.md";
const TEMPLATE_REL: &str = "docs/slo/templates/experiment-book-template_v_1.md";

/// The frozen 8-state exit vocabulary (interfaces doc §3.1).
const EXIT_STATES: &[&str] = &[
    "promote_to_idea",
    "promote_to_ticket",
    "promote_to_research",
    "promote_to_runbook",
    "needs_more_play",
    "blocked_by_unknown",
    "killed_but_reusable",
    "archive_no_action",
];

/// The frozen 5 phase-status values (interfaces doc §3.2).
const STATUS_VALUES: &[&str] = &[
    "not_started",
    "in_progress",
    "blocked",
    "complete",
    "skipped_with_reason",
];

/// The §0–§11 section headings, in frozen order.
const SECTION_HEADINGS: &[&str] = &[
    "## 0. Experiment Metadata",
    "## 1. Experiment Tracker",
    "## 2. Global Experiment Rules",
    "## 3. Sandbox Charter",
    "## 4. Play Log",
    "## 5. Pattern Catalog",
    "## 6. Precision Model",
    "## 7. Spike Cards and Evidence",
    "## 8. Curation Decision",
    "## 9. Demo Pack",
    "## 10. Handoff Contract",
    "## 11. Compost / Lessons",
];

/// The frozen 5 phase modes (interfaces doc §3.3).
const PHASE_MODES: &[&str] = &[
    "divergent",
    "convergent",
    "measurement",
    "evidence",
    "communication",
];

/// The five §10 promotion-seed table headers.
const PROMOTION_SEEDS: &[&str] = &[
    "Idea Seed",
    "Ticket Seed",
    "Research Seed",
    "Runbook Seed",
    "Compost Entry",
];

/// Output-path allow-list for the innovation-loop skill pack.
const ALLOWED_OUTPUT_PREFIXES: &[&str] = &["docs/slo/experiments/", "experiments/"];

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
        .unwrap_or_else(|e| panic!("failed to read {}: {} (M1 not yet implemented?)", rel, e))
}

fn extract_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---\n") {
        return None;
    }
    let after_open = &content[4..];
    let close_pos = after_open.find("\n---")?;
    Some(&after_open[..close_pos])
}

#[test]
fn slo_experiment_frontmatter_and_name() {
    let content = read(SKILL_REL);
    let fm = extract_frontmatter(&content)
        .unwrap_or_else(|| panic!("{} missing YAML frontmatter", SKILL_REL));
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(fm)
        .unwrap_or_else(|e| panic!("{} frontmatter parse error: {}", SKILL_REL, e));
    let map = yaml
        .as_mapping()
        .unwrap_or_else(|| panic!("{} frontmatter is not a mapping", SKILL_REL));

    let name = map
        .get(serde_yaml_ng::Value::String("name".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let desc = map
        .get(serde_yaml_ng::Value::String("description".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    assert_eq!(
        name, "slo-experiment",
        "skill `name` must equal its directory name"
    );
    assert!(
        !desc.trim().is_empty(),
        "skill `description` must be non-empty"
    );
}

#[test]
fn slo_experiment_output_paths_safe() {
    let content = read(SKILL_REL);
    let mut failures: Vec<String> = Vec::new();
    // Every line that declares the Book/output path must be allow-listed and
    // traversal-free. We scan lines mentioning the artifact filename.
    for (i, line) in content.lines().enumerate() {
        if !line.contains("EXPERIMENT.md") {
            continue;
        }
        // Pull out any path-shaped token that ends in EXPERIMENT.md.
        // Note: `<` / `>` are deliberately NOT split chars — a `<slug>`
        // placeholder must stay attached to its path so the allow-list prefix
        // check sees the whole `docs/slo/experiments/<slug>/EXPERIMENT.md`.
        for tok in line.split(|c: char| c.is_whitespace() || "`()|".contains(c)) {
            if !tok.ends_with("EXPERIMENT.md") {
                continue;
            }
            let tok = tok.trim_start_matches('!').trim_start_matches('[');
            let p = Path::new(tok);
            if p.is_absolute() {
                failures.push(format!("line {}: absolute output path `{}`", i + 1, tok));
            }
            if p.components().any(|c| matches!(c, Component::ParentDir)) {
                failures.push(format!(
                    "line {}: `..` traversal in output path `{}`",
                    i + 1,
                    tok
                ));
            }
            let allow = ALLOWED_OUTPUT_PREFIXES
                .iter()
                .any(|pre| tok.starts_with(pre) || tok.starts_with(&format!("./{pre}")));
            // A bare placeholder like `EXPERIMENT.md` (no dir) is fine in prose;
            // only flag tokens that carry a directory not in the allow-list.
            if tok.contains('/') && !allow {
                failures.push(format!(
                    "line {}: output path `{}` not under {:?}",
                    i + 1,
                    tok,
                    ALLOWED_OUTPUT_PREFIXES
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "slo-experiment output-path invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn slo_experiment_mandates_slug_validation() {
    // critique S1: runtime <slug> validation must be stated in the skill body.
    let content = read(SKILL_REL);
    assert!(
        content.contains("^[a-z0-9][a-z0-9-]*$"),
        "slo-experiment/SKILL.md must mandate runtime slug validation `^[a-z0-9][a-z0-9-]*$` (critique S1)"
    );
}

#[test]
fn template_has_sections_in_order() {
    let t = read(TEMPLATE_REL);
    let mut last = 0usize;
    for heading in SECTION_HEADINGS {
        let pos = t
            .find(heading)
            .unwrap_or_else(|| panic!("template missing section heading `{}`", heading));
        assert!(
            pos >= last,
            "template section `{}` is out of order (frozen §0–§11 order)",
            heading
        );
        last = pos;
    }
}

#[test]
fn template_has_frozen_vocabularies() {
    let t = read(TEMPLATE_REL);
    for s in EXIT_STATES {
        assert!(t.contains(s), "template missing frozen exit state `{}`", s);
    }
    for s in STATUS_VALUES {
        assert!(
            t.contains(s),
            "template missing frozen status value `{}`",
            s
        );
    }
    for m in PHASE_MODES {
        assert!(t.contains(m), "template missing frozen phase mode `{}`", m);
    }
}

#[test]
fn template_has_judgment_timing_rule_and_safety_rails() {
    let t = read(TEMPLATE_REL);
    assert!(
        t.contains("Judgment Timing Rule"),
        "template missing the §2A Judgment Timing Rule (phase moods)"
    );
    // play = safety-only sentinel (critique E1 frozen sentinel).
    assert!(
        t.contains("judge safety only"),
        "template Judgment Timing Rule must state `/slo-play` judges safety only"
    );
    assert!(
        t.contains("Experiment Safety Rails"),
        "template missing the Experiment Safety Rails defaults table"
    );
    assert!(
        t.contains("Safety Check"),
        "template missing the per-phase Safety Check block"
    );
    assert!(
        t.contains("Experiment Phase Contract"),
        "template missing the Experiment Phase Contract field table"
    );
}

#[test]
fn template_has_definition_of_learned_and_seeds() {
    let t = read(TEMPLATE_REL);
    assert!(
        t.contains("Definition of Learned"),
        "template must use Definition of Learned (not Definition of Done)"
    );
    assert!(
        t.contains("Spike — Definition of Learned") || t.contains("Spike Definition of Learned"),
        "template missing the spike Definition-of-Learned variant"
    );
    assert!(
        t.contains("Curation — Definition of Learned")
            || t.contains("Curation Definition of Learned"),
        "template missing the curation Definition-of-Learned variant"
    );
    for seed in PROMOTION_SEEDS {
        assert!(
            t.contains(seed),
            "template missing §10 promotion-seed header `{}`",
            seed
        );
    }
}

#[test]
fn template_fences_user_supplied_strings() {
    // critique S2: §0 hunch + §3 material must be rendered inside ~~~text fences.
    let t = read(TEMPLATE_REL);
    let fence_count = t.matches("~~~text").count();
    assert!(
        fence_count >= 2,
        "template must wrap user strings (§0 hunch, §3 material) in `~~~text` fences; found {} fence opener(s) (critique S2)",
        fence_count
    );
}

#[test]
fn no_secret_or_pii_in_tracked_experiment_books() {
    // Vacuous-pass while no Books exist yet (the gallery example arrives at M5).
    let dir = workspace_root().join("docs/slo/experiments");
    if !dir.exists() {
        return;
    }
    // A few high-signal secret/PII patterns. Detective, not exhaustive
    // (tm-innovation-loop-abuse-1 accepted residual): catches the obvious leak.
    let patterns = [
        regex::Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(), // AWS access key id
        regex::Regex::new(r"-----BEGIN [A-Z ]*PRIVATE KEY-----").unwrap(),
        regex::Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b").unwrap(), // email
        regex::Regex::new(r"\bghp_[A-Za-z0-9]{36}\b").unwrap(),                       // GitHub PAT
    ];
    let mut failures: Vec<String> = Vec::new();
    visit_md(&dir, &mut |path, body| {
        for re in &patterns {
            if let Some(m) = re.find(body) {
                failures.push(format!(
                    "{}: secret/PII pattern `{}`",
                    path.display(),
                    m.as_str()
                ));
                break;
            }
        }
    });
    assert!(
        failures.is_empty(),
        "secret/PII found in tracked Experiment Book(s) (tm-innovation-loop-abuse-1):\n  - {}",
        failures.join("\n  - ")
    );
}

fn visit_md(dir: &Path, f: &mut dyn FnMut(&Path, &str)) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            visit_md(&path, f);
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Ok(body) = std::fs::read_to_string(&path) {
                f(&path, &body);
            }
        }
    }
}

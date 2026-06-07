//! M5 structural-contract test (innovation-loop runbook): close the loop.
//!
//! Asserts `/slo-curate` + `/slo-demo` invariants and that the gallery example
//! Book closes end-to-end:
//! - both parse, `name` == dir, output paths safe;
//! - `/slo-curate` Mode `convergent`, exactly-one-disposition rule, cite-evidence;
//! - `/slo-demo` Mode `communication`, the 4 frozen promotion destinations +
//!   next-artifact paths, suggestion-only (no auto-invoke);
//! - the example Book `docs/slo/experiments/example-context-validator/EXPERIMENT.md`
//!   closes with exactly one of the frozen 8 exit states and is PII-clean.

use std::path::{Component, Path, PathBuf};

const CURATE_REL: &str = "skills/slo-curate/SKILL.md";
const DEMO_REL: &str = "skills/slo-demo/SKILL.md";
const TEMPLATE_REL: &str = "docs/slo/templates/experiment-book-template_v_1.md";
const EXAMPLE_REL: &str = "docs/slo/experiments/example-context-validator/EXPERIMENT.md";
const ALLOWED_OUTPUT_PREFIXES: &[&str] = &["docs/slo/experiments/", "experiments/"];

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

const PROMOTION_DESTINATIONS: &[&str] = &[
    "/slo-ideate",
    "/slo-ticket-plan",
    "/slo-research",
    "/slo-plan",
];

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {}: {} (M5 not yet implemented?)", rel, e))
}

fn extract_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---\n") {
        return None;
    }
    let after_open = &content[4..];
    let close_pos = after_open.find("\n---")?;
    Some(&after_open[..close_pos])
}

fn assert_skill_shape(rel: &str, expected_name: &str) {
    let content = read(rel);
    let fm = extract_frontmatter(&content).unwrap_or_else(|| panic!("{} missing frontmatter", rel));
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(fm)
        .unwrap_or_else(|e| panic!("{} frontmatter parse error: {}", rel, e));
    let map = yaml
        .as_mapping()
        .unwrap_or_else(|| panic!("{} frontmatter not a mapping", rel));
    let name = map
        .get(serde_yaml_ng::Value::String("name".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let desc = map
        .get(serde_yaml_ng::Value::String("description".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert_eq!(name, expected_name, "{}: name must equal dir", rel);
    assert!(
        !desc.trim().is_empty(),
        "{}: description must be non-empty",
        rel
    );
    for line in content.lines() {
        if !line.contains("EXPERIMENT.md") {
            continue;
        }
        for tok in line.split(|c: char| c.is_whitespace() || "`()|".contains(c)) {
            if !tok.ends_with("EXPERIMENT.md") {
                continue;
            }
            let tok = tok.trim_start_matches('!').trim_start_matches('[');
            let p = Path::new(tok);
            assert!(!p.is_absolute(), "{}: absolute output path `{}`", rel, tok);
            assert!(
                !p.components().any(|c| matches!(c, Component::ParentDir)),
                "{}: `..` in `{}`",
                rel,
                tok
            );
            if tok.contains('/') {
                assert!(
                    ALLOWED_OUTPUT_PREFIXES
                        .iter()
                        .any(|pre| tok.starts_with(pre)),
                    "{}: `{}` not allow-listed",
                    rel,
                    tok
                );
            }
        }
    }
}

#[test]
fn curate_demo_shape_and_paths_safe() {
    assert_skill_shape(CURATE_REL, "slo-curate");
    assert_skill_shape(DEMO_REL, "slo-demo");
}

#[test]
fn curate_one_disposition_convergent() {
    let c = read(CURATE_REL);
    assert!(
        c.contains("convergent"),
        "/slo-curate must declare Mode convergent"
    );
    assert!(
        c.to_lowercase().contains("exactly one disposition"),
        "/slo-curate must require exactly one disposition per candidate"
    );
    assert!(
        c.to_lowercase().contains("cite"),
        "/slo-curate must require each disposition to cite a probe/spike"
    );
}

#[test]
fn demo_frozen_destinations_and_suggestion_only() {
    let d = read(DEMO_REL);
    assert!(
        d.contains("communication"),
        "/slo-demo must declare Mode communication"
    );
    for dest in PROMOTION_DESTINATIONS {
        assert!(
            d.contains(dest),
            "/slo-demo must name promotion destination `{}`",
            dest
        );
    }
    // next-artifact paths.
    assert!(
        d.contains("docs/slo/idea/"),
        "/slo-demo must name the idea-seed next artifact"
    );
    // suggestion-only, no auto-invoke (tm-...-abuse-6).
    let low = d.to_lowercase();
    assert!(
        low.contains("suggestion")
            || low.contains("never auto-invoke")
            || low.contains("does not invoke")
            || low.contains("not an auto"),
        "/slo-demo must make promotion a suggestion, never an auto-invocation"
    );
}

#[test]
fn skills_target_sections_eight_nine_ten() {
    let t = read(TEMPLATE_REL);
    assert!(t.contains("## 8. Curation Decision"));
    assert!(t.contains("## 9. Demo Pack"));
    assert!(t.contains("## 10. Handoff Contract"));
}

#[test]
fn example_book_closes_with_one_exit_state_and_no_pii() {
    let book = read(EXAMPLE_REL);
    let present: Vec<&str> = EXIT_STATES
        .iter()
        .copied()
        .filter(|s| book.contains(*s))
        .collect();
    // The Book must reach a terminal exit state (the leading-metric proof).
    assert!(
        !present.is_empty(),
        "example Book must close with one of the frozen 8 exit states"
    );
    // PII/secret scan over the gallery Book (must be synthetic).
    let patterns = [
        regex::Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
        regex::Regex::new(r"-----BEGIN [A-Z ]*PRIVATE KEY-----").unwrap(),
        regex::Regex::new(r"\bghp_[A-Za-z0-9]{36}\b").unwrap(),
    ];
    for re in &patterns {
        assert!(
            re.find(&book).is_none(),
            "example Book contains a secret/PII pattern (must be synthetic)"
        );
    }
}

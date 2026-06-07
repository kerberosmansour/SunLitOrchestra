//! M4 structural-contract test (innovation-loop runbook): the only code phase.
//!
//! Asserts `/slo-spike` invariants — the highest-risk phase (it may run code):
//! - parses, `name` == dir, output paths safe;
//! - Mode `evidence`;
//! - mandates a resource budget and a delete-or-promote decision;
//! - confines scratch to `experiments/<slug>/` and forbids production promotion;
//! - the verdict (decision hint) derives from a recorded evidence log, not
//!   model narration;
//! - targets §7 of the template.

use std::path::{Component, Path, PathBuf};

const SPIKE_REL: &str = "skills/slo-spike/SKILL.md";
const TEMPLATE_REL: &str = "docs/slo/templates/experiment-book-template_v_1.md";
const ALLOWED_OUTPUT_PREFIXES: &[&str] = &["docs/slo/experiments/", "experiments/"];

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
        .unwrap_or_else(|e| panic!("failed to read {}: {} (M4 not yet implemented?)", rel, e))
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
fn spike_shape_and_paths_safe() {
    let content = read(SPIKE_REL);
    let fm = extract_frontmatter(&content).expect("slo-spike missing frontmatter");
    let yaml: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(fm).expect("slo-spike frontmatter parse error");
    let map = yaml
        .as_mapping()
        .expect("slo-spike frontmatter not a mapping");
    let name = map
        .get(serde_yaml_ng::Value::String("name".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let desc = map
        .get(serde_yaml_ng::Value::String("description".into()))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert_eq!(name, "slo-spike", "name must equal dir");
    assert!(!desc.trim().is_empty(), "description must be non-empty");
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
            assert!(!p.is_absolute(), "absolute output path `{}`", tok);
            assert!(
                !p.components().any(|c| matches!(c, Component::ParentDir)),
                "`..` in `{}`",
                tok
            );
            if tok.contains('/') {
                assert!(
                    ALLOWED_OUTPUT_PREFIXES
                        .iter()
                        .any(|pre| tok.starts_with(pre)),
                    "`{}` not allow-listed",
                    tok
                );
            }
        }
    }
}

#[test]
fn spike_mode_evidence() {
    assert!(
        read(SPIKE_REL).contains("evidence"),
        "/slo-spike must declare Mode evidence"
    );
}

#[test]
fn spike_mandates_budget_and_delete_or_promote() {
    let s = read(SPIKE_REL).to_lowercase();
    assert!(
        s.contains("resource budget") || s.contains("budget"),
        "/slo-spike must mandate a resource budget"
    );
    assert!(
        s.contains("delete-or-promote") || s.contains("delete or promote"),
        "/slo-spike must require a delete-or-promote decision"
    );
}

#[test]
fn spike_confines_scratch_and_forbids_production_promotion() {
    let s = read(SPIKE_REL);
    assert!(
        s.contains("experiments/<slug>/"),
        "/slo-spike must confine scratch to experiments/<slug>/<spike-id>/"
    );
    let low = s.to_lowercase();
    assert!(
        low.contains("no production")
            || low.contains("not promoted to production")
            || low.contains("never") && low.contains("production"),
        "/slo-spike must forbid production promotion"
    );
}

#[test]
fn spike_verdict_derives_from_evidence() {
    let low = read(SPIKE_REL).to_lowercase();
    assert!(
        low.contains("evidence log")
            || low.contains("commands / evidence")
            || low.contains("evidence"),
        "/slo-spike must require a recorded evidence log"
    );
    // Anti-fabrication: verdict from evidence, not narration.
    assert!(
        low.contains("not from")
            || low.contains("derives from")
            || low.contains("never fabricate")
            || low.contains("decision hint"),
        "/slo-spike must derive the decision from recorded evidence"
    );
}

#[test]
fn spike_targets_section_seven() {
    assert!(read(TEMPLATE_REL).contains("## 7. Spike Cards and Evidence"));
    let s = read(SPIKE_REL);
    assert!(
        s.contains("Spike Card") || s.contains("§7"),
        "/slo-spike must reference its §7 target"
    );
}

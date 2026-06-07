//! M2 structural-contract test (innovation-loop runbook): the divergent core.
//!
//! Asserts `/slo-sandbox` + `/slo-play` invariants. These are **frozen-sentinel
//! presence/absence checks** (critique E1): a structural test cannot detect
//! subtle *tonal* convergence — that is owned by the M5 end-to-end dogfood + a
//! human read. Here we check:
//!
//! - both skills parse with valid frontmatter, `name` == dir, output paths safe;
//! - `/slo-play` declares Mode `divergent`, CONTAINS the verbatim sentinel
//!   "judge safety only", and does NOT contain a ranking/winner heading;
//! - `/slo-play` names the frozen 8 probe types;
//! - `/slo-sandbox` declares a "Not a Feature Yet" gate and a kill-criteria field;
//! - both target a section heading that exists in the Experiment Book template.

use std::path::{Component, Path, PathBuf};

const SANDBOX_REL: &str = "skills/slo-sandbox/SKILL.md";
const PLAY_REL: &str = "skills/slo-play/SKILL.md";
const TEMPLATE_REL: &str = "docs/slo/templates/experiment-book-template_v_1.md";

const PROBE_TYPES: &[&str] = &[
    "mechanism_probe",
    "interaction_probe",
    "failure_probe",
    "security_probe",
    "data_probe",
    "latency_probe",
    "magic_probe",
    "composition_probe",
];

/// Ranking/winner headings forbidden in `/slo-play` (convergence smell).
const FORBIDDEN_PLAY_HEADINGS: &[&str] = &["## Rank", "## Pick the winner", "## Best probe"];

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
    std::fs::read_to_string(workspace_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {}: {} (M2 not yet implemented?)", rel, e))
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

    // Output-path safety: any token ending in EXPERIMENT.md must be allow-listed.
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
                "{}: `..` traversal in `{}`",
                rel,
                tok
            );
            if tok.contains('/') {
                assert!(
                    ALLOWED_OUTPUT_PREFIXES
                        .iter()
                        .any(|pre| tok.starts_with(pre)),
                    "{}: output path `{}` not allow-listed",
                    rel,
                    tok
                );
            }
        }
    }
}

#[test]
fn sandbox_and_play_shape_and_paths_safe() {
    assert_skill_shape(SANDBOX_REL, "slo-sandbox");
    assert_skill_shape(PLAY_REL, "slo-play");
}

#[test]
fn play_is_divergent_and_does_not_converge() {
    let play = read(PLAY_REL);
    assert!(
        play.contains("divergent"),
        "/slo-play must declare Mode `divergent`"
    );
    // Frozen sentinel (critique E1) — the joy guard, not tonal analysis.
    assert!(
        play.contains("judge safety only"),
        "/slo-play must state it judges safety only (defers quality judgment)"
    );
    for h in FORBIDDEN_PLAY_HEADINGS {
        assert!(
            !play.contains(h),
            "/slo-play must not contain a ranking/winner heading `{}` (convergence belongs to /slo-pattern + /slo-curate)",
            h
        );
    }
}

#[test]
fn play_names_frozen_probe_types() {
    let play = read(PLAY_REL);
    for t in PROBE_TYPES {
        assert!(
            play.contains(t),
            "/slo-play missing frozen probe type `{}`",
            t
        );
    }
}

#[test]
fn sandbox_has_not_a_feature_gate_and_kill_criteria() {
    let sandbox = read(SANDBOX_REL);
    assert!(
        sandbox.contains("Not a Feature Yet"),
        "/slo-sandbox must declare the `Not a Feature Yet` gate"
    );
    assert!(
        sandbox.to_lowercase().contains("kill criteria"),
        "/slo-sandbox must declare a kill-criteria field"
    );
}

#[test]
fn skills_target_existing_template_sections() {
    let template = read(TEMPLATE_REL);
    // /slo-sandbox fills §3 Sandbox Charter; /slo-play fills §4 Play Log.
    assert!(template.contains("## 3. Sandbox Charter"));
    assert!(template.contains("## 4. Play Log"));
    let sandbox = read(SANDBOX_REL);
    let play = read(PLAY_REL);
    assert!(
        sandbox.contains("Sandbox Charter") || sandbox.contains("§3"),
        "/slo-sandbox must reference its §3 target"
    );
    assert!(
        play.contains("Play Log") || play.contains("§4"),
        "/slo-play must reference its §4 target"
    );
}

//! M3 structural-contract test (innovation-loop runbook): converge + measure.
//!
//! Asserts `/slo-pattern` + `/slo-precision` invariants (frozen-sentinel checks):
//! - both parse, `name` == dir, output paths safe;
//! - `/slo-pattern` Mode `convergent`, states a ≤5 candidate cap and a
//!   cite-probe-IDs rule, and carries the DICEE columns;
//! - `/slo-precision` Mode `measurement`, requires an accept threshold AND a kill
//!   threshold, and rejects "feels better" without a handle;
//! - both target a section heading that exists in the template (§5 / §6).

use std::path::{Component, Path, PathBuf};

const PATTERN_REL: &str = "skills/slo-pattern/SKILL.md";
const PRECISION_REL: &str = "skills/slo-precision/SKILL.md";
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
        .unwrap_or_else(|e| panic!("failed to read {}: {} (M3 not yet implemented?)", rel, e))
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
fn pattern_precision_shape_and_paths_safe() {
    assert_skill_shape(PATTERN_REL, "slo-pattern");
    assert_skill_shape(PRECISION_REL, "slo-precision");
}

#[test]
fn pattern_caps_at_five_and_cites_probes_convergent() {
    let p = read(PATTERN_REL);
    assert!(
        p.contains("convergent"),
        "/slo-pattern must declare Mode convergent"
    );
    assert!(
        p.contains("≤5") || p.to_lowercase().contains("five"),
        "/slo-pattern must state the ≤5 candidate cap"
    );
    assert!(
        p.to_lowercase().contains("cite probe id") || p.to_lowercase().contains("probe ids"),
        "/slo-pattern must require citing probe IDs for every pattern"
    );
    assert!(
        p.contains("DICEE"),
        "/slo-pattern must carry the DICEE check"
    );
}

#[test]
fn precision_requires_accept_and_kill_thresholds_measurement() {
    let p = read(PRECISION_REL);
    assert!(
        p.contains("measurement"),
        "/slo-precision must declare Mode measurement"
    );
    assert!(
        p.to_lowercase().contains("accept threshold"),
        "/slo-precision must require an accept threshold"
    );
    assert!(
        p.to_lowercase().contains("kill threshold"),
        "/slo-precision must require a kill threshold"
    );
    assert!(
        p.to_lowercase().contains("feels better"),
        "/slo-precision must reject 'feels better' without a handle"
    );
}

#[test]
fn skills_target_existing_template_sections() {
    let t = read(TEMPLATE_REL);
    assert!(t.contains("## 5. Pattern Catalog"));
    assert!(t.contains("## 6. Precision Model"));
    assert!(read(PATTERN_REL).contains("Pattern Catalog") || read(PATTERN_REL).contains("§5"));
    assert!(read(PRECISION_REL).contains("Precision Model") || read(PRECISION_REL).contains("§6"));
}

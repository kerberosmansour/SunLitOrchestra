//! M3 structural-contract tests for the engineering skill-improvements runbook.
//!
//! M3 decomposes `/slo-tla` from a long sequential guide into a thin dispatcher
//! plus four skill-local methodology references. These tests keep the cut honest:
//! the cross-stage suitability gate stays in the dispatcher, source methodology
//! travels with the skill install, and Apalache is pinned with a real SHA-256.

use std::fs;
use std::path::{Path, PathBuf};

const METHODOLOGY_FILES: &[(&str, &str)] = &[
    (
        "methodology-elicitation.md",
        "slo-tla-methodology-elicitation",
    ),
    (
        "methodology-abstraction.md",
        "slo-tla-methodology-abstraction",
    ),
    (
        "methodology-counterexample.md",
        "slo-tla-methodology-counterexample",
    ),
    (
        "methodology-verified-design.md",
        "slo-tla-methodology-verified-design",
    ),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_md() -> String {
    read(repo_root().join("skills/slo-tla/SKILL.md"))
}

fn methodology_path(file: &str) -> PathBuf {
    repo_root().join("skills/slo-tla/references").join(file)
}

fn methodology(file: &str) -> String {
    read(methodology_path(file))
}

fn tools_toml() -> String {
    read(repo_root().join("skills/slo-tla/tools.toml"))
}

fn combined_operating_text() -> String {
    let mut combined = skill_md();
    for (file, _) in METHODOLOGY_FILES {
        let path = methodology_path(file);
        if path.exists() {
            combined.push('\n');
            combined.push_str(&read(path));
        }
    }
    combined
}

fn value_for_key_in_section(body: &str, section: &str, key: &str) -> Option<String> {
    let mut in_section = false;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = trimmed == format!("[{section}]");
            continue;
        }

        if !in_section || trimmed.starts_with('#') {
            continue;
        }

        let Some((candidate, value)) = trimmed.split_once('=') else {
            continue;
        };

        if candidate.trim() == key {
            return Some(value.trim().trim_matches('"').to_string());
        }
    }

    None
}

#[test]
fn slo_tla_skill_md_at_or_under_150_lines_without_exception() {
    let skill = skill_md();
    let line_count = skill.lines().count();

    assert!(
        line_count <= 150,
        "skills/slo-tla/SKILL.md must be a thin dispatcher (<= 150 lines), saw {line_count}"
    );
    assert!(
        !skill.contains("# soft-cap-exception:"),
        "M3 must meet the hard <= 150-line target without a soft-cap exception"
    );
}

#[test]
fn methodology_files_exist_with_frontmatter_and_local_references_path() {
    for (file, name) in METHODOLOGY_FILES {
        let path = methodology_path(file);
        assert!(
            path.exists(),
            "missing skill-local methodology file {}",
            path.display()
        );

        let body = read(&path);
        assert!(
            body.starts_with("---\n"),
            "{file} must begin with YAML frontmatter"
        );
        assert!(
            body.contains(&format!("name: {name}")),
            "{file} must declare frontmatter name `{name}`"
        );
        assert!(
            body.contains("source_skill: skills/slo-tla/SKILL.md"),
            "{file} must record the source skill for install/readability continuity"
        );
    }

    assert!(
        repo_root().join("skills/slo-tla/references").is_dir(),
        "methodology files must live under skills/slo-tla/references/ so they travel with the installed skill symlink"
    );
}

#[test]
fn thin_skill_keeps_gate_and_points_to_every_methodology_contract() {
    let skill = skill_md();
    assert!(
        skill.contains("Suitability gate"),
        "cross-stage suitability gate must stay in SKILL.md"
    );

    for link in [
        "references/methodology-elicitation.md",
        "references/methodology-abstraction.md",
        "references/methodology-counterexample.md",
        "references/methodology-verified-design.md",
    ] {
        assert!(
            skill.contains(link),
            "thin SKILL.md must cite `{link}` instead of inlining that methodology"
        );
    }
}

#[test]
fn tla_methodology_disciplines_survive_decomposition() {
    let combined = combined_operating_text();
    let abstraction = methodology("methodology-abstraction.md");
    let counterexample = methodology("methodology-counterexample.md");
    let verified = methodology("methodology-verified-design.md");

    for needle in [
        "the smallest model that still exhibits the bug on the pre-fix design",
        "State-space budget rule of thumb",
        "Drop liveness first",
        "Power-set subsets become presence booleans",
    ] {
        assert!(
            abstraction.contains(needle),
            "abstraction methodology missing `{needle}`"
        );
    }

    for needle in [
        "Raw TLC output is a sequence of states",
        "Broken design assumption",
        "design fix applied to spec",
    ] {
        assert!(
            counterexample.contains(needle),
            "counterexample methodology missing `{needle}`"
        );
    }

    for needle in [
        "Bound is not stated",
        "Fairness is not declared",
        "Naive / pre-fix variant passes silently",
        "Simplifications from the real design",
    ] {
        assert!(
            verified.contains(needle),
            "verified-design methodology missing `{needle}`"
        );
    }

    for rule in [
        "Only after safety holds. Every liveness property needs a fairness assumption",
        "Always run the Naive / pre-fix variant first",
        "TLA+ is not the right tool here",
        "N actors, M requests, K failures",
    ] {
        assert!(
            combined.contains(rule),
            "decomposition lost required rule: {rule}"
        );
    }
}

#[test]
fn apalache_pin_in_tools_toml_and_tlc_pin_unchanged() {
    let tools = tools_toml();

    assert_eq!(
        value_for_key_in_section(&tools, "tlc", "version").as_deref(),
        Some("1.8.0"),
        "M3 must not change the existing TLC version pin"
    );
    assert_eq!(
        value_for_key_in_section(&tools, "tlc", "url").as_deref(),
        Some("https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar"),
        "M3 must not change the existing TLC URL pin"
    );
    assert_eq!(
        value_for_key_in_section(&tools, "tlc", "sha256").as_deref(),
        Some("d5d07d5dab38ddb840c91ec48fa02f28b37a608d5af9a73570018591dbc8ef7f"),
        "M3 must not change the existing TLC SHA-256 pin"
    );

    let version = value_for_key_in_section(&tools, "apalache", "version")
        .expect("Apalache version pin missing");
    let download_url = value_for_key_in_section(&tools, "apalache", "download_url")
        .expect("Apalache download_url pin missing");
    let sha256 =
        value_for_key_in_section(&tools, "apalache", "sha256").expect("Apalache SHA missing");

    assert_eq!(
        version, "0.57.0",
        "Apalache must be pinned to the source-verified release"
    );
    assert_eq!(
        download_url,
        "https://github.com/apalache-mc/apalache/releases/download/v0.57.0/apalache.tgz"
    );
    assert!(
        sha256.len() == 64 && sha256.chars().all(|c| c.is_ascii_hexdigit()),
        "Apalache sha256 must be a 64-character hex digest, saw `{sha256}`"
    );
    assert_eq!(
        sha256, "cb805df9a68e2f278c45e751522aab119b57a454e3e0e96f5d974b969fe52b5d",
        "Apalache pin must match the locally computed v0.57.0 apalache.tgz digest"
    );
}

//! M1 structural-contract test (kani-verification runbook).
//!
//! Asserts the `/slo-kani` skill skeleton invariants from M1 (and is extended
//! by M2 with the four honesty/scope gate sentences):
//!
//! - `skills/slo-kani/SKILL.md` exists with frontmatter `name: slo-kani`
//!   (the precondition for `discover_skills()` auto-discovery).
//! - The honesty contract sentence is present ("proved within the stated …").
//! - The concurrency-refusal gate is present ("concurrency is out of scope").
//! - `skills/slo-kani/tools.toml` pins a concrete `kani-verifier` version
//!   (not `latest`) — supply-chain control, `tm-kani-verification-abuse-4`.
//! - The output-path allow-list clause is present in SKILL.md — path-injection
//!   defense, `tm-kani-verification-abuse-5`.
//! - The candidate-scoring reference exists and is non-trivial.
//!
//! M2 extends this file with: naive-first, sound-stub, verdict-from-tool,
//! fail-closed parsing (ENG-2), and write-path validation (SEC-1) assertions.

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

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

fn extract_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---\n") {
        return None;
    }
    let after_open = &content[4..];
    let close_pos = after_open.find("\n---")?;
    Some(&after_open[..close_pos])
}

fn skill_md() -> String {
    read(&workspace_root().join("skills/slo-kani/SKILL.md"))
}

#[test]
fn slo_kani_frontmatter_complete() {
    let content = skill_md();
    let fm = extract_frontmatter(&content).expect("skills/slo-kani/SKILL.md missing frontmatter");
    let yaml: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(fm).expect("slo-kani frontmatter does not parse as YAML");
    let name = yaml
        .as_mapping()
        .and_then(|m| m.get(serde_yaml_ng::Value::String("name".into())))
        .and_then(|v| v.as_str());
    assert_eq!(
        name,
        Some("slo-kani"),
        "skills/slo-kani/SKILL.md frontmatter `name` must be `slo-kani` (precondition for discover_skills())"
    );
}

#[test]
fn slo_kani_honesty_and_concurrency_gates_present() {
    let content = skill_md();
    assert!(
        content.contains("proved within the stated"),
        "SKILL.md must carry the honesty contract sentence (a green run is `proved within the stated harness, assumptions, and bounds`)"
    );
    assert!(
        content.to_lowercase().contains("concurrency is out of scope"),
        "SKILL.md must carry the concurrency-refusal gate (`concurrency is out of scope`) — tm-kani-verification-abuse-2"
    );
}

/// Returns true if `toml_text` has a `version = "X..."` whose first value char
/// is an ASCII digit (i.e., a concrete version, not `latest`/`*`).
fn pins_concrete_version(toml_text: &str) -> bool {
    for (idx, _) in toml_text.match_indices("version") {
        let rest = &toml_text[idx..];
        if let Some(eq) = rest.find('=') {
            let after = rest[eq + 1..].trim_start();
            if let Some(q) = after.strip_prefix('"') {
                if q.chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn slo_kani_toolchain_pinned() {
    let toml_text = read(&workspace_root().join("skills/slo-kani/tools.toml"));
    assert!(
        toml_text.contains("kani-verifier"),
        "tools.toml must reference the `kani-verifier` crate"
    );
    assert!(
        !toml_text.to_lowercase().contains("version = \"latest\""),
        "tools.toml must not pin `latest` — pin a concrete version (tm-kani-verification-abuse-4)"
    );
    assert!(
        pins_concrete_version(&toml_text),
        "tools.toml must pin a concrete kani-verifier version (digit-leading), not a floating spec"
    );
}

#[test]
fn slo_kani_output_paths_constrained() {
    let content = skill_md();
    assert!(
        content.contains("docs/slo/verify/"),
        "SKILL.md must document the verified-scope report path under docs/slo/verify/"
    );
    assert!(
        content.contains("target crate"),
        "SKILL.md must document the output-path allow-list (harnesses inside the target crate src/) — tm-kani-verification-abuse-5"
    );
}

#[test]
fn slo_kani_candidate_scoring_reference_present() {
    let path = workspace_root().join("skills/slo-kani/references/candidate-scoring.md");
    let content = read(&path);
    assert!(
        content.len() > 400,
        "candidate-scoring.md must be a non-trivial rubric (raise/lower-score signals), got {} bytes",
        content.len()
    );
    assert!(
        content.to_lowercase().contains("unsafe") && content.to_lowercase().contains("score"),
        "candidate-scoring.md must describe scoring signals (e.g. raise score for unsafe code)"
    );
}

// ---------------------------------------------------------------------------
// M2 extensions: harness-generation + run/triage methodology + honesty/scope
// gates. Gate phrases are asserted as EXACT contiguous substrings (kani-m1
// lesson: a `contains()` check is not satisfied by interleaved prose).
// ---------------------------------------------------------------------------

fn reference(name: &str) -> String {
    read(&workspace_root().join(format!("skills/slo-kani/references/{name}")))
}

const M2_REFERENCES: &[&str] = &[
    "harness-generation.md",
    "run-and-triage.md",
    "fallback-strategies.md",
    "verified-scope-writeup.md",
];

const EVAL_CASES: &[&str] = &[
    "happy-path.md",
    "adversarial.md",
    "ambiguous-input.md",
    "missing-context.md",
    "tool-failure.md",
    "high-risk-case.md",
    "outdated-information.md",
];

#[test]
fn slo_kani_m2_reference_files_present() {
    let mut failures: Vec<String> = Vec::new();
    for name in M2_REFERENCES {
        let path = workspace_root().join(format!("skills/slo-kani/references/{name}"));
        match std::fs::read_to_string(&path) {
            Ok(c) if c.len() > 300 => {}
            Ok(c) => failures.push(format!("{name} too small ({} bytes)", c.len())),
            Err(_) => failures.push(format!("{name} missing")),
        }
    }
    assert!(
        failures.is_empty(),
        "M2 method-dispatch references incomplete:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn slo_kani_naive_first_documented() {
    let text = reference("run-and-triage.md");
    assert!(
        text.contains("pre-fix variant must fail first"),
        "run-and-triage.md must carry the anti-vacuity rule (`pre-fix variant must fail first`)"
    );
}

#[test]
fn slo_kani_sound_stub_rule_documented() {
    let text = reference("fallback-strategies.md").to_lowercase();
    assert!(
        text.contains("sound over-approximating stubs only"),
        "fallback-strategies.md must carry the sound-stub rule (`sound over-approximating stubs only`) — tm-kani-verification-abuse-3"
    );
}

#[test]
fn slo_kani_verdict_from_tool_documented() {
    let text = reference("run-and-triage.md");
    assert!(
        text.contains("never from narration"),
        "run-and-triage.md must state the verdict comes from the tool (`never from narration`)"
    );
}

#[test]
fn slo_kani_parser_fails_closed_documented() {
    // ENG-2: unrecognized cargo kani output must fail closed.
    let text = reference("run-and-triage.md");
    assert!(
        text.contains("fail closed") && text.contains("non-pass"),
        "run-and-triage.md must carry the fail-closed parsing rule (`fail closed` + `non-pass`) — ENG-2"
    );
}

#[test]
fn slo_kani_write_path_validation_documented() {
    // SEC-1 / CWE-22: harness write-path validated by construction.
    let text = reference("harness-generation.md");
    let lc = text.to_lowercase();
    assert!(
        lc.contains("target-crate root") && lc.contains("symlink") && lc.contains("reject"),
        "harness-generation.md must carry write-path validation (`target-crate root`, reject `..`/absolute/`symlink`) — SEC-1 / CWE-22"
    );
}

#[test]
fn slo_kani_eval_cases_present() {
    let dir = workspace_root().join("skills/slo-kani/evals");
    let mut failures: Vec<String> = Vec::new();
    for name in EVAL_CASES {
        let path = dir.join(name);
        match std::fs::read_to_string(&path) {
            Ok(c) => {
                if !c.contains("skill: slo-kani") {
                    failures.push(format!("{name} missing `skill: slo-kani` frontmatter"));
                }
            }
            Err(_) => failures.push(format!("{name} missing")),
        }
    }
    assert!(
        failures.is_empty(),
        "M2 eval cases incomplete:\n  - {}",
        failures.join("\n  - ")
    );
}

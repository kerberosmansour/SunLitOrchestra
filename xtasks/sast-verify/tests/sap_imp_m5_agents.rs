//! M5 structural-contract test (sap-imp runbook).
//!
//! Asserts the host-native agent invariants from M5:
//!
//! - Exactly 4 agent files in `agents/` (or 0 — vacuous-pass for the deferred branch).
//! - Every agent has frontmatter with `name`, `role`, `output-paths`, `copilot-fallback`,
//!   `host-required`.
//! - F-SEC-6: every `output-paths` entry, after `Path::components()` canonicalization,
//!   is a strict prefix-subset of `{docs/slo/critique/, docs/slo/verify/}` AND contains
//!   no `Component::ParentDir` segments and is not absolute.
//! - F-ENG-6: `skills/slo-critique/SKILL.md` byte-identical to its post-Fowler
//!   AI architecture M4 baseline, verified via SHA-256 hash constant.
//! - Each agent's `copilot-fallback` field is non-empty (per host-capability matrix).
//! - Each agent file ≤ 200 lines.

use sha2::{Digest, Sha256};
use std::path::{Component, Path, PathBuf};

/// SHA-256 of `skills/slo-critique/SKILL.md`. Updated 2026-05-19 by
/// slo-threat-model runbook M2 (F-ENG-6 amendment recorded in
/// `docs/RUNBOOK-SLO-THREAT-MODEL.md` §6): the canonical portable critique
/// path was deliberately, additively extended with the threat-model
/// read-side contract. No other behavior of the critique path changed.
/// Prior baseline (post-Fowler M4): 5970f982ebb2f9739efd0186a66556ce1bc3db6d9bff125e5ef901991cbca071.
const CRITIQUE_SKILL_SHA256: &str =
    "9e31b7ddd5f4a440d80e96946b28736b2b5c60c74e0d9906e4ed91a1089f26f8";

const EXPECTED_AGENT_NAMES: &[&str] = &[
    "slo-runbook-review-lead",
    "slo-security-reviewer",
    "slo-design-reviewer",
    "slo-verification-lead",
];

const ALLOWED_OUTPUT_PATH_PREFIXES: &[&str] = &["docs/slo/critique/", "docs/slo/verify/"];

const AGENT_FILE_LINE_CAP: usize = 200;

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

#[test]
fn agents_directory_passes_vacuously_or_strictly() {
    let dir = workspace_root().join("agents");
    if !dir.exists() {
        // Vacuous-pass: deferred branch.
        return;
    }
    // Otherwise: expect exactly 4 agent files matching EXPECTED_AGENT_NAMES.
    let mut found: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('.') || !name.ends_with(".md") {
                return None;
            }
            Some(name.trim_end_matches(".md").to_string())
        })
        .collect();
    found.sort();

    let mut expected: Vec<String> = EXPECTED_AGENT_NAMES.iter().map(|s| s.to_string()).collect();
    expected.sort();

    assert_eq!(
        found, expected,
        "agents/ must contain exactly {:?}, found {:?}",
        expected, found
    );
}

#[test]
fn at_most_four_agent_files() {
    let dir = workspace_root().join("agents");
    if !dir.exists() {
        return; // vacuous
    }
    let count: usize = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !name.starts_with('.') && name.ends_with(".md")
        })
        .count();
    assert!(
        count <= 4,
        "agents/ contains {} files; cap is 4 (per F-CEO-1 deferred resolution + M5 contract)",
        count
    );
}

#[test]
fn every_agent_has_required_frontmatter() {
    let dir = workspace_root().join("agents");
    if !dir.exists() {
        return;
    }
    let mut failures: Vec<String> = Vec::new();
    for name in EXPECTED_AGENT_NAMES {
        let path = dir.join(format!("{}.md", name));
        if !path.exists() {
            failures.push(format!("agents/{}.md does not exist", name));
            continue;
        }
        let content = read(&path);
        let Some(fm) = extract_frontmatter(&content) else {
            failures.push(format!("agents/{}.md missing YAML frontmatter", name));
            continue;
        };
        let yaml: serde_yaml_ng::Value = match serde_yaml_ng::from_str(fm) {
            Ok(v) => v,
            Err(e) => {
                failures.push(format!("agents/{}.md frontmatter parse error: {}", name, e));
                continue;
            }
        };
        let map = match yaml.as_mapping() {
            Some(m) => m,
            None => {
                failures.push(format!("agents/{}.md frontmatter is not a mapping", name));
                continue;
            }
        };
        for required in &[
            "name",
            "role",
            "output-paths",
            "copilot-fallback",
            "host-required",
        ] {
            if !map.contains_key(serde_yaml_ng::Value::String((*required).into())) {
                failures.push(format!(
                    "agents/{}.md frontmatter missing field `{}`",
                    name, required
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "M5 frontmatter invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn every_output_path_in_allowed_set() {
    let dir = workspace_root().join("agents");
    if !dir.exists() {
        return;
    }
    let mut failures: Vec<String> = Vec::new();
    for name in EXPECTED_AGENT_NAMES {
        let path = dir.join(format!("{}.md", name));
        if !path.exists() {
            continue;
        }
        let content = read(&path);
        let Some(fm) = extract_frontmatter(&content) else {
            continue;
        };
        let yaml: serde_yaml_ng::Value = match serde_yaml_ng::from_str(fm) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let paths = yaml
            .as_mapping()
            .and_then(|m| m.get(serde_yaml_ng::Value::String("output-paths".into())))
            .and_then(|v| v.as_sequence())
            .cloned()
            .unwrap_or_default();
        if paths.is_empty() {
            failures.push(format!("agents/{}.md has empty `output-paths`", name));
            continue;
        }
        for p in paths {
            let Some(s) = p.as_str() else {
                failures.push(format!(
                    "agents/{}.md `output-paths` entry is not a string",
                    name
                ));
                continue;
            };
            // Path traversal / absolute-path rejection (F-SEC-6).
            let path_obj = Path::new(s);
            if path_obj.is_absolute() {
                failures.push(format!(
                    "agents/{}.md output-path `{}` is absolute (per F-SEC-6)",
                    name, s
                ));
                continue;
            }
            if path_obj
                .components()
                .any(|c| matches!(c, Component::ParentDir))
            {
                failures.push(format!(
                    "agents/{}.md output-path `{}` contains `..` traversal segment (per F-SEC-6)",
                    name, s
                ));
                continue;
            }
            // Prefix membership.
            let in_set = ALLOWED_OUTPUT_PATH_PREFIXES
                .iter()
                .any(|prefix| s.starts_with(prefix));
            if !in_set {
                failures.push(format!(
                    "agents/{}.md output-path `{}` not in allowed prefix set {:?}",
                    name, s, ALLOWED_OUTPUT_PATH_PREFIXES
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "M5 output-path invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn copilot_fallback_documented() {
    let dir = workspace_root().join("agents");
    if !dir.exists() {
        return;
    }
    let mut failures: Vec<String> = Vec::new();
    for name in EXPECTED_AGENT_NAMES {
        let path = dir.join(format!("{}.md", name));
        if !path.exists() {
            continue;
        }
        let content = read(&path);
        let Some(fm) = extract_frontmatter(&content) else {
            continue;
        };
        let yaml: serde_yaml_ng::Value = match serde_yaml_ng::from_str(fm) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let fallback = yaml
            .as_mapping()
            .and_then(|m| m.get(serde_yaml_ng::Value::String("copilot-fallback".into())))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        if fallback.as_deref().unwrap_or("").is_empty() {
            failures.push(format!(
                "agents/{}.md `copilot-fallback` field empty or missing",
                name
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "M5 copilot-fallback invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn agent_file_under_line_cap() {
    let dir = workspace_root().join("agents");
    if !dir.exists() {
        return;
    }
    let mut failures: Vec<String> = Vec::new();
    for name in EXPECTED_AGENT_NAMES {
        let path = dir.join(format!("{}.md", name));
        if !path.exists() {
            continue;
        }
        let content = read(&path);
        let line_count = content.lines().count();
        if line_count > AGENT_FILE_LINE_CAP {
            failures.push(format!(
                "agents/{}.md is {} lines, cap is {} (per F-ENG-? / M5 contract)",
                name, line_count, AGENT_FILE_LINE_CAP
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "M5 line-cap invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn slo_critique_skill_md_unchanged() {
    let path = workspace_root().join("skills/slo-critique/SKILL.md");
    let bytes =
        std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = hasher.finalize();
    let hex: String = hash.iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(
        hex, CRITIQUE_SKILL_SHA256,
        "skills/slo-critique/SKILL.md SHA-256 changed (expected {}, got {}) — M5 must not modify the canonical portable critique path. Update the constant only via a runbook amendment (per F-ENG-6).",
        CRITIQUE_SKILL_SHA256, hex
    );
}

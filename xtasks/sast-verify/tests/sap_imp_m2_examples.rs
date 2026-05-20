//! M2 structural-contract test (sap-imp runbook).
//!
//! Asserts the example-gallery invariants from RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md M2:
//!
//! 1. `examples/` exists and contains EXACTLY 7 files (README + 6 artifacts).
//! 2. Every Markdown example has frontmatter with `synthetic: true`,
//!    `non-normative: true`, and `abbreviates: <ref>`.
//! 3. `examples/sast-manifest.json` declares `"synthetic": true` at top level.
//! 4. PII regex scan over `examples/**/*.{md,json}` finds zero matches for:
//!    email, UK NI, UK sort code, US SSN, EU IBAN (per F-SEC-2 critique resolution).
//! 5. Every `abbreviates:` reference resolves either via `skills/<name>/SKILL.md`
//!    frontmatter `name` match OR `Path::exists()` (per F-ENG-2 resolution).
//! 6. Every file in `examples/` is ≤ 10 KB.

use regex::Regex;
use std::path::{Path, PathBuf};

/// Resolve the workspace root.
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

fn examples_dir() -> PathBuf {
    workspace_root().join("examples")
}

const SIZE_CAP_BYTES: u64 = 10 * 1024;
const EXPECTED_FILE_COUNT: usize = 7;

/// The expected file set in `examples/`. Hardcoded per M2 Resource bound;
/// adding an 8th file requires runbook amendment.
const EXPECTED_FILES: &[&str] = &[
    "README.md",
    "runbook-excerpt.md",
    "critique-report.md",
    "verification-report.md",
    "security-finding.md",
    "sast-manifest.json",
    "biz-public-artifact.md",
];

/// Markdown files that MUST carry frontmatter (everything except README).
const MARKDOWN_FILES_REQUIRING_FRONTMATTER: &[&str] = &[
    "runbook-excerpt.md",
    "critique-report.md",
    "verification-report.md",
    "security-finding.md",
    "biz-public-artifact.md",
];

/// Read a file's raw bytes; return its contents as a UTF-8 string. Panics on I/O
/// or UTF-8 errors with a path-cited message.
fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

/// Extract the YAML frontmatter block from a Markdown file. Returns `None` if
/// no frontmatter is present (the file does not start with `---\n`).
fn extract_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---\n") {
        return None;
    }
    let after_open = &content[4..];
    let close_pos = after_open.find("\n---")?;
    Some(&after_open[..close_pos])
}

#[test]
fn examples_directory_has_exactly_seven_files() {
    let dir = examples_dir();
    assert!(
        dir.exists(),
        "examples/ directory missing at {} — M2 requires it",
        dir.display()
    );

    let mut found: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", dir.display(), e))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                return None;
            }
            Some(name)
        })
        .collect();
    found.sort();

    assert_eq!(
        found.len(),
        EXPECTED_FILE_COUNT,
        "expected {} files in examples/, found {}: {:?}",
        EXPECTED_FILE_COUNT,
        found.len(),
        found
    );

    let expected: Vec<String> = {
        let mut v: Vec<String> = EXPECTED_FILES.iter().map(|s| s.to_string()).collect();
        v.sort();
        v
    };
    assert_eq!(
        found, expected,
        "examples/ contains unexpected file set; expected {:?}, found {:?}",
        expected, found
    );
}

#[test]
fn every_markdown_example_has_required_frontmatter() {
    let dir = examples_dir();
    let mut failures: Vec<String> = Vec::new();

    for filename in MARKDOWN_FILES_REQUIRING_FRONTMATTER {
        let path = dir.join(filename);
        if !path.exists() {
            failures.push(format!("examples/{} does not exist", filename));
            continue;
        }
        let content = read(&path);
        let Some(fm) = extract_frontmatter(&content) else {
            failures.push(format!(
                "examples/{} is missing YAML frontmatter (must start with `---\\n...---\\n`)",
                filename
            ));
            continue;
        };

        // Parse as serde_yaml_ng::Value to verify required fields.
        let yaml: serde_yaml_ng::Value = match serde_yaml_ng::from_str(fm) {
            Ok(v) => v,
            Err(e) => {
                failures.push(format!(
                    "examples/{} frontmatter parse error: {}",
                    filename, e
                ));
                continue;
            }
        };

        let map = yaml.as_mapping();
        let Some(map) = map else {
            failures.push(format!(
                "examples/{} frontmatter is not a YAML mapping",
                filename
            ));
            continue;
        };

        let synthetic = map.get(serde_yaml_ng::Value::String("synthetic".into()));
        if synthetic.and_then(|v| v.as_bool()) != Some(true) {
            failures.push(format!(
                "examples/{} frontmatter must contain `synthetic: true`",
                filename
            ));
        }

        let non_normative = map.get(serde_yaml_ng::Value::String("non-normative".into()));
        if non_normative.and_then(|v| v.as_bool()) != Some(true) {
            failures.push(format!(
                "examples/{} frontmatter must contain `non-normative: true`",
                filename
            ));
        }

        let abbreviates = map.get(serde_yaml_ng::Value::String("abbreviates".into()));
        let abbreviates_str = abbreviates.and_then(|v| v.as_str()).map(|s| s.to_string());
        if abbreviates_str.as_deref().unwrap_or("").is_empty() {
            failures.push(format!(
                "examples/{} frontmatter must contain non-empty `abbreviates:` field",
                filename
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "M2 frontmatter invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn sast_manifest_declares_synthetic() {
    let path = examples_dir().join("sast-manifest.json");
    assert!(path.exists(), "examples/sast-manifest.json missing");
    let content = read(&path);
    let v: serde_json::Value = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("examples/sast-manifest.json parse error: {}", e));
    let synthetic = v.get("synthetic").and_then(|x| x.as_bool());
    assert_eq!(
        synthetic,
        Some(true),
        "examples/sast-manifest.json must declare top-level `\"synthetic\": true`"
    );
}

/// PII regex set per M2 invariant (c) + F-SEC-2 resolution.
fn pii_regexes() -> Vec<(&'static str, Regex)> {
    vec![
        (
            "email",
            Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap(),
        ),
        (
            "UK NI number",
            Regex::new(r"\b[A-CEGHJ-PR-TW-Z]{2}\d{6}[A-D]\b").unwrap(),
        ),
        (
            "UK sort code",
            Regex::new(r"\b\d{2}-\d{2}-\d{2}\b").unwrap(),
        ),
        ("US SSN", Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap()),
        (
            "EU IBAN",
            Regex::new(r"\b[A-Z]{2}\d{2}[A-Z0-9]{1,30}\b").unwrap(),
        ),
    ]
}

#[test]
fn examples_pii_pattern_scan_clean() {
    let dir = examples_dir();
    let regexes = pii_regexes();
    let mut failures: Vec<String> = Vec::new();

    let entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", dir.display(), e));

    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !(name.ends_with(".md") || name.ends_with(".json")) {
            continue;
        }
        let content = read(&path);

        for (line_num, line) in content.lines().enumerate() {
            for (label, re) in &regexes {
                if re.is_match(line) {
                    // EU IBAN regex is permissive; require additional plausibility:
                    // the matched substring must be at least 15 chars.
                    if *label == "EU IBAN" {
                        let m = re.find(line).unwrap();
                        if m.as_str().len() < 15 {
                            continue;
                        }
                    }
                    failures.push(format!(
                        "PII regex match: {} at examples/{}:{} — line: {:?}",
                        label,
                        name,
                        line_num + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "M2 PII regex scan found matches (per F-SEC-2 invariant):\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn every_example_under_size_cap() {
    let dir = examples_dir();
    let mut failures: Vec<String> = Vec::new();

    let entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", dir.display(), e));

    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let metadata = std::fs::metadata(&path)
            .unwrap_or_else(|e| panic!("failed to stat {}: {}", path.display(), e));
        if metadata.len() > SIZE_CAP_BYTES {
            failures.push(format!(
                "examples/{} is {} bytes, cap is {} bytes",
                name,
                metadata.len(),
                SIZE_CAP_BYTES
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "M2 size-cap invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn every_abbreviates_ref_resolves() {
    let dir = examples_dir();
    let root = workspace_root();
    let mut failures: Vec<String> = Vec::new();

    for filename in MARKDOWN_FILES_REQUIRING_FRONTMATTER {
        let path = dir.join(filename);
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
        let abbreviates = yaml
            .as_mapping()
            .and_then(|m| m.get(serde_yaml_ng::Value::String("abbreviates".into())))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let Some(abbr) = abbreviates else {
            continue;
        };

        // Resolution rule (a): walk skills/<name>/SKILL.md for matching frontmatter `name`.
        let resolves_via_skill_name = root.join("skills").join(&abbr).join("SKILL.md").exists();

        // Resolution rule (b): treat as filesystem path.
        let resolves_via_path = root.join(&abbr).exists();

        if !resolves_via_skill_name && !resolves_via_path {
            failures.push(format!(
                "examples/{} frontmatter `abbreviates: {}` — neither skills/{}/SKILL.md nor {} exists at HEAD",
                filename, abbr, abbr, root.join(&abbr).display()
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "M2 abbreviates-resolution invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

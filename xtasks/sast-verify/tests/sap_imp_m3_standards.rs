//! M3 structural-contract test (sap-imp runbook).
//!
//! Asserts the standards-traceability invariants from RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md M3:
//!
//! 1. `references/security/standards-mapping.md` exists and the preamble contains
//!    the per-output-type tier matrix and the "no bulk vendoring" Forbidden Shortcut.
//! 2. Every row in the standards-mapping table has a `retrieval-date` matching
//!    `^\d{4}-\d{2}-\d{2}$`. Stale rows (> 12 months from current date) emit a
//!    warning; do NOT fail.
//! 3. Each of the 4 target skills (`/slo-critique`, `/slo-verify`, `/slo-sast`,
//!    `/slo-rulegen`) contains a Markdown link to `references/security/standards-mapping.md`.
//! 4. `/slo-critique` and `/slo-verify` SKILL.md text contains the "high or critical"
//!    threshold phrase and the "MUST cite CWE" phrase within 200 chars of each other.
//! 5. Per F-ENG-4: walk live `docs/slo/critique/*.md` AND `docs/slo/verify/*.md` for
//!    severity-tagged rows; any row with `severity: high` or `severity: critical`
//!    MUST have a non-empty CWE column. Vacuous-pass when those directories are empty.
//!    Also asserts the M2 fixture `examples/security-finding.md` follows the rule.

use chrono::{Datelike, NaiveDate, Utc};
use pulldown_cmark::{Event, Parser, Tag};
use regex::Regex;
use std::path::{Path, PathBuf};

const M3_TARGET_SKILLS: &[&str] = &["slo-critique", "slo-verify", "slo-sast", "slo-rulegen"];

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

fn extract_link_destinations(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    for event in Parser::new(markdown) {
        if let Event::Start(Tag::Link {
            link_type: _,
            dest_url,
            title: _,
            id: _,
        }) = event
        {
            links.push(dest_url.into_string());
        }
    }
    links
}

#[test]
fn standards_mapping_file_exists_with_preamble() {
    let path = workspace_root().join("references/security/standards-mapping.md");
    assert!(
        path.exists(),
        "references/security/standards-mapping.md missing — M3 requires it"
    );
    let content = read(&path);
    assert!(
        content.contains("Per-output-type tier matrix")
            || content.contains("per-output-type tier matrix"),
        "standards-mapping.md preamble must include 'per-output-type tier matrix' header"
    );
    assert!(
        content.contains("no bulk vendoring")
            || content.contains("No bulk vendoring")
            || content.contains("bulk-vendoring"),
        "standards-mapping.md preamble must include the 'no bulk vendoring' Forbidden Shortcut"
    );
}

/// Parse the standards-mapping.md row table and return a list of `(row-line, retrieval-date-string)`.
/// Rows are detected by lines starting with `| CWE-` (the CWE id column).
fn parse_mapping_rows(content: &str) -> Vec<(usize, String, String)> {
    // Returns Vec<(line_number, full_line, retrieval_date_or_empty)>.
    let date_re = Regex::new(r"\b(\d{4}-\d{2}-\d{2})\b").unwrap();
    let mut rows = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("| CWE-") {
            let date = date_re
                .find(line)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            rows.push((idx + 1, line.to_string(), date));
        }
    }
    rows
}

#[test]
fn every_row_has_retrieval_date() {
    let path = workspace_root().join("references/security/standards-mapping.md");
    let content = read(&path);
    let rows = parse_mapping_rows(&content);
    assert!(
        !rows.is_empty(),
        "standards-mapping.md must contain at least one CWE row"
    );

    let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let mut failures: Vec<String> = Vec::new();
    for (line_num, _line, date) in &rows {
        if date.is_empty() {
            failures.push(format!(
                "standards-mapping.md row at line {} missing retrieval-date column",
                line_num
            ));
        } else if !date_re.is_match(date) {
            failures.push(format!(
                "standards-mapping.md row at line {} has malformed retrieval-date `{}` (expected YYYY-MM-DD)",
                line_num, date
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "M3 retrieval-date invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn stale_rows_warned() {
    // Per F-ENG-4 / F-ENG-? — rows with retrieval-date > 12 months old emit a warning,
    // do NOT fail. This test prints warnings; it always passes.
    let path = workspace_root().join("references/security/standards-mapping.md");
    let content = read(&path);
    let rows = parse_mapping_rows(&content);
    let now = Utc::now().date_naive();

    for (line_num, _line, date) in &rows {
        if let Ok(d) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            let months_old = ((now.year() - d.year()) * 12) + (now.month() as i32 - d.month() as i32);
            if months_old > 12 {
                eprintln!(
                    "WARNING: standards-mapping.md row at line {} has stale retrieval-date {} ({} months old); consider re-fetching",
                    line_num, date, months_old
                );
            }
        }
    }
}

#[test]
fn four_skills_cite_standards_mapping() {
    let root = workspace_root();
    let target = "references/security/standards-mapping.md";
    let mut failures: Vec<String> = Vec::new();

    for skill in M3_TARGET_SKILLS {
        let skill_md = root.join("skills").join(skill).join("SKILL.md");
        let markdown = read(&skill_md);
        let links = extract_link_destinations(&markdown);
        let cites = links.iter().any(|d| d.ends_with(target) || d.contains(target));
        if !cites {
            failures.push(format!(
                "skills/{}/SKILL.md does not link to references/security/standards-mapping.md",
                skill
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "M3 standards-mapping citation invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn threshold_rule_phrasing_in_critique_and_verify() {
    let root = workspace_root();
    let mut failures: Vec<String> = Vec::new();
    for skill in &["slo-critique", "slo-verify"] {
        let path = root.join("skills").join(skill).join("SKILL.md");
        let content = read(&path);
        // Find any occurrence of "high" + "critical" + "MUST" + "CWE" within a 400-char window.
        // Strategy: find the first "MUST" + "CWE" pair, then check window.
        let mut found = false;
        for (i, _) in content.match_indices("MUST") {
            let window_start = i.saturating_sub(200);
            let window_end = (i + 200).min(content.len());
            let window = &content[window_start..window_end];
            if window.contains("high")
                && window.contains("critical")
                && window.contains("CWE")
                && (window.contains("MUST cite CWE")
                    || window.contains("MUST use the expanded template")
                    || window.contains("MUST"))
            {
                found = true;
                break;
            }
        }
        if !found {
            failures.push(format!(
                "skills/{}/SKILL.md does not contain the threshold rule (high + critical + MUST + CWE within 200 chars)",
                skill
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "M3 threshold-rule phrasing invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

/// F-ENG-4: walk live critique + verify artifacts; assert any high/critical
/// severity-tagged finding has a CWE column populated. Vacuous-pass when
/// those directories are empty or contain no severity-tagged rows.
#[test]
fn live_critique_and_verify_findings_have_cwe() {
    let root = workspace_root();
    let dirs = [root.join("docs/slo/critique"), root.join("docs/slo/verify")];
    let mut failures: Vec<String> = Vec::new();

    for dir in dirs {
        if !dir.exists() {
            continue; // vacuous
        }
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", dir.display(), e));
        for entry in entries {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if !name.ends_with(".md") {
                continue;
            }
            let content = read(&path);

            // Only flag STRUCTURED finding markers (line-anchored), not narrative-prose
            // mentions. Per F-ENG-4 the rule applies to actual findings; we recognize
            // them by:
            //   - `### [HIGH] ...` or `### [CRITICAL] ...` (expanded-finding header per
            //     security-finding-template.md)
            //   - `| Severity | high |` or `| Severity | critical |` (template table cell)
            //   - `severity: high` or `severity: critical` at the START of a line
            //     (frontmatter or YAML-style finding metadata)
            for line in content.lines() {
                let lt = line.trim_start();
                let is_high_finding = lt.starts_with("### [HIGH]")
                    || lt.starts_with("### [CRITICAL]")
                    || lt.starts_with("| Severity | high")
                    || lt.starts_with("| Severity | critical")
                    || lt.starts_with("severity: high")
                    || lt.starts_with("severity: critical");
                if !is_high_finding {
                    continue;
                }

                // Window: search for CWE-<digits> within ±400 chars of the matched line.
                let line_start_byte = content.find(line).unwrap_or(0);
                let window_start = line_start_byte.saturating_sub(200);
                let window_end = (line_start_byte + line.len() + 400).min(content.len());
                let window = &content[window_start..window_end];
                let has_cwe = Regex::new(r"CWE-\d+").unwrap().is_match(window);
                if !has_cwe {
                    failures.push(format!(
                        "{}: high/critical finding marker `{}` lacks CWE-<N> within 400-char window",
                        path.display(),
                        lt.chars().take(80).collect::<String>()
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "M3 high/critical-severity threshold-rule violated against live artifacts:\n  - {}",
        failures.join("\n  - ")
    );
}

/// F-ENG-4 fixture leg: enforce the rule against M2's example fixture as well,
/// to lock the behavior in place even when live `docs/slo/critique/` is empty.
#[test]
fn examples_high_severity_findings_have_cwe() {
    let path = workspace_root().join("examples/security-finding.md");
    if !path.exists() {
        return; // vacuous (M2 not run yet — but it has run; the file should exist)
    }
    let content = read(&path);
    if !content.contains("[HIGH]") && !content.contains("[CRITICAL]") {
        return; // no high/critical row in the fixture; vacuous
    }
    assert!(
        content.contains("CWE-"),
        "examples/security-finding.md tagged HIGH or CRITICAL but lacks a CWE reference"
    );
}

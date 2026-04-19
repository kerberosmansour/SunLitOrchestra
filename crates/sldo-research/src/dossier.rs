//! Dossier format, writer, and validator for sldo-research.
//!
//! A **research dossier** is the markdown artifact produced by `sldo-research`.
//! At M4 it is assembled straight from the raw findings emitted by
//! [`crate::research::research_loop`] — each required section is either
//! populated with findings text (Key Findings) or stubbed with a sentinel
//! marker ([`M4_STUB_SENTINEL`]) that M6's synthesis pass will replace with
//! real content. The format is designed so that M7 can feed the file directly
//! to `sldo-plan` as its `prompt_file`.

use std::path::Path;

use anyhow::{Context, Result};
use chrono::Local;

// ── Required section headers ──────────────────────────────────────────────
/// The dossier sections that must always appear. Mirrors
/// `sldo_plan::validate_runbook`'s `REQUIRED_SECTIONS` pattern. The optional
/// "## Repository Context" section is handled separately because its presence
/// is conditional on `--repo-dir`.
pub const REQUIRED_SECTIONS: &[&str] = &[
    "## Executive Summary",
    "## Topic Decomposition",
    "## Key Findings",
    "## Library & Tool Evaluations",
    "## Architecture Options",
    "## API & SDK Documentation",
    "## Design Recommendations",
    "## Risks & Open Questions",
    "## References",
];

/// Header used for the optional repository-context section.
pub const SECTION_REPOSITORY_CONTEXT: &str = "## Repository Context";

/// Placeholder patterns that indicate un-filled template content. These cause
/// [`validate_dossier`] to report an issue. The M4 synthesis stub sentinel
/// ([`M4_STUB_SENTINEL`]) is deliberately *not* in this list — M4 dossiers
/// are expected to contain it until M6's synthesis pass runs.
pub const PLACEHOLDER_PATTERNS: &[&str] = &[
    "[TBD]",
    "[description]",
    "[findings]",
    "[to be filled]",
    "TODO:",
];

/// Sentinel marker inserted into M4 dossier section stubs that M6 will
/// replace with synthesised content. M4's `validate_dossier` tolerates this
/// string; M6/M7's stricter checks detect its presence.
pub const M4_STUB_SENTINEL: &str = "To be synthesised in M6";

/// Minimum dossier size (in bytes) below which the validator reports a
/// size-related issue. Mirrors `validate_runbook`'s 500-byte threshold.
const MIN_DOSSIER_SIZE: usize = 500;

/// Maximum number of characters from the user's prompt to embed in the
/// dossier frontmatter's "topic" line.
const TOPIC_EXCERPT_MAX_CHARS: usize = 200;

/// Write a research dossier to `path`.
///
/// Creates any missing parent directories, embeds a short YAML-ish
/// frontmatter block (topic excerpt, generated-on date, prompt byte count),
/// then writes every entry of [`REQUIRED_SECTIONS`] in order. The
/// `## Key Findings` section carries `findings` verbatim; all other required
/// sections are stubbed with [`M4_STUB_SENTINEL`] (M6 replaces these). When
/// `repo_context` is `Some`, a `## Repository Context` section is inserted
/// between the frontmatter and the required sections.
pub fn write_dossier(
    path: &Path,
    prompt: &str,
    findings: &str,
    repo_context: Option<&str>,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create dossier parent dir: {}", parent.display())
            })?;
        }
    }

    let topic = topic_excerpt(prompt);
    let generated_on = Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string();
    let prompt_bytes = prompt.len();

    let mut body = String::new();
    body.push_str("---\n");
    body.push_str(&format!("topic: {}\n", topic));
    body.push_str(&format!("generated_on: {}\n", generated_on));
    body.push_str(&format!("source_prompt_bytes: {}\n", prompt_bytes));
    body.push_str("generator: sldo-research\n");
    body.push_str("---\n\n");
    body.push_str("# Research Dossier\n\n");
    body.push_str(
        "This dossier is a structured research artifact produced by \
`sldo-research`. It is intended as the `prompt_file` input to `sldo-plan`.\n\n",
    );

    if let Some(ctx) = repo_context {
        body.push_str(SECTION_REPOSITORY_CONTEXT);
        body.push_str("\n\n");
        body.push_str(ctx.trim_end());
        body.push_str("\n\n");
    }

    for section in REQUIRED_SECTIONS {
        body.push_str(section);
        body.push_str("\n\n");
        if *section == "## Key Findings" {
            let content = findings.trim();
            if content.is_empty() {
                body.push_str(M4_STUB_SENTINEL);
                body.push_str(" — raw findings were empty; the synthesis pass will attempt to recover content from scratch files.\n\n");
            } else {
                body.push_str(content);
                body.push_str("\n\n");
            }
        } else {
            body.push_str(M4_STUB_SENTINEL);
            body.push_str(". The M6 synthesis pass will replace this stub with consolidated content derived from the raw findings above.\n\n");
        }
    }

    std::fs::write(path, body)
        .with_context(|| format!("Failed to write dossier: {}", path.display()))?;

    Ok(())
}

/// Validate a dossier file.
///
/// Returns a list of human-readable issue descriptions. An empty vector means
/// the dossier passes M4 validation. Mirrors `sldo_plan::validate_runbook`'s
/// structure: never panics, never returns `Result`.
///
/// At M4, the [`M4_STUB_SENTINEL`] is permitted — M6's synthesis step replaces
/// it, and M7's `check_plan_readiness` (added later) will assert its absence.
pub fn validate_dossier(path: &Path) -> Vec<String> {
    let mut issues = Vec::new();

    if !path.exists() {
        issues.push(format!(
            "Dossier file was not created at: {}",
            path.display()
        ));
        return issues;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            issues.push(format!("Failed to read dossier: {}", e));
            return issues;
        }
    };

    if content.len() < MIN_DOSSIER_SIZE {
        issues.push(format!(
            "Dossier is suspiciously small ({} bytes). May be incomplete.",
            content.len()
        ));
    }

    for section in REQUIRED_SECTIONS {
        if !content.contains(section) {
            issues.push(format!("Missing section: {}", section));
        }
    }

    let placeholder_count: usize = PLACEHOLDER_PATTERNS
        .iter()
        .map(|p| content.matches(p).count())
        .sum();

    if placeholder_count > 0 {
        issues.push(format!(
            "Found {} unfilled placeholder pattern(s).",
            placeholder_count
        ));
    }

    issues
}

/// First `TOPIC_EXCERPT_MAX_CHARS` characters of `prompt`, single-lined so it
/// fits cleanly on one YAML frontmatter line.
fn topic_excerpt(prompt: &str) -> String {
    let flattened: String = prompt
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    let trimmed = flattened.trim();
    if trimmed.chars().count() <= TOPIC_EXCERPT_MAX_CHARS {
        trimmed.to_string()
    } else {
        let head: String = trimmed.chars().take(TOPIC_EXCERPT_MAX_CHARS).collect();
        format!("{}…", head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn unique_tmp(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "sldo_research_m4_unit_{}_{}_{}",
            label,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ))
    }

    // ── Writer: file placement & frontmatter ────────────────────────────────

    #[test]
    fn write_dossier_creates_file_with_required_sections() {
        // Given: a temp dir and valid inputs
        let tmp = unique_tmp("writes_file");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("dossier.md");
        let findings =
            "Finding A\nFinding B\nFinding C with enough text to push the body over 500 bytes. "
                .repeat(10);
        // When
        write_dossier(&path, "sample prompt", &findings, None).unwrap();
        // Then
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.len() >= 500);
        for section in REQUIRED_SECTIONS {
            assert!(
                content.contains(section),
                "expected dossier to contain section {}",
                section
            );
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_creates_nested_parent_directories() {
        // Given: an output path whose parent dirs don't exist yet
        let tmp = unique_tmp("nested");
        let _ = std::fs::remove_dir_all(&tmp);
        let nested = tmp.join("a").join("b").join("c").join("dossier.md");
        // When
        write_dossier(&nested, "p", "findings text", None).unwrap();
        // Then
        assert!(nested.exists());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_includes_repo_context_when_some() {
        // Given: a repo_context string
        let tmp = unique_tmp("repo_ctx_some");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        // When
        write_dossier(&path, "prompt", "findings", Some("Tech: Rust")).unwrap();
        // Then
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains(SECTION_REPOSITORY_CONTEXT));
        assert!(content.contains("Tech: Rust"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_omits_repo_context_when_none() {
        // Given: None for repo_context
        let tmp = unique_tmp("repo_ctx_none");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        // When
        write_dossier(&path, "prompt", "findings", None).unwrap();
        // Then
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            !content.contains(SECTION_REPOSITORY_CONTEXT),
            "dossier should not contain '## Repository Context' when repo_context is None"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_frontmatter_includes_year() {
        // Given: any inputs
        let tmp = unique_tmp("frontmatter_year");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        // When
        write_dossier(&path, "prompt", "findings", None).unwrap();
        // Then: file contains a 4-digit current year
        let content = std::fs::read_to_string(&path).unwrap();
        let year = Local::now().format("%Y").to_string();
        assert!(content.contains(&year), "expected year {} in dossier", year);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_embeds_findings_under_key_findings() {
        // Given: findings text containing a unique marker
        let tmp = unique_tmp("findings_under_key");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let marker = "UNIQUE-MARKER-XYZ";
        // When
        write_dossier(&path, "prompt", marker, None).unwrap();
        // Then: marker is present and appears after the Key Findings header
        let content = std::fs::read_to_string(&path).unwrap();
        let idx_key = content
            .find("## Key Findings")
            .expect("dossier should contain Key Findings header");
        let idx_marker = content
            .find(marker)
            .expect("dossier should contain findings marker");
        assert!(
            idx_marker > idx_key,
            "marker should appear after Key Findings header"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_handles_empty_findings() {
        // Given: empty findings — M4 behaviour: still write a dossier with
        // the synthesis stub sentinel inside Key Findings.
        let tmp = unique_tmp("empty_findings");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        // When
        write_dossier(&path, "prompt", "", None).unwrap();
        // Then
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains(M4_STUB_SENTINEL));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── Validator ──────────────────────────────────────────────────────────

    #[test]
    fn validate_dossier_returns_empty_for_complete_file() {
        // Given: a dossier just written by write_dossier with substantial findings
        let tmp = unique_tmp("validate_complete");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let findings = "Finding line. ".repeat(200);
        write_dossier(&path, "prompt", &findings, None).unwrap();
        // When
        let issues = validate_dossier(&path);
        // Then
        assert!(
            issues.is_empty(),
            "expected no validation issues, got: {:?}",
            issues
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn validate_dossier_flags_missing_file() {
        // Given: a non-existent path
        let tmp = unique_tmp("validate_missing");
        let _ = std::fs::remove_dir_all(&tmp);
        let path = tmp.join("does_not_exist.md");
        // When
        let issues = validate_dossier(&path);
        // Then
        assert!(!issues.is_empty());
        let joined = issues.join("\n");
        assert!(
            joined.to_lowercase().contains("not created")
                || joined.to_lowercase().contains("not found"),
            "expected not-found issue, got: {}",
            joined
        );
    }

    #[test]
    fn validate_dossier_flags_too_small() {
        // Given: a tiny hand-written file
        let tmp = unique_tmp("validate_small");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("tiny.md");
        std::fs::write(&path, "tiny").unwrap();
        // When
        let issues = validate_dossier(&path);
        // Then
        let joined = issues.join("\n");
        assert!(
            joined.to_lowercase().contains("small"),
            "expected size issue, got: {}",
            joined
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn validate_dossier_flags_missing_section() {
        // Given: a hand-written dossier missing "## Key Findings"
        let tmp = unique_tmp("validate_missing_section");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let mut body = String::from("# Dossier\n\n");
        for s in REQUIRED_SECTIONS {
            if *s == "## Key Findings" {
                continue;
            }
            body.push_str(s);
            body.push_str("\n\ncontent\n\n");
        }
        body.push_str(&"padding line. ".repeat(50));
        std::fs::write(&path, body).unwrap();
        // When
        let issues = validate_dossier(&path);
        // Then
        let joined = issues.join("\n");
        assert!(
            joined.contains("Missing section: ## Key Findings"),
            "expected missing-Key-Findings issue, got: {}",
            joined
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn validate_dossier_flags_placeholder_patterns() {
        // Given: a dossier that contains [TBD]
        let tmp = unique_tmp("validate_placeholder");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let mut body = String::from("# Dossier\n\n");
        for s in REQUIRED_SECTIONS {
            body.push_str(s);
            body.push_str("\n\ncontent [TBD]\n\n");
        }
        body.push_str(&"padding line. ".repeat(50));
        std::fs::write(&path, body).unwrap();
        // When
        let issues = validate_dossier(&path);
        // Then
        let joined = issues.join("\n");
        assert!(
            joined.to_lowercase().contains("placeholder"),
            "expected placeholder issue, got: {}",
            joined
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn validate_dossier_tolerates_m4_stub_sentinel() {
        // Given: the writer-produced dossier (full of M4 stub sentinels)
        // at M4, the validator must NOT treat the stub sentinel as a placeholder.
        let tmp = unique_tmp("validate_m4_stub");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let findings = "Finding.\n".repeat(80);
        write_dossier(&path, "prompt", &findings, None).unwrap();
        // Sanity: the stub sentinel is present in the writer output.
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains(M4_STUB_SENTINEL));
        // When
        let issues = validate_dossier(&path);
        // Then: the writer output passes M4 validation despite containing the sentinel.
        assert!(
            issues.is_empty(),
            "M4 writer output should pass validate_dossier; issues: {:?}",
            issues
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── Topic excerpt helper ───────────────────────────────────────────────

    #[test]
    fn topic_excerpt_truncates_long_prompts() {
        // Given: a long single-line prompt
        let long = "a".repeat(400);
        // When
        let excerpt = topic_excerpt(&long);
        // Then
        assert!(
            excerpt.chars().count() <= TOPIC_EXCERPT_MAX_CHARS + 1,
            "excerpt too long: {}",
            excerpt.chars().count()
        );
        assert!(excerpt.ends_with('…'));
    }

    #[test]
    fn topic_excerpt_flattens_newlines() {
        // Given: a prompt with newlines
        let prompt = "line one\nline two\r\nline three";
        // When
        let excerpt = topic_excerpt(prompt);
        // Then: no newline characters remain
        assert!(!excerpt.contains('\n'));
        assert!(!excerpt.contains('\r'));
    }
}

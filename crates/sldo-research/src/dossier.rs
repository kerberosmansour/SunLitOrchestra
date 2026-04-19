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
/// Creates any missing parent directories and embeds a short YAML-ish
/// frontmatter block (topic excerpt, generated-on date, prompt byte count).
///
/// **Synthesis-aware body assembly:**
/// - When `synthesised` is `Some(text)`, the synthesis pass produced a
///   coherent dossier body — write it verbatim after the frontmatter (and
///   the optional repo-context section). The synthesis prompt embeds
///   [`REQUIRED_SECTIONS`] verbatim, so the section structure is the
///   model's responsibility; if the model omitted a header,
///   [`validate_dossier`] will catch it and the operator can re-run.
/// - When `synthesised` is `None` (synthesis failed, returned empty, or the
///   raw findings were empty), fall back to the M4 layout: write every entry
///   of [`REQUIRED_SECTIONS`] in order, populate `## Key Findings` with
///   `findings`, and stub all other required sections with
///   [`M4_STUB_SENTINEL`].
///
/// When `repo_context` is `Some`, a `## Repository Context` section is
/// inserted between the frontmatter and the body in both branches.
pub fn write_dossier(
    path: &Path,
    prompt: &str,
    findings: &str,
    repo_context: Option<&str>,
    synthesised: Option<&str>,
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

    match synthesised {
        Some(synth) if !synth.trim().is_empty() => {
            body.push_str(synth.trim());
            body.push_str("\n");
        }
        _ => {
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

/// Stricter post-M6 readiness check: returns issues if the dossier still
/// contains the M4 stub sentinel ([`M4_STUB_SENTINEL`]). M6's synthesis
/// pass is supposed to replace every stub with synthesised content; the
/// presence of even one sentinel string indicates the synthesis fell back
/// to the raw layout. This is intentionally separate from
/// [`validate_dossier`] (which tolerates the sentinel for M4 compatibility)
/// and is designed to be called by M7's plan-readiness gate.
///
/// Returns an empty vector when the dossier passes the stricter check
/// (sentinel absent), or a non-empty vector with a human-readable issue
/// when the sentinel is present. Returns a "file not found" issue when
/// `path` does not exist.
pub fn check_synthesis_complete(path: &Path) -> Vec<String> {
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

    let count = content.matches(M4_STUB_SENTINEL).count();
    if count > 0 {
        issues.push(format!(
            "Found {} occurrence(s) of the M4 stub sentinel '{}' — synthesis did not replace the stubs.",
            count, M4_STUB_SENTINEL
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
        write_dossier(&path, "sample prompt", &findings, None, None).unwrap();
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
        write_dossier(&nested, "p", "findings text", None, None).unwrap();
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
        write_dossier(&path, "prompt", "findings", Some("Tech: Rust"), None).unwrap();
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
        write_dossier(&path, "prompt", "findings", None, None).unwrap();
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
        write_dossier(&path, "prompt", "findings", None, None).unwrap();
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
        write_dossier(&path, "prompt", marker, None, None).unwrap();
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
        write_dossier(&path, "prompt", "", None, None).unwrap();
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
        write_dossier(&path, "prompt", &findings, None, None).unwrap();
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
        write_dossier(&path, "prompt", &findings, None, None).unwrap();
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

    // ── Synthesis branch (M6) ──────────────────────────────────────────────

    fn synth_body_with_all_sections(marker: &str) -> String {
        let mut body = String::new();
        for s in REQUIRED_SECTIONS {
            body.push_str(s);
            body.push_str("\n\n");
            body.push_str(marker);
            body.push_str(" — synthesised content for this section.\n\n");
        }
        body
    }

    #[test]
    fn write_dossier_with_some_synth_embeds_synth_verbatim() {
        // Given: a synthesised body containing a unique marker
        let tmp = unique_tmp("synth_some");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let marker = "SYNTH-MARKER-XYZ";
        let synth = synth_body_with_all_sections(marker);
        // When
        write_dossier(&path, "prompt", "raw findings", None, Some(&synth)).unwrap();
        // Then: marker is present and the M4 stub sentinel is absent
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains(marker), "synth marker missing from dossier");
        assert!(
            !content.contains(M4_STUB_SENTINEL),
            "synthesised dossier should NOT contain the M4 stub sentinel"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_with_empty_synth_falls_back_to_m4_layout() {
        // Given: an empty synthesised string (counts as None for layout purposes)
        let tmp = unique_tmp("synth_empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        // When
        write_dossier(&path, "prompt", "raw findings", None, Some("   ")).unwrap();
        // Then: dossier still has the stub sentinels (fallback layout)
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            content.contains(M4_STUB_SENTINEL),
            "empty synth should trigger M4 fallback layout containing the sentinel"
        );
        // and raw findings appear under Key Findings
        assert!(content.contains("raw findings"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn write_dossier_with_synth_keeps_repo_context() {
        // Given: synthesised body + a repo_context
        let tmp = unique_tmp("synth_with_repo_ctx");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let synth = synth_body_with_all_sections("SYNTH-OK");
        // When
        write_dossier(
            &path,
            "prompt",
            "raw",
            Some("Tech: Rust\nMSRV: 1.85"),
            Some(&synth),
        )
        .unwrap();
        // Then: repo-context section is present, and synthesised body follows it
        let content = std::fs::read_to_string(&path).unwrap();
        let idx_ctx = content
            .find(SECTION_REPOSITORY_CONTEXT)
            .expect("dossier should contain Repository Context header");
        let idx_synth = content
            .find("SYNTH-OK")
            .expect("dossier should contain synth marker");
        assert!(
            idx_synth > idx_ctx,
            "synthesised body should appear after Repository Context"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── check_synthesis_complete ───────────────────────────────────────────

    #[test]
    fn check_synthesis_complete_flags_stub_sentinel() {
        // Given: an M4-style dossier (writer output with synthesised = None)
        let tmp = unique_tmp("check_synth_stub");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        write_dossier(&path, "prompt", "raw", None, None).unwrap();
        // When
        let issues = check_synthesis_complete(&path);
        // Then: the sentinel is reported
        assert!(!issues.is_empty(), "expected stub-sentinel issue");
        let joined = issues.join("\n");
        assert!(
            joined.to_lowercase().contains("stub")
                || joined.to_lowercase().contains("sentinel")
                || joined.contains("To be synthesised"),
            "expected sentinel-related issue text; got: {}",
            joined
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn check_synthesis_complete_returns_empty_for_clean_synth_dossier() {
        // Given: a synthesised dossier (writer with Some(synth))
        let tmp = unique_tmp("check_synth_clean");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("d.md");
        let synth = synth_body_with_all_sections("CLEAN");
        write_dossier(&path, "prompt", "raw", None, Some(&synth)).unwrap();
        // When
        let issues = check_synthesis_complete(&path);
        // Then: no issues reported
        assert!(
            issues.is_empty(),
            "clean synth dossier should pass check_synthesis_complete; got: {:?}",
            issues
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn check_synthesis_complete_flags_missing_file() {
        // Given: a non-existent path
        let tmp = unique_tmp("check_synth_missing");
        let _ = std::fs::remove_dir_all(&tmp);
        let path = tmp.join("does_not_exist.md");
        // When
        let issues = check_synthesis_complete(&path);
        // Then: returns a not-found issue
        assert!(!issues.is_empty());
        let joined = issues.join("\n");
        assert!(
            joined.to_lowercase().contains("not created")
                || joined.to_lowercase().contains("not found"),
            "expected not-found issue; got: {}",
            joined
        );
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

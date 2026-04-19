//! Research loop orchestration for sldo-research.
//!
//! Drives the multi-phase Claude Code pipeline:
//!   1. Optional repo-context invocation when `--repo-dir` is supplied.
//!   2. Exploration invocation (iteration 1).
//!   3. Deepening invocations for iterations 2..=`max_iterations`.
//!
//! Each invocation gets its own log file under `cfg.log_dir` and persists its
//! captured stdout to a scratch file under the dossier's parent directory.
//! Failures (spawn errors or non-zero exits) are logged and the loop continues
//! with whatever findings have been accumulated so far.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};

use sldo_common::color::warn;
use sldo_common::copilot::ClaudeInvocation;
use sldo_common::logging::LogFile;
use sldo_common::toolflags;

use crate::prompt::{
    build_deepening_prompt, build_exploration_prompt, build_repo_context_prompt,
    build_websearch_prompt, SECTION_KEY_QUESTIONS,
};

/// Configuration for one research-loop run.
pub struct ResearchConfig {
    pub prompt_content: String,
    pub repo_dir: Option<PathBuf>,
    pub output_path: PathBuf,
    pub model: String,
    pub max_iterations: u32,
    /// Maximum number of web-search invocations inserted between exploration
    /// and deepening. Zero skips the phase entirely.
    pub max_searches: u32,
    pub cooldown_secs: u64,
    pub log_dir: PathBuf,
}

/// Result of one research-loop run.
///
/// `raw` holds every phase's captured findings (exploration + deepening)
/// concatenated in run order. `repo_context` is split out so [`crate::dossier::write_dossier`]
/// can emit it as its own "## Repository Context" section when a `--repo-dir`
/// was supplied.
pub struct ResearchFindings {
    pub raw: String,
    pub repo_context: Option<String>,
}

/// Drive the multi-phase research loop. Returns a [`ResearchFindings`] bundle.
/// Per-phase failures are logged and skipped — the loop only returns `Err`
/// when log-file or output-dir setup fails.
pub fn research_loop(cfg: &ResearchConfig) -> Result<ResearchFindings> {
    let output_parent = cfg
        .output_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    std::fs::create_dir_all(&output_parent)
        .with_context(|| format!("Failed to create output dir: {}", output_parent.display()))?;

    let working_dir = cfg
        .repo_dir
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    let mut raw = String::new();
    let mut repo_context: Option<String> = None;

    if let Some(repo) = &cfg.repo_dir {
        let prompt = build_repo_context_prompt(repo);
        match run_phase(cfg, &working_dir, "research-repo-context.log", prompt) {
            Ok(out) => {
                let trimmed = out.trim();
                if !trimmed.is_empty() {
                    repo_context = Some(trimmed.to_string());
                }
            }
            Err(e) => warn(&format!("repo-context phase failed: {:#}", e)),
        }
    }

    let exploration_prompt = build_exploration_prompt(&cfg.prompt_content, cfg.repo_dir.as_deref());
    match run_phase(
        cfg,
        &working_dir,
        "research-exploration.log",
        exploration_prompt,
    ) {
        Ok(out) => {
            persist_scratch(&output_parent, 1, &out)?;
            raw.push_str(&out);
        }
        Err(e) => warn(&format!("exploration phase failed: {:#}", e)),
    }

    // ── Web-search phase (M5) ───────────────────────────────────────────────
    // Inserted between exploration and deepening. `max_searches == 0` skips
    // the phase entirely — no log files, no invocations. Per-search failures
    // log a warning and continue; they never halt the loop. No cooldown here:
    // web-search invocations are lighter than deepening passes and don't need
    // the inter-call pause that deepening uses to smooth rate limits.
    let questions = extract_key_questions(&raw);
    for n in 1..=cfg.max_searches {
        let websearch_prompt = build_websearch_prompt(&cfg.prompt_content, &questions, n);
        let log_name = format!("research-websearch-{}.log", n);
        match run_phase(cfg, &working_dir, &log_name, websearch_prompt) {
            Ok(out) => {
                if !raw.is_empty() {
                    raw.push_str("\n\n");
                }
                raw.push_str(&out);
            }
            Err(e) => warn(&format!("web-search invocation {} failed: {:#}", n, e)),
        }
    }

    for iter in 2..=cfg.max_iterations {
        if cfg.cooldown_secs > 0 {
            std::thread::sleep(Duration::from_secs(cfg.cooldown_secs));
        }
        let deepen_prompt =
            build_deepening_prompt(&cfg.prompt_content, &raw, iter, cfg.repo_dir.as_deref());
        let log_name = format!("research-deepen-{}.log", iter);
        match run_phase(cfg, &working_dir, &log_name, deepen_prompt) {
            Ok(out) => {
                persist_scratch(&output_parent, iter, &out)?;
                if !raw.is_empty() {
                    raw.push_str("\n\n");
                }
                raw.push_str(&out);
            }
            Err(e) => warn(&format!("deepening iteration {} failed: {:#}", iter, e)),
        }
    }

    Ok(ResearchFindings { raw, repo_context })
}

/// Invoke Claude Code for one phase, capturing stdout and tee-ing it to the
/// terminal. Non-zero exit codes log a warning but still return `Ok` with the
/// captured text. Spawn failures bubble up as `Err`.
fn run_phase(
    cfg: &ResearchConfig,
    working_dir: &Path,
    log_name: &str,
    prompt: String,
) -> Result<String> {
    let log_file = LogFile::new(&cfg.log_dir, log_name)?;
    log_file.append(&format!("Phase prompt: {} bytes", prompt.len()))?;

    let invocation = ClaudeInvocation {
        prompt,
        model: cfg.model.clone(),
        allow_flags: toolflags::research_allow_flags(),
        deny_flags: toolflags::research_deny_flags(),
        working_dir: working_dir.to_path_buf(),
    };

    let mut captured = String::new();
    let exit_code = invocation.run_with_callback(&log_file, |line, stream| match stream {
        "stdout" => {
            println!("{}", line);
            captured.push_str(line);
            captured.push('\n');
        }
        _ => eprintln!("{}", line),
    })?;

    if exit_code != 0 {
        warn(&format!(
            "claude exited with code {} during {}; continuing with partial findings",
            exit_code, log_name
        ));
    }

    Ok(captured)
}

/// Pull the body of the `## Key Questions` section out of an exploration-phase
/// dump so the web-search phase can partition it across invocations. Returns
/// an empty string when the section is absent or empty — the web-search prompt
/// then falls back to broad topic research.
fn extract_key_questions(exploration_output: &str) -> String {
    let header = SECTION_KEY_QUESTIONS;
    let Some(start) = exploration_output.find(header) else {
        return String::new();
    };
    let after_header = &exploration_output[start + header.len()..];
    // Stop at the next top-level `## ` header (but skip the `##` line we just
    // consumed — find requires at least one newline before the next header).
    let body = match after_header.find("\n## ") {
        Some(end) => &after_header[..end],
        None => after_header,
    };
    body.trim().to_string()
}

fn persist_scratch(parent: &Path, iter: u32, contents: &str) -> Result<()> {
    let scratch = parent.join(format!(".research-scratch-iter-{}.md", iter));
    std::fs::write(&scratch, contents)
        .with_context(|| format!("Failed to write scratch file: {}", scratch.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_tmp(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "sldo_research_m3_unit_{}_{}_{}",
            label,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ))
    }

    // ── ResearchConfig construction ────────────────────────────────────────

    #[test]
    fn config_struct_constructs_with_all_fields_public() {
        // Given/When: construct ResearchConfig with explicit values
        let cfg = ResearchConfig {
            prompt_content: "p".to_string(),
            repo_dir: Some(PathBuf::from("/r")),
            output_path: PathBuf::from("o.md"),
            model: "m".to_string(),
            max_iterations: 2,
            max_searches: 3,
            cooldown_secs: 1,
            log_dir: PathBuf::from(".sldo-logs"),
        };
        // Then: every field is publicly accessible after construction
        assert_eq!(cfg.prompt_content, "p");
        assert_eq!(cfg.repo_dir, Some(PathBuf::from("/r")));
        assert_eq!(cfg.output_path, PathBuf::from("o.md"));
        assert_eq!(cfg.model, "m");
        assert_eq!(cfg.max_iterations, 2);
        assert_eq!(cfg.max_searches, 3);
        assert_eq!(cfg.cooldown_secs, 1);
        assert_eq!(cfg.log_dir, PathBuf::from(".sldo-logs"));
    }

    #[test]
    fn config_struct_accepts_optional_repo_dir_none() {
        // Given/When: construct ResearchConfig with repo_dir = None
        let cfg = ResearchConfig {
            prompt_content: String::new(),
            repo_dir: None,
            output_path: PathBuf::from("x.md"),
            model: String::new(),
            max_iterations: 0,
            max_searches: 0,
            cooldown_secs: 0,
            log_dir: PathBuf::from("."),
        };
        // Then: repo_dir is None, max_searches is 0 (skip phase)
        assert!(cfg.repo_dir.is_none());
        assert_eq!(cfg.max_searches, 0);
    }

    // ── Key-questions extraction helper ───────────────────────────────────

    #[test]
    fn extract_key_questions_returns_body_between_headers() {
        // Given: an exploration dump with `## Key Questions` followed by `## Initial Findings`
        let dump = "## Topic Decomposition\n\n- A\n- B\n\n## Key Questions\n\n- q1\n- q2\n- q3\n\n## Initial Findings\n\nStuff.\n";
        // When: extract_key_questions is called
        let out = extract_key_questions(dump);
        // Then: only the body of Key Questions is returned (trimmed)
        assert!(out.contains("- q1"));
        assert!(out.contains("- q3"));
        assert!(!out.contains("Topic Decomposition"));
        assert!(!out.contains("Initial Findings"));
    }

    #[test]
    fn extract_key_questions_missing_header_returns_empty() {
        // Given: an exploration dump with no Key Questions header
        let dump = "## Topic Decomposition\n- A\n## Initial Findings\nStuff\n";
        // When: extract_key_questions is called
        let out = extract_key_questions(dump);
        // Then: returns empty string (caller falls back to broad research)
        assert!(out.is_empty());
    }

    // ── Tool-flag preservation (M5 regression guard) ─────────────────────

    #[test]
    fn research_allow_flags_include_web_tools() {
        // Given: research-phase tool flags
        // When:  inspected
        // Then:  both WebFetch and WebSearch are present (M5 depends on them)
        let flags = sldo_common::toolflags::research_allow_flags().join(" ");
        assert!(flags.contains("WebFetch"), "research flags missing WebFetch: {}", flags);
        assert!(flags.contains("WebSearch"), "research flags missing WebSearch: {}", flags);
    }

    #[test]
    fn plan_flags_do_not_include_web_search() {
        // Given: plan-phase tool flags
        // When:  inspected
        // Then:  WebSearch is absent — planning is not a web phase and we do
        //        not want Claude to use the internet while generating runbooks
        let flags = sldo_common::toolflags::plan_allow_flags().join(" ");
        assert!(
            !flags.contains("WebSearch"),
            "plan flags should not include WebSearch: {}",
            flags
        );
    }

    #[test]
    fn extract_key_questions_last_section_returns_tail() {
        // Given: an exploration dump where Key Questions is the final section
        let dump = "## Initial Findings\nStuff\n\n## Key Questions\n\n- q1\n- q2\n";
        // When: extract_key_questions is called
        let out = extract_key_questions(dump);
        // Then: returns the tail after the header
        assert!(out.contains("- q1"));
        assert!(out.contains("- q2"));
    }

    // ── Scratch persistence helper ─────────────────────────────────────────

    #[test]
    fn persist_scratch_writes_named_file() {
        // Given: a scratch parent dir
        let tmp = unique_tmp("scratch");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        // When: persist_scratch is called
        persist_scratch(&tmp, 7, "scratch contents").unwrap();
        // Then: file exists with the iteration-numbered name and exact contents
        let path = tmp.join(".research-scratch-iter-7.md");
        assert!(path.exists());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "scratch contents");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn persist_scratch_handles_empty_contents() {
        // Given: a scratch parent dir
        let tmp = unique_tmp("scratch_empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        // When: persist_scratch is called with empty findings
        persist_scratch(&tmp, 1, "").unwrap();
        // Then: the file is created (empty) so the caller can inspect it
        let path = tmp.join(".research-scratch-iter-1.md");
        assert!(path.exists());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn persist_scratch_filename_pattern() {
        // Given: distinct iteration numbers
        let tmp = unique_tmp("scratch_pattern");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        // When: persisting for iters 1 and 2
        persist_scratch(&tmp, 1, "one").unwrap();
        persist_scratch(&tmp, 2, "two").unwrap();
        // Then: each file exists at its iteration-numbered path
        assert!(tmp.join(".research-scratch-iter-1.md").exists());
        assert!(tmp.join(".research-scratch-iter-2.md").exists());
        let _ = std::fs::remove_dir_all(&tmp);
    }
}

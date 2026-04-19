//! Pure prompt constructors for the sldo-research pipeline.
//!
//! Each `build_*_prompt` function takes its inputs and returns a `String`.
//! No I/O, no network, no env reads — caller supplies any path canonicalization.
//! The Claude Code CLI is the consumer; section names embedded here are the
//! contract that later milestones (M3 capture, M4 validation, M6 synthesis)
//! rely on.

use std::path::Path;

// ── Exploration-phase section headers ──────────────────────────────────────
pub const SECTION_TOPIC_DECOMPOSITION: &str = "## Topic Decomposition";
pub const SECTION_KEY_QUESTIONS: &str = "## Key Questions";
pub const SECTION_INITIAL_FINDINGS: &str = "## Initial Findings";
pub const SECTION_REPO_CONTEXT: &str = "## Repo Context";

// ── Deepening-phase section headers ────────────────────────────────────────
pub const SECTION_DEEPENED_FINDINGS: &str = "## Deepened Findings";
pub const SECTION_LIBRARY_EVAL: &str = "## Library Evaluations";
pub const SECTION_ARCHITECTURE_OPTIONS: &str = "## Architecture Options";
pub const SECTION_UNANSWERED_QUESTIONS: &str = "## Unanswered Questions";

// ── Repo-context-phase section headers ─────────────────────────────────────
pub const SECTION_TECH_STACK: &str = "## Tech Stack";
pub const SECTION_PROJECT_STRUCTURE: &str = "## Project Structure";
pub const SECTION_BUILD_AND_TEST: &str = "## Build & Test";
pub const SECTION_EXISTING_PATTERNS: &str = "## Existing Patterns";
pub const SECTION_CONSTRAINTS: &str = "## Constraints";

// ── Web-search-phase section headers ───────────────────────────────────────
pub const SECTION_WEB_SEARCH_RESULTS: &str = "## Web Search Results";
pub const SECTION_DOCUMENTATION_FOUND: &str = "## Documentation Found";
pub const SECTION_LIBRARY_VERSIONS: &str = "## Library Versions";

/// Maximum size of `previous_findings` embedded in the deepening prompt.
/// Keeps total prompt size bounded so we don't blow Claude's context window.
const DEEPENING_FINDINGS_MAX_BYTES: usize = 32 * 1024; // 32 KiB
const DEEPENING_TRUNCATION_MARKER: &str =
    "[truncated — earlier findings omitted to fit prompt size limits]";

/// Build the exploration-phase prompt.
///
/// Instructs Claude Code to decompose the user's research topic, optionally
/// explore the supplied repo, and emit findings under the four
/// exploration-phase section headers.
pub fn build_exploration_prompt(prompt_content: &str, repo_dir: Option<&Path>) -> String {
    let repo_section = match repo_dir {
        Some(path) => format!(
            r#"

A repository is available for context at: `{repo}`

While exploring the topic, also examine this repository to understand the
codebase that the research must integrate with. Focus on tech stack, existing
patterns, and constraints. Surface repo-specific findings under the
"{repo_section}" header below.
"#,
            repo = path.display(),
            repo_section = SECTION_REPO_CONTEXT,
        ),
        None => String::new(),
    };

    let repo_header_block = match repo_dir {
        Some(_) => format!(
            "\n\n{}\n\n[Repository-specific observations.]",
            SECTION_REPO_CONTEXT
        ),
        None => String::new(),
    };

    format!(
        r#"You are an expert research agent. Your job is to perform the **exploration phase**
of a multi-phase research pipeline. Decompose the user's topic into well-scoped
sub-questions and gather initial findings.

## INPUT

<user_prompt>
{prompt_content}
</user_prompt>
{repo_section}

## YOUR TASK

1. **Decompose the topic** into 5-10 specific, narrowly scoped sub-questions
   that, when answered together, would fully address the user's research goal.

2. **Identify key concepts, libraries, APIs, and standards** referenced (or
   implied) by the prompt. List each one with a one-line summary.

3. **Capture initial findings** from your existing knowledge. Be explicit about
   confidence: mark uncertain claims so a later deepening pass can verify them.

4. **(If a repo is provided)** explore it: read `README*`, manifest files
   (`Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`), top-level
   directory layout, and a sampling of source files. Summarise tech stack,
   existing patterns, and constraints under the repo-context header.

## OUTPUT FORMAT

Emit a markdown document with the following section headers, in order. Do not
rename or reorder them. Each section must contain content (not placeholders).

{topic_decomp}

[A bulleted list of sub-questions with brief rationale for each.]

{key_questions}

[Prioritised list of the most important questions to answer next.]

{initial_findings}

[Bullet points of factual claims and references gathered so far. Note
confidence (high/medium/low) where applicable.]{repo_header_block}

## HARD RULES

- Do NOT invent facts. If unsure, mark a finding as "needs verification".
- Do NOT produce milestones or implementation plans — that is a later stage.
- Stay within the section headers above. Do not invent additional top-level
  headers.
"#,
        topic_decomp = SECTION_TOPIC_DECOMPOSITION,
        key_questions = SECTION_KEY_QUESTIONS,
        initial_findings = SECTION_INITIAL_FINDINGS,
    )
}

/// Build the deepening-phase prompt.
///
/// Embeds prior findings (truncated if oversized), asks Claude Code to answer
/// unanswered questions, evaluate libraries with pros/cons, and emit deepened
/// findings under the four deepening-phase section headers.
///
/// When `iteration >= 3`, the prompt also asks for consolidation/synthesis
/// preparation (groundwork for M6's dedicated synthesis pass).
pub fn build_deepening_prompt(
    prompt_content: &str,
    previous_findings: &str,
    iteration: u32,
    repo_dir: Option<&Path>,
) -> String {
    let (truncated_findings, was_truncated) =
        if previous_findings.len() > DEEPENING_FINDINGS_MAX_BYTES {
            // Keep the tail — most recent additions are usually the most relevant.
            let start = previous_findings.len() - DEEPENING_FINDINGS_MAX_BYTES;
            // Avoid splitting mid-codepoint.
            let safe_start = (start..previous_findings.len())
                .find(|i| previous_findings.is_char_boundary(*i))
                .unwrap_or(previous_findings.len());
            (&previous_findings[safe_start..], true)
        } else {
            (previous_findings, false)
        };

    let truncation_note = if was_truncated {
        DEEPENING_TRUNCATION_MARKER
    } else {
        ""
    };

    let repo_section = match repo_dir {
        Some(path) => format!(
            r#"

A repository is available at: `{repo}`. Re-examine it as needed to ground your
deepened findings in the codebase's actual constraints.
"#,
            repo = path.display(),
        ),
        None => String::new(),
    };

    let synthesis_directive = if iteration >= 3 {
        r#"

This is iteration 3 or later — begin to **consolidate and synthesise** the
findings. Resolve contradictions, deduplicate overlapping claims, and prepare
the material for a final synthesis pass.
"#
    } else {
        ""
    };

    format!(
        r#"You are an expert research agent. This is the **deepening phase**
(iteration {iteration}) of a multi-phase research pipeline. Build on the
previous findings: answer open questions, evaluate specific libraries, and
sharpen recommendations.

## INPUT

<user_prompt>
{prompt_content}
</user_prompt>

### Previous Findings
{truncation_note}
<previous_findings>
{previous}
</previous_findings>
{repo_section}{synthesis_directive}

## YOUR TASK

1. **Answer the most important unanswered questions** from the previous round.
   Cite sources (URLs, doc paths, file paths) where applicable.

2. **Evaluate candidate libraries / tools / APIs** with concrete pros and cons,
   version compatibility notes, and license/maintenance status.

3. **Sketch architecture options** that satisfy the user's goal. For each
   option list trade-offs and the situations it best fits.

4. **List remaining unanswered questions** so a future iteration knows where
   to focus.

## OUTPUT FORMAT

Emit a markdown document with the following section headers, in order:

{deepened}

[Refined factual claims, with confidence levels and citations.]

{lib_eval}

[Per-candidate evaluation: name, version, pros, cons, fit.]

{arch_options}

[At least two architecture options, each with trade-offs.]

{unanswered}

[Open questions still to resolve, ordered by priority.]

## HARD RULES

- Do NOT invent facts. Distinguish your inferences from sourced claims.
- Do NOT produce milestones or implementation plans.
- Stay within the section headers above.
"#,
        iteration = iteration,
        previous = truncated_findings,
        deepened = SECTION_DEEPENED_FINDINGS,
        lib_eval = SECTION_LIBRARY_EVAL,
        arch_options = SECTION_ARCHITECTURE_OPTIONS,
        unanswered = SECTION_UNANSWERED_QUESTIONS,
    )
}

/// Build the repo-context prompt.
///
/// Instructs Claude Code to read README, manifest files, top-level structure,
/// and emit a summary under the five repo-context section headers.
pub fn build_repo_context_prompt(repo_dir: &Path) -> String {
    format!(
        r#"You are an expert research agent. Your job is to gather **repository
context** that will ground the rest of a multi-phase research pipeline. The
target repository is: `{repo}`.

## YOUR TASK

1. Read top-level files: `README*`, `Cargo.toml` / `package.json` /
   `pyproject.toml` / `go.mod` / `Makefile` (whichever exist).
2. Walk the top-level directory structure (one level deep is sufficient — note
   any `crates/`, `src/`, `tests/`, `docs/`, `app/`, `pkg/` style trees).
3. Identify build and test commands (`make`, `cargo`, `npm`, `pytest`, etc.).
4. Spot-check a handful of source files to identify recurring patterns (error
   handling, logging, module layout, async runtime, framework choices).
5. Surface constraints that future research must respect (target platforms,
   licensing, MSRV, Node version, etc.).

## OUTPUT FORMAT

Emit a markdown document with the following section headers, in order. Each
section must contain real content discovered from the repo.

{tech_stack}

[Languages, frameworks, runtime versions, key crates / packages.]

{project_structure}

[Top-level layout with a one-line summary per directory.]

{build_and_test}

[Concrete commands to build, lint, and test the project.]

{existing_patterns}

[Recurring patterns: error handling, logging, async style, module conventions.]

{constraints}

[Hard constraints: MSRV, supported platforms, license obligations, perf
requirements, etc.]

## HARD RULES

- Do NOT invent facts. If a file is missing, say so explicitly.
- Do NOT modify any files in the repository.
- Stay within the section headers above.
"#,
        repo = repo_dir.display(),
        tech_stack = SECTION_TECH_STACK,
        project_structure = SECTION_PROJECT_STRUCTURE,
        build_and_test = SECTION_BUILD_AND_TEST,
        existing_patterns = SECTION_EXISTING_PATTERNS,
        constraints = SECTION_CONSTRAINTS,
    )
}

/// Number of questions fed to a single web-search invocation. Subsequent
/// invocations (search_index 2, 3, …) walk through the question list in
/// non-overlapping slices of this size so multiple searches cover distinct
/// ground rather than repeating the same queries.
const WEBSEARCH_QUESTIONS_PER_INVOCATION: usize = 3;

/// Build the web-search-phase prompt.
///
/// Drives a Claude Code invocation that uses built-in `WebFetch` + `WebSearch`
/// tools to look up current documentation, library versions, and
/// best-practice articles for a slice of the research questions. `search_index`
/// (1-based) selects which slice of `questions` to focus on — this lets the
/// research loop call this builder N times with monotonically increasing
/// indexes and have each call cover a distinct subset.
///
/// Questions are partitioned line-by-line: index 1 covers lines
/// 0..WEBSEARCH_QUESTIONS_PER_INVOCATION, index 2 covers the next slice, and
/// so on. If `questions` is empty (or the slice for this index is empty), the
/// prompt instructs Claude to research the topic broadly instead of against a
/// specific question list.
pub fn build_websearch_prompt(topic: &str, questions: &str, search_index: u32) -> String {
    let all_questions: Vec<&str> = questions
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    // Partition: 1-based index → zero-based slice.
    let idx0 = search_index.saturating_sub(1) as usize;
    let start = idx0 * WEBSEARCH_QUESTIONS_PER_INVOCATION;
    let end = (start + WEBSEARCH_QUESTIONS_PER_INVOCATION).min(all_questions.len());

    let focus_block = if start >= all_questions.len() {
        // No specific questions for this index — fall back to broad research.
        "No specific questions remain for this search slice. Research the topic \
broadly: look up the most commonly recommended libraries, the latest versions, \
and recent (within the last 2 years) best-practice articles."
            .to_string()
    } else {
        let slice = &all_questions[start..end];
        let mut block = String::from(
            "Focus your web searches on answering the following questions. \
Do NOT branch beyond this list — later invocations cover other questions.\n\n",
        );
        for q in slice {
            block.push_str("- ");
            block.push_str(q);
            block.push('\n');
        }
        block
    };

    format!(
        r#"You are an expert research agent running the **web-search phase**
(search {search_index}) of a multi-phase research pipeline. Your job is to use
your built-in web-search tools to gather current, authoritative external
information about the topic below.

## INPUT

<topic>
{topic}
</topic>

### Focus

{focus_block}

## YOUR TASK

1. **Run up to {searches_per_invocation} web searches** focused on the question slice above.
   Prefer official documentation, reputable engineering blogs, and primary
   sources. Avoid low-quality content farms.

2. **Cross-check versions.** When a library or tool is mentioned, look up the
   latest stable release, any MSRV / runtime-version requirements, and
   recent breaking-change notes.

3. **Capture references explicitly.** Every URL you consult must appear in
   the output with its page title so a later synthesis pass can fold them
   into a reference list.

4. **Stay within scope.** Do NOT produce milestones, implementation plans,
   or opinionated architecture recommendations — that is a later stage.

## OUTPUT FORMAT

Emit a markdown document with exactly these three section headers, in order.
Each section must contain real content grounded in what your web searches
returned. If a search turned up nothing for a given header, write one short
sentence explaining that rather than inventing content.

{websearch_results}

[Per-question summary of what the web searches found, each claim paired with
the URL that supports it. Use bullet lists.]

{documentation_found}

[URL + title pairs for any official documentation, reference guides, or spec
documents discovered. One bullet per link, in the format: `- [Title](URL)`.]

{library_versions}

[Per-library table or bullet list of: library name, latest stable version,
release date if known, and a one-line compatibility/status note.]

## HARD RULES

- Do NOT invent URLs. Every link must come from a search you actually ran.
- Do NOT invent facts. If a search is inconclusive, say so explicitly.
- Do NOT produce milestones or implementation plans.
- Stay within the three section headers above.
"#,
        topic = topic,
        search_index = search_index,
        focus_block = focus_block,
        searches_per_invocation = WEBSEARCH_QUESTIONS_PER_INVOCATION,
        websearch_results = SECTION_WEB_SEARCH_RESULTS,
        documentation_found = SECTION_DOCUMENTATION_FOUND,
        library_versions = SECTION_LIBRARY_VERSIONS,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── Exploration prompt scenarios ────────────────────────────────────

    #[test]
    fn exploration_with_repo_dir_includes_repo_reference() {
        // Given: prompt text and a repo path
        // When:  build_exploration_prompt is called
        // Then:  result contains the prompt text, the repo path, and all four section headers
        let prompt = "add OAuth2 to the API";
        let repo = PathBuf::from("/tmp/repo");
        let out = build_exploration_prompt(prompt, Some(&repo));
        assert!(out.contains(prompt), "missing user prompt text");
        assert!(out.contains("/tmp/repo"), "missing repo path");
        assert!(out.contains(SECTION_TOPIC_DECOMPOSITION));
        assert!(out.contains(SECTION_KEY_QUESTIONS));
        assert!(out.contains(SECTION_INITIAL_FINDINGS));
        assert!(out.contains(SECTION_REPO_CONTEXT));
    }

    #[test]
    fn exploration_without_repo_dir_omits_repo_reference() {
        // Given: prompt text and no repo dir
        // When:  build_exploration_prompt is called
        // Then:  result contains the prompt text and the three core headers,
        //        but no Repo Context header and no /tmp path
        let prompt = "evaluate async runtimes";
        let out = build_exploration_prompt(prompt, None);
        assert!(out.contains(prompt));
        assert!(out.contains(SECTION_TOPIC_DECOMPOSITION));
        assert!(out.contains(SECTION_KEY_QUESTIONS));
        assert!(out.contains(SECTION_INITIAL_FINDINGS));
        assert!(
            !out.contains(SECTION_REPO_CONTEXT),
            "repo context header should be omitted"
        );
        assert!(!out.contains("/tmp"), "no repo path should leak in");
    }

    #[test]
    fn exploration_output_format_instruction_present() {
        // Given: any input
        // When:  build_exploration_prompt is called
        // Then:  the three exploration-phase section headers are present verbatim
        let out = build_exploration_prompt("anything", None);
        assert!(out.contains("## Topic Decomposition"));
        assert!(out.contains("## Key Questions"));
        assert!(out.contains("## Initial Findings"));
    }

    #[test]
    fn exploration_preserves_large_prompt_verbatim() {
        // Given: 10 KiB of user prompt text
        // When:  build_exploration_prompt is called
        // Then:  the full text appears in the output
        let big = "A".repeat(10 * 1024);
        let out = build_exploration_prompt(&big, None);
        assert!(
            out.contains(&big),
            "10 KiB user prompt should appear verbatim"
        );
    }

    // ── Deepening prompt scenarios ──────────────────────────────────────

    #[test]
    fn deepening_references_previous_findings() {
        // Given: a unique marker in previous_findings
        // When:  build_deepening_prompt is called
        // Then:  the marker and the deepened-findings header both appear
        let prev = "FOUND-MARKER-123";
        let out = build_deepening_prompt("topic", prev, 2, None);
        assert!(out.contains("FOUND-MARKER-123"));
        assert!(out.contains(SECTION_DEEPENED_FINDINGS));
    }

    #[test]
    fn deepening_iteration_three_asks_for_synthesis() {
        // Given: iteration 3
        // When:  build_deepening_prompt is called
        // Then:  the result contains a synthesis/consolidation hint (case-insensitive)
        let out = build_deepening_prompt("topic", "prev", 3, None);
        let lower = out.to_lowercase();
        assert!(
            lower.contains("synthes") || lower.contains("consolidat"),
            "iteration 3 prompt should invite synthesis/consolidation; got: {}",
            out
        );
    }

    #[test]
    fn deepening_truncates_very_large_findings() {
        // Given: 1 MiB of previous findings
        // When:  build_deepening_prompt is called
        // Then:  the result is < 100 KiB and contains a truncation marker
        let big = "B".repeat(1024 * 1024);
        let out = build_deepening_prompt("topic", &big, 2, None);
        assert!(
            out.len() < 100 * 1024,
            "truncated prompt should fit under 100 KiB, got {}",
            out.len()
        );
        assert!(
            out.contains("[truncated"),
            "expected truncation marker in prompt"
        );
    }

    #[test]
    fn deepening_iteration_one_does_not_panic() {
        // Given: iteration 1 (the caller may legitimately call deepen at iter=1
        //        even though typical flow is exploration→deepen at iter≥2)
        // When:  build_deepening_prompt is called
        // Then:  it returns a non-empty string without panicking
        let out = build_deepening_prompt("topic", "prev", 1, None);
        assert!(!out.is_empty());
    }

    #[test]
    fn deepening_iteration_two_does_not_request_synthesis() {
        // Given: iteration 2 (still mid-loop, synthesis only at iter ≥ 3)
        // When:  build_deepening_prompt is called
        // Then:  result does not contain consolidation directive
        let out = build_deepening_prompt("topic", "prev", 2, None);
        let lower = out.to_lowercase();
        assert!(
            !lower.contains("consolidate and synthesise"),
            "iteration 2 should not request consolidation"
        );
    }

    // ── Repo-context prompt scenarios ───────────────────────────────────

    #[test]
    fn repo_context_includes_repo_path() {
        // Given: a repo path
        // When:  build_repo_context_prompt is called
        // Then:  the path appears in the prompt
        let out = build_repo_context_prompt(Path::new("/proj/x"));
        assert!(out.contains("/proj/x"));
    }

    #[test]
    fn repo_context_includes_all_section_headers() {
        // Given: any repo path
        // When:  build_repo_context_prompt is called
        // Then:  all five repo-context section headers are present
        let out = build_repo_context_prompt(Path::new("/x"));
        assert!(out.contains(SECTION_TECH_STACK));
        assert!(out.contains(SECTION_PROJECT_STRUCTURE));
        assert!(out.contains(SECTION_BUILD_AND_TEST));
        assert!(out.contains(SECTION_EXISTING_PATTERNS));
        assert!(out.contains(SECTION_CONSTRAINTS));
    }

    // ── Web-search prompt scenarios ─────────────────────────────────────

    #[test]
    fn websearch_contains_output_format_headers() {
        // Given: any inputs
        // When:  build_websearch_prompt is called
        // Then:  all three web-search section headers are present verbatim
        let out = build_websearch_prompt("topic", "questions", 1);
        assert!(out.contains(SECTION_WEB_SEARCH_RESULTS));
        assert!(out.contains(SECTION_DOCUMENTATION_FOUND));
        assert!(out.contains(SECTION_LIBRARY_VERSIONS));
    }

    #[test]
    fn websearch_search_index_varies_prompt() {
        // Given: same topic + questions with ≥2 items
        // When:  build_websearch_prompt is called twice with index 1 vs 2
        // Then:  the two prompts differ (question slice differs by index)
        let questions = "- what are the top runtimes?\n- how do they differ in perf?\n- what is ecosystem support?\n- which has best tooling?";
        let a = build_websearch_prompt("topic", questions, 1);
        let b = build_websearch_prompt("topic", questions, 2);
        assert_ne!(a, b, "different search_index values should produce different prompts");
    }

    #[test]
    fn websearch_empty_questions_includes_fallback() {
        // Given: empty questions string
        // When:  build_websearch_prompt("topic", "", 1) is called
        // Then:  the output contains the topic and a broadcast-style fallback instruction
        let out = build_websearch_prompt("topic", "", 1);
        assert!(out.contains("topic"));
        let lower = out.to_lowercase();
        assert!(
            lower.contains("research broadly")
                || lower.contains("search broadly")
                || lower.contains("no specific questions"),
            "empty-questions fallback should mention broad research; got: {}",
            out
        );
    }

    #[test]
    fn websearch_instructs_claude_to_list_urls_with_titles() {
        // Given: any inputs
        // When:  build_websearch_prompt is called
        // Then:  the prompt instructs listing URL + title so M6 can extract references
        let out = build_websearch_prompt("topic", "questions", 1).to_lowercase();
        assert!(out.contains("url") && out.contains("title"),
            "web-search prompt should ask for URL + title pairs; got: {}",
            out);
    }

    #[test]
    fn websearch_prompt_non_empty_for_index_zero_or_one() {
        // Given: search_index 1 (the first index)
        // When:  build_websearch_prompt is called
        // Then:  the output is a non-empty markdown-ish prompt
        let out = build_websearch_prompt("topic", "q", 1);
        assert!(!out.is_empty());
    }

    // ── Purity guards ───────────────────────────────────────────────────

    #[test]
    fn builders_are_pure_no_panic_on_empty_inputs() {
        // Given: empty inputs
        // When:  each builder is invoked
        // Then:  no panic; non-empty result
        let a = build_exploration_prompt("", None);
        let b = build_deepening_prompt("", "", 1, None);
        let c = build_repo_context_prompt(Path::new(""));
        let d = build_websearch_prompt("", "", 1);
        assert!(!a.is_empty() && !b.is_empty() && !c.is_empty() && !d.is_empty());
    }
}

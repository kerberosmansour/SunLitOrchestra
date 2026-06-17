//! Theme-A mid-stream dogfood checkpoint (outcome-first runbook, after M3).
//!
//! The structural tests (M1–M3) prove the Outcome Validation contract is
//! *documented*. This dogfood proves the gate actually *fires*: it mechanically
//! demonstrates that the Outcome Validation decision (Pass 0 + `/slo-retro`
//! refusal) BLOCKS a milestone with an unproven outcome and PASSES a remediated
//! one — the executable "the gate fires" proof the `/slo-critique` CEO-1 + ENG-3
//! findings asked for. Same non-vacuity discipline as the measurement-loop M4
//! failure-bar fixture pair.
//!
//! `gate_blocks()` re-implements the two hardest, mechanically-checkable gate
//! criteria from the §5C / §17 contract:
//!   1. a required Core Capability Regression Matrix row has a BLANK resolution;
//!   2. an Outcome Scenario is mock-only / pending (theatre, tm-outcome-first-abuse-2).

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

fn read(rel: &str) -> String {
    let p = workspace_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("failed to read {}: {}", p.display(), e))
}

/// Return the markdown from `heading` up to the next `#### ` heading (or EOF).
fn section<'a>(md: &'a str, heading: &str) -> Option<&'a str> {
    let start = md.find(heading)?;
    let rest = &md[start + heading.len()..];
    let end = rest.find("\n#### ").unwrap_or(rest.len());
    Some(&rest[..end])
}

/// The Outcome Validation gate decision. `Some(reason)` == milestone BLOCKED.
fn gate_blocks(md: &str) -> Option<String> {
    // Criterion 1 — blank Regression-Matrix resolution on a required row.
    if let Some(sec) = section(md, "#### Core Capability Regression Matrix") {
        for line in sec.lines().filter(|l| l.trim_start().starts_with('|')) {
            let cells: Vec<String> = line.split('|').map(|c| c.trim().to_string()).collect();
            // | Capability | Must still pass | Evidence path | Resolution | -> 6 parts
            if cells.len() < 6 {
                continue;
            }
            let (cap, must, resolution) = (&cells[1], &cells[2], &cells[4]);
            if cap.eq_ignore_ascii_case("capability") || cap.starts_with("---") {
                continue; // header / separator
            }
            if must.eq_ignore_ascii_case("yes") && resolution.is_empty() {
                return Some(format!("blank Regression-Matrix resolution for `{cap}`"));
            }
        }
    }
    // Criterion 2 — mock-only / pending Outcome Scenario (theatre).
    if let Some(sec) = section(md, "#### Outcome Scenarios") {
        let lc = sec.to_lowercase();
        if lc.contains("assert(true)") || lc.contains("mock-only") || lc.contains("pending") {
            return Some("mock-only / pending Outcome Scenario (theatre)".to_string());
        }
    }
    None
}

const BLOCKED: &str = "xtasks/sast-verify/tests/fixtures/outcome_first_dogfood/blocked.md";
const PROVEN: &str = "xtasks/sast-verify/tests/fixtures/outcome_first_dogfood/proven.md";

#[test]
fn gate_blocks_unproven_milestone() {
    let reason = gate_blocks(&read(BLOCKED));
    assert!(
        reason.is_some(),
        "the Outcome Validation gate MUST block an unproven milestone (mock-only outcome + blank regression resolution)"
    );
}

#[test]
fn gate_passes_proven_milestone() {
    let reason = gate_blocks(&read(PROVEN));
    assert!(
        reason.is_none(),
        "the Outcome Validation gate MUST pass a remediated milestone, got blocked: {reason:?}"
    );
}

#[test]
fn gate_is_non_vacuous() {
    // The whole point: bad blocks, good passes — the check distinguishes them.
    assert!(gate_blocks(&read(BLOCKED)).is_some() && gate_blocks(&read(PROVEN)).is_none());
}

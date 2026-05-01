//! Agent-host M5 — structural guards for targeted skill + structural-test
//! cleanup.
//!
//! These tests defend two contracts:
//!
//! 1. **Host-neutral skills must not hide a Claude-only requirement.** The
//!    relevant skills (`/slo-second-opinion`, `/slo-rulegen`) describe logic
//!    that is host-neutral; their copy must use neutral wording where the
//!    behavior does not actually require Claude.
//! 2. **Explicit Claude-only surfaces remain marked as Claude-only.** The
//!    capability matrix at `docs/slo/design/agent-host-capabilities.md` must
//!    keep its honest "Not supported yet" / Claude-only rows. M5 is not a
//!    repo-wide "replace Claude everywhere" pass.

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn skill(name: &str) -> String {
    fs::read_to_string(repo_root().join("skills").join(name).join("SKILL.md"))
        .unwrap_or_else(|e| panic!("{name} SKILL.md missing: {e}"))
}

fn capability_matrix() -> String {
    fs::read_to_string(repo_root().join("docs/slo/design/agent-host-capabilities.md"))
        .expect("agent-host-capabilities.md present")
}

#[test]
fn test_host_neutral_skills_have_no_hidden_claude_requirement() {
    // Given: `/slo-second-opinion` describes a comparison whose host-neutral
    // half is "the current host". Its diff-of-findings table column
    // historically said "Claude said", which baked Claude into the
    // host-neutral logic.
    // When: the skill body is read after the M5 cleanup.
    // Then: the column reads "current host said" (or similar host-neutral
    // wording), and any "asking Claude to pretend" language has been
    // rewritten to "the current host" so a reader in Copilot is not told
    // their host is Claude.
    let so = skill("slo-second-opinion");
    assert!(
        !so.contains("Claude said"),
        "/slo-second-opinion still contains the 'Claude said' diff-table column header — \
         this column describes the current host's review and must read 'current host said' \
         (or similar) so the table is host-neutral"
    );
    let so_lower = so.to_lowercase();
    assert!(
        so_lower.contains("current host said") || so_lower.contains("current host's"),
        "/slo-second-opinion must include host-neutral wording such as 'current host said' \
         in the diff-of-findings table after M5"
    );
    assert!(
        !so.contains("asking Claude to"),
        "/slo-second-opinion still contains 'asking Claude to' phrasing — the silent-fallback \
         rule should refer to the current host, not assume the reader is in Claude"
    );

    // Given: `/slo-rulegen` describes a workflow that takes a bug summary
    // produced by an agent-driven loop. The summary's *origin* does not
    // need to be Claude.
    // When: the skill is read after M5.
    // Then: the description does not single out Claude as the bug-finder.
    let rg = skill("slo-rulegen");
    assert!(
        !rg.contains("Claude-found bug"),
        "/slo-rulegen description still says 'Claude-found bug' — the bug summary can come \
         from any agent-driven workflow, so the wording should be host-neutral"
    );
}

#[test]
fn test_explicit_claude_only_skills_are_marked_in_capability_matrix() {
    // Given: some surfaces remain Claude-only (the optional `sldo-research`
    // batch backend; the live business-judgment runtime harness).
    // When: the capability matrix is read.
    // Then: it still names those surfaces explicitly as Claude-only, and
    // calls out the GitHub Copilot side as "Not supported yet" rather than
    // pretending parity.
    let matrix = capability_matrix();

    assert!(
        matrix.contains("Optional Claude batch backend") || matrix.contains("Claude batch backend"),
        "capability matrix must continue to name `sldo-research` as the optional Claude \
         batch backend so the boundary is honest"
    );
    assert!(
        matrix.contains("Live business judgment runtime harness")
            || matrix.contains("live business judgment runtime"),
        "capability matrix must continue to name the live business judgment runtime as a \
         Claude-only path"
    );
    let lower = matrix.to_lowercase();
    assert!(
        lower.contains("not supported yet"),
        "capability matrix must keep at least one explicit 'Not supported yet' row so the \
         host story stays honest about Copilot-side automation gaps"
    );

    // The matrix must also note `/slo-second-opinion` in the per-skill
    // notes — not as a Claude-only skill, but as one whose host-neutral
    // half is "the current host" (M5 added this clarification so future
    // readers understand the skill's design).
    assert!(
        matrix.contains("/slo-second-opinion"),
        "capability matrix must record a per-skill note for /slo-second-opinion after M5"
    );
}

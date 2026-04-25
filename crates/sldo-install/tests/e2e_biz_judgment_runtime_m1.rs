//! M1 — biz-pack judgment runtime harness, single-fixture proof.
//!
//! This file is the first runtime user of `tests/common/judgment_runtime.rs`.
//! It runs ONE fixture (`ir35-genuine-contractor.md`) end-to-end:
//!   1. parse the fixture
//!   2. build a tempdir with skills + references symlinked, HOME redirected
//!   3. invoke `claude -p` with a budget cap
//!   4. discover the written artifact
//!   5. assert frontmatter matches the fixture's expectations
//!
//! Default `cargo test -p sldo-install` does NOT run this — the test is
//! `#[ignore]` AND env-gated by `BIZ_JUDGMENT_RUNTIME_LIVE=1`. Reason: real
//! `claude` calls cost real money + require network.
//!
//! To run live:
//!   BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install \
//!       --test e2e_biz_judgment_runtime_m1 -- --ignored

mod common;

use std::time::Duration;

use common::judgment_runtime::{
    claude_available, discover_artifact, invoke_claude, repo_root, skip_if_not_live,
    JudgmentFixture, TempRepo,
};

#[test]
#[ignore]
fn runtime_harness_green_on_ir35_genuine_contractor() {
    if skip_if_not_live() {
        return;
    }
    if let Err(e) = claude_available() {
        panic!(
            "claude binary not invocable: {e}\n\
             Override the binary path with BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN=/path/to/claude"
        );
    }

    let repo = repo_root();
    let fixture_path = repo
        .join("references/biz/judgment-fixtures/slo-legal/ir35-genuine-contractor.md");
    let fixture = JudgmentFixture::parse(&fixture_path).expect("parse fixture");

    let temp = TempRepo::build(&repo).expect("build tempdir");
    let out = invoke_claude(&temp, &fixture.founder_prompt, 0.50, Duration::from_secs(180))
        .expect("invoke claude");

    if !out.exit_status.success() {
        panic!(
            "claude exited non-zero ({}). stdout: {}\nstderr: {}",
            out.exit_status, out.stdout, out.stderr
        );
    }

    let artifact = match discover_artifact(&temp).expect("discover artifact") {
        Some(a) => a,
        None => panic!(
            "no artifact written under docs/biz/ or docs/biz-public/ for happy-path fixture {}\n\
             claude stdout: {}\n\
             claude stderr: {}",
            fixture.frontmatter.name, out.stdout, out.stderr
        ),
    };

    // Assertions on the discovered artifact's frontmatter.
    let fm = &artifact.frontmatter;
    assert_eq!(
        fm.get("tier").map(String::as_str),
        Some("confidential"),
        "ir35-genuine-contractor expected tier:confidential, got {:?}\n\
         artifact: {}",
        fm.get("tier"),
        artifact.path.display()
    );
    assert_eq!(
        fm.get("triage_gate_passed").map(String::as_str),
        Some("true"),
        "ir35-genuine-contractor expected triage_gate_passed:true, got {:?}",
        fm.get("triage_gate_passed")
    );

    // gates_fired should be `[]` or absent. Tolerate both: the artifact-schema
    // says it's required only when triage_gate_passed:false.
    if let Some(gates) = fm.get("gates_fired") {
        assert!(
            gates.trim().is_empty()
                || gates.trim() == "[]"
                || gates.contains("none"),
            "ir35-genuine-contractor expected empty gates_fired, got `{gates}`"
        );
    }

    eprintln!(
        "M1 harness: passed. artifact={} tier={} triage_gate_passed={}",
        artifact.path.display(),
        fm.get("tier").cloned().unwrap_or_default(),
        fm.get("triage_gate_passed").cloned().unwrap_or_default()
    );
}

//! `gate` subcommand — composes validate → test → check-coverage → check-clean.
//!
//! THE single deterministic entry point that `/slo-rulegen` and `/slo-ruleverify`
//! shell out to before authorising any rule write. Short-circuits on first
//! failure; exit code propagates from the failing sub-step.
//!
//! Per the runbook M1 "Important design rule": both skills MUST shell out to
//! `gate`, NOT directly to `validate` / `test` / `check-coverage` / `check-clean`.
//! Bypassing `gate` is a P1 finding for `/slo-critique`.

use anyhow::Result;
use std::path::Path;

use crate::{check_clean, check_coverage, test_cmd, validate, GlobalOpts};

pub fn run(
    rule_path: &Path,
    references_dir: Option<&Path>,
    clean_dir: Option<&Path>,
    opts: &GlobalOpts,
) -> Result<i32> {
    // Step 1: validate (yaml + semgrep --validate)
    let code = validate::run(rule_path, opts)?;
    if code != 0 {
        emit(
            opts,
            "fail",
            "validate",
            &format!("first failing sub-step: validate (exit {code})"),
        );
        return Ok(code);
    }

    // Step 2: test (paired fixture fire/silent — runs --validate again internally,
    // but that's idempotent and cheap; the synthesis design rule "validate
    // before test" is preserved either way).
    let code = test_cmd::run(rule_path, opts)?;
    if code != 0 {
        emit(
            opts,
            "fail",
            "test",
            &format!("first failing sub-step: test (exit {code})"),
        );
        return Ok(code);
    }

    // Step 3: check-coverage (per-CWE arm minimum + ceiling)
    let code = check_coverage::run(rule_path, references_dir, opts)?;
    if code != 0 {
        emit(
            opts,
            "fail",
            "check-coverage",
            &format!("first failing sub-step: check-coverage (exit {code})"),
        );
        return Ok(code);
    }

    // Step 4: check-clean (zero-FP on known-clean subset)
    let code = check_clean::run(rule_path, clean_dir, opts)?;
    if code != 0 {
        emit(
            opts,
            "fail",
            "check-clean",
            &format!("first failing sub-step: check-clean (exit {code})"),
        );
        return Ok(code);
    }

    emit(opts, "pass", "all", "validate + test + check-coverage + check-clean all green");
    Ok(0)
}

fn emit(opts: &GlobalOpts, status: &str, label: &str, detail: &str) {
    if opts.json {
        let v = serde_json::json!({
            "subcommand": "gate",
            "status": status,
            "label": label,
            "detail": detail,
        });
        println!("{v}");
    } else if status == "pass" {
        println!("gate: PASS — {detail}");
    } else {
        eprintln!("gate: {status} (failed at {label}): {detail}");
    }
}

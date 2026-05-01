//! `test` subcommand — wraps `semgrep --validate` then `semgrep --test`.
//!
//! Per Semgrep issue #10319: validate before test (synthesis design rule).

use anyhow::Result;
use std::path::Path;

use crate::{semgrep_runner, validate, GlobalOpts};

pub fn run(rule_path: &Path, opts: &GlobalOpts) -> Result<i32> {
    let validate_code = validate::run(rule_path, opts)?;
    if validate_code != 0 {
        emit(
            opts,
            "fail",
            "validate_failed_before_test",
            &format!("validate exit {validate_code}; refusing to run --test on invalid rule per semgrep issue #10319"),
        );
        return Ok(validate_code);
    }

    if semgrep_runner::paired_fixture_for(rule_path).is_none() {
        emit(
            opts,
            "fail",
            "paired_fixture_missing",
            &format!(
                "paired fixture <rule-id>.rs not found next to {} (Semgrep upstream convention)",
                rule_path.display()
            ),
        );
        return Ok(5);
    }

    let path_str = rule_path.to_string_lossy();
    let args = [
        "--test",
        "--json",
        "--config",
        path_str.as_ref(),
        path_str.as_ref(),
    ];
    let run = match semgrep_runner::run(&args, opts) {
        Ok(r) => r,
        Err(e) => {
            emit(opts, "fail", "semgrep_invocation_failed", &format!("{e:#}"));
            return Ok(4);
        }
    };

    match run.exit_code {
        0 => {
            emit(opts, "pass", "fire_on_bad_silent_on_good", "");
            Ok(0)
        }
        1 => {
            emit(opts, "fail", "test_assertion_failed", &run.stderr);
            Ok(2)
        }
        other => {
            emit(
                opts,
                "fail",
                "unexpected_test_exit",
                &format!("exit {other}: {}", run.stderr),
            );
            Ok(2)
        }
    }
}

fn emit(opts: &GlobalOpts, status: &str, label: &str, detail: &str) {
    if opts.json {
        let v = serde_json::json!({
            "subcommand": "test",
            "status": status,
            "label": label,
            "detail": detail,
        });
        println!("{v}");
    } else if status == "pass" {
        println!("test: fire-on-bad / silent-on-good");
    } else {
        eprintln!("test: {status} ({label}): {detail}");
    }
}

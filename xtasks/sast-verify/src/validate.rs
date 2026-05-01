//! `validate` subcommand — wraps `semgrep --validate <rule-path>`.
//!
//! Exit codes (per interfaces §1):
//! - `0` valid
//! - `2` bad YAML / unknown field
//! - `3` unknown rule shape
//! - `4` semgrep CLI not on PATH

use anyhow::Result;
use std::path::Path;

use crate::{semgrep_runner, yaml_schema, GlobalOpts};

pub fn run(rule_path: &Path, opts: &GlobalOpts) -> Result<i32> {
    if let Err(parse_err) = yaml_schema::parse_file(rule_path) {
        emit(
            opts,
            "fail",
            "yaml_parse_or_unknown_field",
            &format!("{parse_err:#}"),
        );
        return Ok(2);
    }

    if which::which("semgrep").is_err() && opts.semgrep_bin.is_none() {
        emit(opts, "fail", "semgrep_not_found", "semgrep CLI not on PATH");
        return Ok(4);
    }

    let path_str = rule_path.to_string_lossy();
    let args = ["--validate", "--json", "--config", path_str.as_ref()];
    let run = match semgrep_runner::run(&args, opts) {
        Ok(r) => r,
        Err(e) => {
            emit(opts, "fail", "semgrep_invocation_failed", &format!("{e:#}"));
            return Ok(4);
        }
    };

    match run.exit_code {
        0 => {
            emit(opts, "pass", "valid", "");
            Ok(0)
        }
        5 => {
            emit(opts, "fail", "bad_config", &run.stderr);
            Ok(2)
        }
        7 | 4 => {
            emit(opts, "fail", "invalid_rule_shape", &run.stderr);
            Ok(3)
        }
        other => {
            emit(
                opts,
                "fail",
                "unknown_semgrep_exit",
                &format!("exit {other}: {}", run.stderr),
            );
            Ok(2)
        }
    }
}

fn emit(opts: &GlobalOpts, status: &str, label: &str, detail: &str) {
    if opts.json {
        let v = serde_json::json!({
            "subcommand": "validate",
            "status": status,
            "label": label,
            "detail": detail,
        });
        println!("{v}");
    } else if status == "pass" {
        println!("validate: valid");
    } else {
        eprintln!("validate: {status} ({label}): {detail}");
    }
}

//! `check-coverage` subcommand — variation-coverage gate.
//!
//! Asserts pattern-either arm count is in [minimum_pattern_either_arms, 25].
//! Minimum N comes from `references/sast/variations/cwe-<NNN>.md` frontmatter.

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::{yaml_schema, GlobalOpts};

const DEFAULT_CEILING: usize = 25;

pub fn run(rule_path: &Path, references_dir: Option<&Path>, opts: &GlobalOpts) -> Result<i32> {
    let rule_file = match yaml_schema::parse_file(rule_path) {
        Ok(rf) => rf,
        Err(e) => {
            emit(opts, "fail", "yaml_parse_failed", &format!("{e:#}"));
            return Ok(2);
        }
    };

    for rule in &rule_file.rules {
        let cwe = match rule.cwe_id() {
            Some(c) => c,
            None => {
                emit(
                    opts,
                    "fail",
                    "missing_cwe_metadata",
                    &format!("rule `{}` has no metadata.cwe field", rule.id),
                );
                return Ok(6);
            }
        };

        let refs_dir = resolve_references_dir(references_dir);
        let variation_path = variation_file_for(&refs_dir, &cwe);

        let min_arms = match read_minimum_arms(&variation_path) {
            Ok(m) => m,
            Err(e) => {
                emit(
                    opts,
                    "fail",
                    "variation_template_unreadable",
                    &format!(
                        "could not read minimum-arms for {cwe} from {}: {e:#}",
                        variation_path.display()
                    ),
                );
                return Ok(6);
            }
        };

        let arms = rule.pattern_either_arm_count();
        if arms == 0 && min_arms > 0 {
            emit(
                opts,
                "fail",
                "missing_pattern_either",
                &format!(
                    "rule `{}` ({}) requires {min_arms} pattern-either arms (per {}); got single-pattern rule",
                    rule.id, cwe, variation_path.display()
                ),
            );
            return Ok(3);
        }
        if arms < min_arms {
            emit(
                opts,
                "fail",
                "below_minimum_arms",
                &format!(
                    "rule `{}` ({}) has {arms} pattern-either arms; minimum is {min_arms} per {}",
                    rule.id,
                    cwe,
                    variation_path.display()
                ),
            );
            return Ok(2);
        }
        if arms > DEFAULT_CEILING {
            emit(
                opts,
                "fail",
                "above_ceiling_arms",
                &format!(
                    "rule `{}` ({}) has {arms} pattern-either arms; ceiling is {DEFAULT_CEILING} (DoS-via-pattern-explosion mitigation)",
                    rule.id, cwe
                ),
            );
            return Ok(7);
        }
    }

    emit(opts, "pass", "within_bounds", "");
    Ok(0)
}

fn resolve_references_dir(override_dir: Option<&Path>) -> PathBuf {
    if let Some(d) = override_dir {
        return d.to_path_buf();
    }
    let cwd = std::env::current_dir().unwrap_or_default();
    let mut candidate = cwd.clone();
    for _ in 0..6 {
        let check = candidate.join("references").join("sast");
        if check.is_dir() {
            return check;
        }
        if !candidate.pop() {
            break;
        }
    }
    cwd.join("references").join("sast")
}

fn variation_file_for(refs_dir: &Path, cwe: &str) -> PathBuf {
    let lower = cwe.to_lowercase();
    refs_dir.join("variations").join(format!("{lower}.md"))
}

fn read_minimum_arms(path: &Path) -> Result<usize> {
    let content = std::fs::read_to_string(path)?;
    let mut in_fm = false;
    let mut fm_buf = String::new();
    for line in content.lines() {
        if line.trim() == "---" {
            if in_fm {
                break;
            }
            in_fm = true;
            continue;
        }
        if in_fm {
            fm_buf.push_str(line);
            fm_buf.push('\n');
        }
    }
    if fm_buf.is_empty() {
        return Err(anyhow::anyhow!(
            "no frontmatter block found in {}",
            path.display()
        ));
    }
    let fm: serde_yaml_ng::Value = serde_yaml_ng::from_str(&fm_buf)?;
    let m = fm
        .get("minimum_pattern_either_arms")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "frontmatter at {} missing `minimum_pattern_either_arms:` key (integer)",
                path.display()
            )
        })?;
    Ok(m as usize)
}

fn emit(opts: &GlobalOpts, status: &str, label: &str, detail: &str) {
    if opts.json {
        let v = serde_json::json!({
            "subcommand": "check-coverage",
            "status": status,
            "label": label,
            "detail": detail,
        });
        println!("{v}");
    } else if status == "pass" {
        println!("check-coverage: within bounds");
    } else {
        eprintln!("check-coverage: {status} ({label}): {detail}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn read_minimum_arms_from_frontmatter() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("cwe-755.md");
        std::fs::write(
            &p,
            "---\ncwe: \"CWE-755\"\nminimum_pattern_either_arms: 4\nsink_shapes:\n  - unwrap\n  - expect\n---\n\n# Body\n",
        )
        .unwrap();
        assert_eq!(read_minimum_arms(&p).unwrap(), 4);
    }

    #[test]
    fn read_minimum_arms_missing_key_errors() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("cwe-foo.md");
        std::fs::write(&p, "---\ncwe: \"CWE-FOO\"\n---\n").unwrap();
        let err = read_minimum_arms(&p).unwrap_err();
        assert!(format!("{err:#}").contains("minimum_pattern_either_arms"));
    }
}

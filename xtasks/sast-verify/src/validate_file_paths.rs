//! `validate-file-paths` subcommand — repo-traversal-safe path checker.
//!
//! Validates a comma-separated list of file paths against a repo root.
//! Per the M2.5 contract: each path MUST canonicalize and the canonicalized
//! result MUST live under the repo root. Rejects:
//! - absolute paths (input must be repo-relative)
//! - paths containing `..` segments
//! - symlinks whose target falls outside the repo (caught by canonicalize +
//!   `starts_with(repo_root)`)
//! - paths that do not exist (we cannot canonicalize a non-existent path)
//!
//! Sole purpose: shell-out target for `/slo-rulegen --extend`'s prose
//! validation contract (per `skills/slo-rulegen/SKILL.md` step 1 and
//! `references/sast/prompts/extend.md` "Validate inputs" section). Moving
//! the check into the xtask makes it deterministic + testable + reusable
//! by other tools that need the same guard (CI, future skills).
//!
//! Threat-model citation: `tm-sast-rulegen-skill-pack-sec-3`
//! (`--file-paths` traversal); the helper closes the gap between the
//! skill's prose-driven validation and a Rust-implemented gate.
//!
//! Exit codes (per `docs/slo/design/sast-rulegen-skill-pack-interfaces.md` §1):
//! - `0` — all paths valid
//! - `4` — at least one path rejected (text + JSON output enumerates each)

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::GlobalOpts;

#[derive(Debug)]
pub struct PathVerdict {
    pub input: String,
    pub canonical: Option<PathBuf>,
    pub reason: Option<String>,
}

impl PathVerdict {
    pub fn ok(input: String, canonical: PathBuf) -> Self {
        Self {
            input,
            canonical: Some(canonical),
            reason: None,
        }
    }
    pub fn reject(input: String, reason: String) -> Self {
        Self {
            input,
            canonical: None,
            reason: Some(reason),
        }
    }
    pub fn is_ok(&self) -> bool {
        self.reason.is_none()
    }
}

pub fn run(csv: &str, repo_dir: Option<&Path>, opts: &GlobalOpts) -> Result<i32> {
    let repo_root = resolve_repo_root(repo_dir)?;
    let verdicts = validate_csv(csv, &repo_root);

    if opts.json {
        let validated: Vec<String> = verdicts
            .iter()
            .filter_map(|v| v.canonical.as_ref().map(|c| c.display().to_string()))
            .collect();
        let rejected: Vec<serde_json::Value> = verdicts
            .iter()
            .filter(|v| !v.is_ok())
            .map(|v| serde_json::json!({ "path": v.input, "reason": v.reason }))
            .collect();
        let out = serde_json::json!({
            "subcommand": "validate-file-paths",
            "repo_root": repo_root.display().to_string(),
            "validated": validated,
            "rejected": rejected,
        });
        println!("{out}");
    } else {
        for v in &verdicts {
            if let Some(c) = &v.canonical {
                println!("OK {} -> {}", v.input, c.display());
            } else {
                println!(
                    "REJECT {}: {}",
                    v.input,
                    v.reason.as_deref().unwrap_or("(no reason)")
                );
            }
        }
    }

    if verdicts.iter().any(|v| !v.is_ok()) {
        Ok(4)
    } else {
        Ok(0)
    }
}

fn resolve_repo_root(repo_dir: Option<&Path>) -> Result<PathBuf> {
    let raw = match repo_dir {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().context("cannot read current_dir")?,
    };
    raw.canonicalize()
        .with_context(|| format!("cannot canonicalize repo root {}", raw.display()))
}

pub fn validate_csv(csv: &str, repo_root: &Path) -> Vec<PathVerdict> {
    csv.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(|p| validate_one(p, repo_root))
        .collect()
}

fn validate_one(input: &str, repo_root: &Path) -> PathVerdict {
    // Quick syntactic checks first — cheaper than canonicalize and produce
    // clearer rejection reasons than "starts_with failed".
    let p = Path::new(input);
    if p.is_absolute() {
        return PathVerdict::reject(
            input.to_string(),
            "absolute paths are forbidden — input must be repo-relative".to_string(),
        );
    }
    if input.split('/').any(|seg| seg == "..") || input.split('\\').any(|seg| seg == "..") {
        return PathVerdict::reject(
            input.to_string(),
            "path contains `..` segment — refusing path-traversal-shaped input".to_string(),
        );
    }

    let joined = repo_root.join(input);
    let canonical = match joined.canonicalize() {
        Ok(c) => c,
        Err(e) => {
            return PathVerdict::reject(
                input.to_string(),
                format!("cannot canonicalize ({e}) — does the file exist?"),
            );
        }
    };

    if !canonical.starts_with(repo_root) {
        return PathVerdict::reject(
            input.to_string(),
            format!(
                "canonical path {} escapes repo root {} (likely a symlink target outside the repo)",
                canonical.display(),
                repo_root.display()
            ),
        );
    }
    PathVerdict::ok(input.to_string(), canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_repo(files: &[&str]) -> TempDir {
        let td = TempDir::new().unwrap();
        for f in files {
            let path = td.path().join(f);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(&path, "").unwrap();
        }
        td
    }

    #[test]
    fn validates_repo_relative_existing_file() {
        let td = make_repo(&["src/lib.rs"]);
        let v = validate_one("src/lib.rs", &td.path().canonicalize().unwrap());
        assert!(v.is_ok(), "got reason: {:?}", v.reason);
    }

    #[test]
    fn rejects_absolute_path() {
        let td = make_repo(&["src/lib.rs"]);
        let v = validate_one("/etc/passwd", &td.path().canonicalize().unwrap());
        assert!(!v.is_ok());
        assert!(v.reason.unwrap().contains("absolute"));
    }

    #[test]
    fn rejects_dotdot_segment() {
        let td = make_repo(&["src/lib.rs"]);
        let v = validate_one("../etc/passwd", &td.path().canonicalize().unwrap());
        assert!(!v.is_ok());
        assert!(v.reason.unwrap().contains("`..`"));
    }

    #[test]
    fn rejects_dotdot_in_middle() {
        let td = make_repo(&["src/lib.rs"]);
        let v = validate_one("src/../../etc/passwd", &td.path().canonicalize().unwrap());
        assert!(!v.is_ok());
    }

    #[test]
    fn rejects_nonexistent_path() {
        let td = make_repo(&["src/lib.rs"]);
        let v = validate_one("does-not-exist.rs", &td.path().canonicalize().unwrap());
        assert!(!v.is_ok());
        assert!(v.reason.unwrap().contains("canonicalize"));
    }

    #[test]
    fn rejects_symlink_escape() {
        // Symlink inside the repo pointing at a path outside the repo.
        let td_outside = TempDir::new().unwrap();
        std::fs::write(td_outside.path().join("secret"), "").unwrap();
        let td = make_repo(&["src/lib.rs"]);
        let link = td.path().join("escape.rs");
        std::os::unix::fs::symlink(td_outside.path().join("secret"), &link).unwrap();

        let v = validate_one("escape.rs", &td.path().canonicalize().unwrap());
        assert!(!v.is_ok(), "symlink escape should be rejected");
        assert!(v.reason.unwrap().contains("escapes repo root"));
    }

    #[test]
    fn validate_csv_returns_one_verdict_per_path() {
        let td = make_repo(&["src/lib.rs", "src/main.rs"]);
        let root = td.path().canonicalize().unwrap();
        let vs = validate_csv("src/lib.rs, src/main.rs, /etc/passwd", &root);
        assert_eq!(vs.len(), 3);
        assert!(vs[0].is_ok());
        assert!(vs[1].is_ok());
        assert!(!vs[2].is_ok());
    }

    #[test]
    fn validate_csv_skips_empty_segments() {
        let td = make_repo(&["src/lib.rs"]);
        let root = td.path().canonicalize().unwrap();
        let vs = validate_csv(",src/lib.rs,, ", &root);
        assert_eq!(vs.len(), 1, "empty / whitespace segments are skipped");
        assert!(vs[0].is_ok());
    }
}

//! `detect-tier` subcommand — confidentiality-tier auto-detection from `git remote`.
//!
//! Per the M2 contract + `/slo-critique` eng-4: the helper handles HTTPS,
//! SSH, no-remote, multi-remote, and non-GitHub URL shapes. **Default-deny**:
//! returns `Confidential` on any parse failure or unknown host so the corpus
//! defaults to `.gitignore`'d when the tier cannot be confidently determined.
//!
//! Returns `Public` ONLY when the remote URL is unambiguously a public GitHub
//! repo AND the user has passed `--target-tier public` explicitly elsewhere
//! (this helper is one input; the skill is the authoritative tier picker).
//!
//! For v1 the helper conservatively returns `Confidential` ALWAYS unless the
//! caller passes an explicit `--target-tier public` upstream — that is, the
//! auto-detection is the safe-default informer, not the public-tier authority.
//!
//! Exit codes:
//! - `0` always (the verdict is on stdout)

use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::GlobalOpts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Confidential,
    Public,
}

impl Tier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tier::Confidential => "Confidential",
            Tier::Public => "Public",
        }
    }
}

pub fn run(repo_dir: Option<&Path>, opts: &GlobalOpts) -> Result<i32> {
    let tier = detect_tier(repo_dir);
    if opts.json {
        let v = serde_json::json!({
            "subcommand": "detect-tier",
            "tier": tier.as_str(),
        });
        println!("{v}");
    } else {
        println!("{}", tier.as_str());
    }
    Ok(0)
}

/// Default-deny tier detection. Returns `Confidential` on every failure mode:
/// no remote, parse failure, ambiguous URL, multiple remotes, non-github.com host.
///
/// For v1 the helper is conservative-by-design — `--target-tier public` is the
/// only way the skill writes to a tracked-and-labelled corpus tier.
pub fn detect_tier(repo_dir: Option<&Path>) -> Tier {
    let remote_url = match read_remote_url(repo_dir) {
        Some(u) => u,
        None => return Tier::Confidential,
    };

    // For v1: every successfully-parsed remote returns Confidential. The
    // `--target-tier public` opt-in is the only authoritative way to land in
    // the public tier. This is per /slo-critique eng-4: "auto-detection is for
    // the safe (confidential) default only."
    let _ = parse_remote_host(&remote_url);
    Tier::Confidential
}

fn read_remote_url(repo_dir: Option<&Path>) -> Option<String> {
    let mut cmd = Command::new("git");
    cmd.arg("remote").arg("get-url").arg("origin");
    if let Some(d) = repo_dir {
        cmd.current_dir(d);
    }
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Parse `git@github.com:user/repo.git` (SSH) or `https://github.com/user/repo.git`
/// (HTTPS) into the bare host. Returns None for any unparseable shape.
pub fn parse_remote_host(url: &str) -> Option<String> {
    if url.starts_with("git@") {
        // git@host:user/repo.git
        let after_at = url.strip_prefix("git@")?;
        let host = after_at.split(':').next()?;
        return Some(host.to_string());
    }
    if let Some(rest) = url.strip_prefix("https://") {
        let host = rest.split('/').next()?;
        return Some(host.to_string());
    }
    if let Some(rest) = url.strip_prefix("http://") {
        let host = rest.split('/').next()?;
        return Some(host.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ssh_url() {
        assert_eq!(
            parse_remote_host("git@github.com:foo/bar.git").as_deref(),
            Some("github.com")
        );
    }

    #[test]
    fn parse_https_url() {
        assert_eq!(
            parse_remote_host("https://github.com/foo/bar.git").as_deref(),
            Some("github.com")
        );
    }

    #[test]
    fn parse_unknown_scheme_returns_none() {
        assert_eq!(parse_remote_host("ftp://something/else"), None);
        assert_eq!(parse_remote_host("not-a-url"), None);
    }

    #[test]
    fn detect_tier_no_remote_is_confidential() {
        // Use a TempDir which is not a git repo; `git remote get-url origin`
        // will fail and we should default to Confidential.
        let dir = tempfile::TempDir::new().unwrap();
        assert_eq!(detect_tier(Some(dir.path())), Tier::Confidential);
    }
}

//! Path resolution for the installer.
//!
//! Centralizes where the manifest lives and where skills are installed,
//! so tests can override HOME or CWD without touching install logic.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::host::Host;

/// Resolve the user's home directory, honoring `$HOME` (which `tempfile`-backed
/// tests override). Falls back to `dirs`-style fallback not implemented here —
/// we require `$HOME` to be set because every test and production environment
/// we target has one.
pub fn home_dir() -> Result<PathBuf> {
    let home = std::env::var_os("HOME").context(
        "$HOME is not set. sldo-install cannot locate the user home directory. \
         Set HOME or pass --skills-dir with an explicit path.",
    )?;
    Ok(PathBuf::from(home))
}

/// Where global installs land: `~/.claude/skills/` or `~/.copilot/skills/`.
pub fn global_skills_root(home: &Path, host: Host) -> PathBuf {
    home.join(host.descriptor().config_dir).join("skills")
}

/// Where local installs land, relative to the given CWD.
pub fn local_skills_root(cwd: &Path, host: Host) -> PathBuf {
    cwd.join(host.descriptor().config_dir).join("skills")
}

/// Where the global manifest lives: `~/.sldo/install.toml`.
pub fn global_manifest_path(home: &Path) -> PathBuf {
    home.join(".sldo").join("install.toml")
}

/// Where the local manifest lives: `<cwd>/<host>/slo-install.toml`.
pub fn local_manifest_path(cwd: &Path, host: Host) -> PathBuf {
    cwd.join(host.descriptor().config_dir).join("slo-install.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_skills_root_under_home() {
        let home = Path::new("/tmp/fakehome");
        assert_eq!(
            global_skills_root(home, Host::ClaudeCode),
            PathBuf::from("/tmp/fakehome/.claude/skills")
        );
    }

    #[test]
    fn local_skills_root_under_cwd() {
        let cwd = Path::new("/tmp/fake-repo");
        assert_eq!(
            local_skills_root(cwd, Host::ClaudeCode),
            PathBuf::from("/tmp/fake-repo/.claude/skills")
        );
    }

    #[test]
    fn github_copilot_roots_use_copilot_config_dir() {
        let base = Path::new("/tmp/fakehome");
        assert_eq!(
            global_skills_root(base, Host::GithubCopilot),
            PathBuf::from("/tmp/fakehome/.copilot/skills")
        );

        let cwd = Path::new("/tmp/fake-repo");
        assert_eq!(
            local_manifest_path(cwd, Host::GithubCopilot),
            PathBuf::from("/tmp/fake-repo/.copilot/slo-install.toml")
        );
    }
}

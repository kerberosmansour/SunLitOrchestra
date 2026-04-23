//! Path resolution for the installer.
//!
//! Centralizes where the manifest lives and where skills are installed,
//! so tests can override HOME or CWD without touching install logic.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

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

/// Where global installs land: `~/.claude/skills/`.
pub fn global_skills_root(home: &Path) -> PathBuf {
    home.join(".claude").join("skills")
}

/// Where local installs land, relative to the given CWD: `<cwd>/.claude/skills/`.
pub fn local_skills_root(cwd: &Path) -> PathBuf {
    cwd.join(".claude").join("skills")
}

/// Where the global manifest lives: `~/.sldo/install.toml`.
pub fn global_manifest_path(home: &Path) -> PathBuf {
    home.join(".sldo").join("install.toml")
}

/// Where the local manifest lives: `<cwd>/.claude/slo-install.toml`.
pub fn local_manifest_path(cwd: &Path) -> PathBuf {
    cwd.join(".claude").join("slo-install.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_skills_root_under_home() {
        let home = Path::new("/tmp/fakehome");
        assert_eq!(
            global_skills_root(home),
            PathBuf::from("/tmp/fakehome/.claude/skills")
        );
    }

    #[test]
    fn local_skills_root_under_cwd() {
        let cwd = Path::new("/tmp/fake-repo");
        assert_eq!(
            local_skills_root(cwd),
            PathBuf::from("/tmp/fake-repo/.claude/skills")
        );
    }
}

//! Path resolution for the installer.
//!
//! Centralizes where the manifest lives and where skills are installed,
//! so tests can override HOME or CWD without touching install logic.

use anyhow::{bail, Result};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::host::Host;

/// Resolve the user's home directory.
///
/// Unix-like shells usually provide `$HOME`. Native Windows shells usually
/// provide `%USERPROFILE%`, or `%HOMEDRIVE%` + `%HOMEPATH%`.
pub fn home_dir() -> Result<PathBuf> {
    let windows_userprofile = if cfg!(windows) {
        std::env::var_os("USERPROFILE")
    } else {
        None
    };
    let windows_homedrive = if cfg!(windows) {
        std::env::var_os("HOMEDRIVE")
    } else {
        None
    };
    let windows_homepath = if cfg!(windows) {
        std::env::var_os("HOMEPATH")
    } else {
        None
    };

    if let Some(home) = resolve_home_from_env(
        std::env::var_os("HOME"),
        windows_userprofile,
        windows_homedrive,
        windows_homepath,
    ) {
        return Ok(home);
    }

    bail!(
        "Could not locate the user home directory. Set HOME on Unix/macOS, \
         or USERPROFILE (or HOMEDRIVE + HOMEPATH) on Windows."
    )
}

fn resolve_home_from_env(
    home: Option<OsString>,
    userprofile: Option<OsString>,
    homedrive: Option<OsString>,
    homepath: Option<OsString>,
) -> Option<PathBuf> {
    if let Some(home) = non_empty_path(home) {
        return Some(home);
    }
    if let Some(userprofile) = non_empty_path(userprofile) {
        return Some(userprofile);
    }

    let drive = homedrive.and_then(non_empty_os_string)?;
    let path = homepath.and_then(non_empty_os_string)?;
    let mut combined = drive;
    combined.push(path);
    Some(PathBuf::from(combined))
}

fn non_empty_path(value: Option<OsString>) -> Option<PathBuf> {
    value.and_then(non_empty_os_string).map(PathBuf::from)
}

fn non_empty_os_string(value: OsString) -> Option<OsString> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Where global installs land, e.g. `~/.claude/skills/`,
/// `~/.copilot/skills/`, or `~/.codex/skills/`.
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
    cwd.join(host.descriptor().config_dir)
        .join("slo-install.toml")
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

    #[test]
    fn codex_roots_use_codex_config_dir() {
        let base = Path::new("/tmp/fakehome");
        assert_eq!(
            global_skills_root(base, Host::Codex),
            PathBuf::from("/tmp/fakehome/.codex/skills")
        );

        let cwd = Path::new("/tmp/fake-repo");
        assert_eq!(
            local_manifest_path(cwd, Host::Codex),
            PathBuf::from("/tmp/fake-repo/.codex/slo-install.toml")
        );
    }

    #[test]
    fn home_resolution_prefers_home_when_present() {
        assert_eq!(
            resolve_home_from_env(
                Some(OsString::from("/home/slo")),
                Some(OsString::from(r"C:\Users\slo")),
                None,
                None,
            ),
            Some(PathBuf::from("/home/slo"))
        );
    }

    #[test]
    fn home_resolution_accepts_windows_userprofile() {
        assert_eq!(
            resolve_home_from_env(None, Some(OsString::from(r"C:\Users\slo")), None, None),
            Some(PathBuf::from(r"C:\Users\slo"))
        );
    }

    #[test]
    fn home_resolution_accepts_windows_drive_and_path() {
        assert_eq!(
            resolve_home_from_env(
                None,
                None,
                Some(OsString::from("C:")),
                Some(OsString::from(r"\Users\slo")),
            ),
            Some(PathBuf::from(r"C:\Users\slo"))
        );
    }

    #[test]
    fn home_resolution_rejects_empty_values() {
        assert_eq!(
            resolve_home_from_env(
                Some(OsString::new()),
                Some(OsString::new()),
                Some(OsString::from("C:")),
                Some(OsString::new()),
            ),
            None
        );
    }
}

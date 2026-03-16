//! Pre-flight validation checks.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::git;

/// Check that the `copilot` CLI is installed and return its path.
pub fn check_copilot_installed() -> Result<PathBuf> {
    which::which("copilot").context(
        "GitHub Copilot CLI ('copilot') not found on PATH. \
         Install it: https://docs.github.com/en/copilot/concepts/agents/copilot-cli",
    )
}

/// Check that a file exists at the given path.
pub fn check_file_exists(path: &Path, label: &str) -> Result<()> {
    if !path.exists() {
        bail!("{} not found: {}", label, path.display());
    }
    Ok(())
}

/// Verify git safety: the repo is a git repo and we are not on a protected branch.
/// Returns the current branch name on success.
pub fn check_git_safety(repo_dir: &Path) -> Result<String> {
    if !git::is_git_repo(repo_dir) {
        bail!("Not a git repository: {}", repo_dir.display());
    }
    let branch = git::current_branch(repo_dir)?;
    if git::is_protected_branch(&branch) {
        bail!(
            "You are on '{}'. Refusing to run on a protected branch. \
             Please create a feature branch first: git checkout -b feature/my-changes",
            branch
        );
    }
    Ok(branch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_file_exists_with_existing_file() {
        // Given: A file that exists (this source file)
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/preflight.rs");
        // When: check_file_exists is called
        let result = check_file_exists(&path, "Source file");
        // Then: Returns Ok
        assert!(result.is_ok());
    }

    #[test]
    fn check_file_exists_with_missing_file() {
        // Given: A path that does not exist
        let path = Path::new("/nonexistent/path/to/file.txt");
        // When: check_file_exists is called
        let result = check_file_exists(path, "Missing file");
        // Then: Returns Err with descriptive message
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Missing file"));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn check_git_safety_on_own_repo() {
        // Given: We are in a git repo not on main/master
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        // When: check_git_safety is called
        let result = check_git_safety(repo);
        // Then: either succeeds (feature branch) or fails (protected branch)
        // We just verify it doesn't panic
        match result {
            Ok(branch) => assert!(!branch.is_empty()),
            Err(e) => assert!(e.to_string().contains("protected branch")),
        }
    }

    #[test]
    fn check_copilot_installed_does_not_panic() {
        // Given: System may or may not have copilot
        // When: check_copilot_installed is called
        let result = check_copilot_installed();
        // Then: Returns either Ok(path) or Err, never panics
        match result {
            Ok(path) => assert!(path.exists()),
            Err(e) => assert!(e.to_string().contains("copilot")),
        }
    }
}

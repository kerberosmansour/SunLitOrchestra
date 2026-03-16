//! Git repository detection and branch safety checks.

use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Check whether the given path is inside a git repository.
pub fn is_git_repo(path: &Path) -> bool {
    Command::new("git")
        .args(["-C", &path.to_string_lossy(), "rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Return the current branch name for the repo at the given path.
pub fn current_branch(path: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["-C", &path.to_string_lossy(), "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to run git rev-parse")?;
    if !output.status.success() {
        bail!(
            "git rev-parse failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Returns true if the branch name is a protected branch (`main` or `master`).
pub fn is_protected_branch(branch: &str) -> bool {
    matches!(branch, "main" | "master")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detect_git_repo() {
        // Given: A directory inside a git repository (this repo)
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // When: is_git_repo(path) is called
        // Then: Returns true
        assert!(is_git_repo(&path));
    }

    #[test]
    fn detect_non_git_dir() {
        // Given: A /tmp directory with no .git
        let tmp = std::env::temp_dir().join("sldo_test_git_non_repo");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        // When: is_git_repo(path)
        // Then: Returns false
        assert!(!is_git_repo(&tmp));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn protected_branch_check_main() {
        // Given: Branch name is "main"
        // When: is_protected_branch("main")
        // Then: Returns true
        assert!(is_protected_branch("main"));
    }

    #[test]
    fn protected_branch_check_master() {
        // Given: Branch name is "master"
        // When: is_protected_branch("master")
        // Then: Returns true
        assert!(is_protected_branch("master"));
    }

    #[test]
    fn safe_branch_check() {
        // Given: Branch name is "feature/foo"
        // When: is_protected_branch("feature/foo")
        // Then: Returns false
        assert!(!is_protected_branch("feature/foo"));
    }

    #[test]
    fn current_branch_returns_non_empty() {
        // Given: We are inside a git repo
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // When: current_branch is called
        let branch = current_branch(&path).unwrap();
        // Then: Returns a non-empty string
        assert!(!branch.is_empty());
    }
}

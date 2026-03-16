//! Timestamped logging and log-file writing.

use anyhow::{Context, Result};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::color::ts;

/// A log file that supports timestamped append operations.
pub struct LogFile {
    path: PathBuf,
}

impl LogFile {
    /// Create a new LogFile, ensuring the parent directory exists.
    pub fn new(dir: &Path, filename: &str) -> Result<Self> {
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create log directory: {}", dir.display()))?;
        let path = dir.join(filename);
        Ok(Self { path })
    }

    /// Append a timestamped line to the log file.
    pub fn append(&self, line: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| format!("Failed to open log file: {}", self.path.display()))?;
        writeln!(file, "[{}] {}", ts(), line)
            .with_context(|| format!("Failed to write to log file: {}", self.path.display()))?;
        Ok(())
    }

    /// Returns the path to the log file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Ensure the `.copilot-logs/` directory exists under the given project dir.
/// Returns the path to the log directory.
pub fn ensure_log_dir(project_dir: &Path) -> Result<PathBuf> {
    let log_dir = project_dir.join(".copilot-logs");
    fs::create_dir_all(&log_dir)
        .with_context(|| format!("Failed to create log dir: {}", log_dir.display()))?;
    Ok(log_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn log_file_creates_parent_dirs() {
        // Given: A temporary directory
        let tmp = std::env::temp_dir().join("sldo_test_logging_create");
        let _ = fs::remove_dir_all(&tmp);
        let sub = tmp.join("sub").join("dir");

        // When: LogFile::new is called with nested path
        let log = LogFile::new(&sub, "test.log");

        // Then: The directory is created and the log is ok
        assert!(log.is_ok());
        assert!(sub.exists());

        // Cleanup
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn log_file_appends_timestamped_line() {
        // Given: A LogFile in a temp directory
        let tmp = std::env::temp_dir().join("sldo_test_logging_append");
        let _ = fs::remove_dir_all(&tmp);
        let log = LogFile::new(&tmp, "test.log").unwrap();

        // When: append is called
        log.append("hello world").unwrap();

        // Then: The file contains the timestamped line
        let content = fs::read_to_string(log.path()).unwrap();
        assert!(content.contains("hello world"));
        assert!(content.contains("[20")); // starts with a year

        // Cleanup
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn ensure_log_dir_creates_copilot_logs() {
        // Given: A temporary project directory
        let tmp = std::env::temp_dir().join("sldo_test_logging_ensure");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        // When: ensure_log_dir is called
        let log_dir = ensure_log_dir(&tmp).unwrap();

        // Then: .copilot-logs directory exists
        assert!(log_dir.ends_with(".copilot-logs"));
        assert!(log_dir.exists());

        // Cleanup
        let _ = fs::remove_dir_all(&tmp);
    }
}

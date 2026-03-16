//! Copilot CLI invocation.
//!
//! Builds and executes a `copilot` CLI command, piping output to both the
//! terminal and a log file.

use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::logging::LogFile;

/// Configuration for a single Copilot CLI invocation.
pub struct CopilotInvocation {
    pub prompt: String,
    pub model: String,
    pub allow_flags: Vec<String>,
    pub deny_flags: Vec<String>,
    pub working_dir: PathBuf,
}

impl CopilotInvocation {
    /// Spawn the copilot process, pipe stdout/stderr to both the terminal and
    /// the log file, and return the exit code.
    pub fn run(&self, log_file: &LogFile) -> Result<i32> {
        let mut cmd = Command::new("copilot");
        cmd.arg("-p")
            .arg(&self.prompt)
            .arg("--model")
            .arg(&self.model);

        for flag in &self.allow_flags {
            cmd.arg(flag);
        }
        for flag in &self.deny_flags {
            cmd.arg(flag);
        }

        cmd.current_dir(&self.working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        log_file.append(&format!("Running copilot with model={}", self.model))?;

        let mut child = cmd.spawn().context("Failed to spawn copilot process")?;

        // Stream stdout
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                println!("{}", line);
                let _ = log_file.append(&line);
            }
        }

        // Stream stderr
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                eprintln!("{}", line);
                let _ = log_file.append(&format!("STDERR: {}", line));
            }
        }

        let status = child.wait().context("Failed to wait for copilot process")?;
        let code = status.code().unwrap_or(-1);
        log_file.append(&format!("copilot exited with code {}", code))?;
        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn copilot_invocation_builds_correctly() {
        // Given: A CopilotInvocation with known fields
        let inv = CopilotInvocation {
            prompt: "test prompt".to_string(),
            model: "test-model".to_string(),
            allow_flags: vec!["--allow-tool=write".to_string()],
            deny_flags: vec!["--deny-tool=shell(rm -rf /)".to_string()],
            working_dir: PathBuf::from("/tmp"),
        };
        // When: Fields are accessed
        // Then: They match what was set
        assert_eq!(inv.prompt, "test prompt");
        assert_eq!(inv.model, "test-model");
        assert_eq!(inv.allow_flags.len(), 1);
        assert_eq!(inv.deny_flags.len(), 1);
    }

    #[test]
    fn copilot_invocation_run_handles_missing_binary() {
        // Given: A CopilotInvocation pointing at the working dir
        // but copilot may not be installed
        let tmp = std::env::temp_dir().join("sldo_test_copilot_run");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let log = LogFile::new(&tmp, "copilot.log").unwrap();
        let inv = CopilotInvocation {
            prompt: "hello".to_string(),
            model: "test".to_string(),
            allow_flags: vec![],
            deny_flags: vec![],
            working_dir: tmp.clone(),
        };

        // When: run() is called and copilot is not installed
        let result = inv.run(&log);

        // Then: It either succeeds (copilot exists) or returns an error (not panics)
        match result {
            Ok(code) => {
                // copilot existed, we got some exit code
                let _ = code;
            }
            Err(e) => {
                assert!(
                    e.to_string().contains("copilot") || e.to_string().contains("spawn"),
                    "Unexpected error: {}",
                    e
                );
            }
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn copilot_invocation_working_dir() {
        // Given: An invocation with a specific working dir
        let inv = CopilotInvocation {
            prompt: "test".to_string(),
            model: "m".to_string(),
            allow_flags: vec![],
            deny_flags: vec![],
            working_dir: PathBuf::from("/tmp/test"),
        };
        // When: working_dir is checked
        // Then: it matches
        assert_eq!(inv.working_dir, Path::new("/tmp/test"));
    }
}

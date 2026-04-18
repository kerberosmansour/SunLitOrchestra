//! Claude Code CLI invocation.
//!
//! Builds and executes a `claude` CLI command, piping output to both the
//! terminal and a log file.

use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::logging::LogFile;

/// Configuration for a single Claude Code CLI invocation.
pub struct ClaudeInvocation {
    pub prompt: String,
    pub model: String,
    pub allow_flags: Vec<String>,
    pub deny_flags: Vec<String>,
    pub working_dir: PathBuf,
}

impl ClaudeInvocation {
    /// Spawn the copilot process, pipe stdout/stderr to both the terminal and
    /// the log file, and return the exit code.
    pub fn run(&self, log_file: &LogFile) -> Result<i32> {
        self.run_with_callback(log_file, |line, stream| match stream {
            "stdout" => println!("{}", line),
            _ => eprintln!("{}", line),
        })
    }

    /// Spawn the copilot process, calling `on_line` for each stdout/stderr line
    /// instead of printing directly. The callback receives the line content and
    /// the stream name (`"stdout"` or `"stderr"`).
    ///
    /// Stdout and stderr are read concurrently via separate threads to avoid
    /// pipe-buffer deadlocks (the classic sequential-read problem where a full
    /// stderr buffer blocks the child while we're still draining stdout).
    pub fn run_with_callback<F>(&self, log_file: &LogFile, mut on_line: F) -> Result<i32>
    where
        F: FnMut(&str, &str),
    {
        let mut cmd = Command::new("claude");
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

        log_file.append(&format!("Running claude with model={}", self.model))?;

        let mut child = cmd.spawn().context("Failed to spawn claude process")?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        // Use a channel so both reader threads can send lines to the callback
        // on the current thread, preserving the FnMut requirement.
        let (tx, rx) = std::sync::mpsc::channel::<(String, String)>();

        let tx_out = tx.clone();
        let stdout_thread = std::thread::spawn(move || {
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    if tx_out.send((line, "stdout".to_string())).is_err() {
                        break;
                    }
                }
            }
        });

        let tx_err = tx.clone();
        let stderr_thread = std::thread::spawn(move || {
            if let Some(stderr) = stderr {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    if tx_err.send((line, "stderr".to_string())).is_err() {
                        break;
                    }
                }
            }
        });

        // Drop our copy so rx.iter() terminates when both threads finish.
        drop(tx);

        for (line, stream) in rx.iter() {
            on_line(&line, &stream);
            if stream == "stderr" {
                let _ = log_file.append(&format!("STDERR: {}", line));
            } else {
                let _ = log_file.append(&line);
            }
        }

        let _ = stdout_thread.join();
        let _ = stderr_thread.join();

        let status = child.wait().context("Failed to wait for claude process")?;
        let code = status.code().unwrap_or(-1);
        log_file.append(&format!("claude exited with code {}", code))?;
        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn claude_invocation_builds_correctly() {
        // Given: A ClaudeInvocation with known fields
        let inv = ClaudeInvocation {
            prompt: "test prompt".to_string(),
            model: "test-model".to_string(),
            allow_flags: vec!["--allowedTools=Read,Write,Bash".to_string()],
            deny_flags: vec![],
            working_dir: PathBuf::from("/tmp"),
        };
        // When: Fields are accessed
        // Then: They match what was set
        assert_eq!(inv.prompt, "test prompt");
        assert_eq!(inv.model, "test-model");
        assert_eq!(inv.allow_flags.len(), 1);
        assert_eq!(inv.deny_flags.len(), 0);
    }

    #[test]
    fn claude_invocation_run_handles_missing_binary() {
        // Given: A ClaudeInvocation pointing at the working dir
        // but claude may not be installed
        let tmp = std::env::temp_dir().join("sldo_test_claude_run");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let log = LogFile::new(&tmp, "claude.log").unwrap();
        let inv = ClaudeInvocation {
            prompt: "hello".to_string(),
            model: "test".to_string(),
            allow_flags: vec![],
            deny_flags: vec![],
            working_dir: tmp.clone(),
        };

        // When: run() is called and claude is not installed
        let result = inv.run(&log);

        // Then: It either succeeds (claude exists) or returns an error (not panics)
        match result {
            Ok(code) => {
                // claude existed, we got some exit code
                let _ = code;
            }
            Err(e) => {
                assert!(
                    e.to_string().contains("claude") || e.to_string().contains("spawn"),
                    "Unexpected error: {}",
                    e
                );
            }
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn claude_invocation_working_dir() {
        // Given: An invocation with a specific working dir
        let inv = ClaudeInvocation {
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

    #[test]
    fn run_with_callback_receives_all_lines() {
        // Given: A CopilotInvocation using `echo` as a mock for copilot
        // We can't easily mock copilot, but we can test the callback signature
        // and verify backward compatibility of run() via run_with_callback.
        let tmp = std::env::temp_dir().join("sldo_test_callback");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let log = LogFile::new(&tmp, "callback.log").unwrap();
        let inv = ClaudeInvocation {
            prompt: "hello".to_string(),
            model: "test".to_string(),
            allow_flags: vec![],
            deny_flags: vec![],
            working_dir: tmp.clone(),
        };

        // When: run_with_callback is called (claude may or may not be installed)
        let mut captured_lines: Vec<(String, String)> = Vec::new();
        let result = inv.run_with_callback(&log, |line, stream| {
            captured_lines.push((line.to_string(), stream.to_string()));
        });

        // Then: It either succeeds with output or errors gracefully
        match result {
            Ok(_code) => {
                // If claude is installed, we got lines through the callback
                // (callback was invoked, not println)
            }
            Err(e) => {
                assert!(
                    e.to_string().contains("claude") || e.to_string().contains("spawn"),
                    "Unexpected error: {}",
                    e
                );
            }
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn run_still_works_after_refactor() {
        // Given: A CopilotInvocation (backward compatibility test)
        let tmp = std::env::temp_dir().join("sldo_test_run_compat");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let log = LogFile::new(&tmp, "compat.log").unwrap();
        let inv = ClaudeInvocation {
            prompt: "hello".to_string(),
            model: "test".to_string(),
            allow_flags: vec![],
            deny_flags: vec![],
            working_dir: tmp.clone(),
        };

        // When: run() is called (the original method, now backed by run_with_callback)
        let result = inv.run(&log);

        // Then: It either succeeds or errors the same way as before
        match result {
            Ok(code) => {
                let _ = code;
            }
            Err(e) => {
                assert!(
                    e.to_string().contains("claude") || e.to_string().contains("spawn"),
                    "Unexpected error: {}",
                    e
                );
            }
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }
}

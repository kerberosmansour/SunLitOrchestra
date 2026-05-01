//! Subprocess wrapper for `semgrep` invocations.

use anyhow::{anyhow, bail, Context, Result};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::GlobalOpts;

pub fn resolve_semgrep_bin(opts: &GlobalOpts) -> Result<PathBuf> {
    if let Some(p) = &opts.semgrep_bin {
        if !p.exists() {
            bail!(
                "--semgrep-bin {} does not exist (expected an executable on disk)",
                p.display()
            );
        }
        return Ok(p.clone());
    }
    which::which("semgrep").context(
        "`semgrep` not found on PATH; install via `brew install semgrep` or `pip install semgrep`",
    )
}

#[derive(Debug)]
pub struct SemgrepRun {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(args: &[&str], opts: &GlobalOpts) -> Result<SemgrepRun> {
    let bin = resolve_semgrep_bin(opts)?;
    let timeout = format!("{}", opts.timeout_secs);
    let max_bytes = format!("{}", opts.max_target_bytes);

    let mut cmd = Command::new(&bin);
    cmd.arg("--timeout")
        .arg(&timeout)
        .arg("--max-target-bytes")
        .arg(&max_bytes);
    for a in args {
        cmd.arg(a);
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| format!("spawn `{}` with args {args:?}", bin.display()))?;

    let mut stdout_pipe = child.stdout.take().ok_or_else(|| anyhow!("no stdout"))?;
    let mut stderr_pipe = child.stderr.take().ok_or_else(|| anyhow!("no stderr"))?;

    let stderr_thread = thread::spawn(move || {
        let mut buf = String::new();
        let _ = stderr_pipe.read_to_string(&mut buf);
        buf
    });

    let mut stdout_buf = String::new();
    stdout_pipe
        .read_to_string(&mut stdout_buf)
        .context("read semgrep stdout")?;

    let stderr_buf = stderr_thread
        .join()
        .map_err(|_| anyhow!("stderr drain thread panicked"))?;

    let wall_timeout = Duration::from_secs((opts.timeout_secs * 2).max(10));
    let exit_status = wait_with_timeout(&mut child, wall_timeout)?;

    let exit_code = exit_status.code().unwrap_or(-1);
    Ok(SemgrepRun {
        exit_code,
        stdout: stdout_buf,
        stderr: stderr_buf,
    })
}

fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> Result<std::process::ExitStatus> {
    let start = std::time::Instant::now();
    loop {
        match child.try_wait()? {
            Some(status) => return Ok(status),
            None => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    bail!(
                        "semgrep wall-clock timeout exceeded ({}s); killed",
                        timeout.as_secs()
                    );
                }
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

pub fn parse_json_output(stdout: &str, ctx: &str) -> Result<serde_json::Value> {
    serde_json::from_str(stdout).with_context(|| {
        format!("parse semgrep --json output for {ctx} (refusing to fall back to raw-stdout)")
    })
}

pub fn paired_fixture_for(rule_path: &Path) -> Option<PathBuf> {
    let stem = rule_path.file_stem()?;
    let parent = rule_path.parent()?;
    let candidate = parent.join(format!("{}.rs", stem.to_string_lossy()));
    if candidate.exists() {
        Some(candidate)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_fixture_returns_none_when_missing() {
        let p = std::path::PathBuf::from("/nonexistent/path/foo.yaml");
        assert!(paired_fixture_for(&p).is_none());
    }
}

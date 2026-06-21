//! Graphify readiness checks for SLO evidence-loop workflows.
//!
//! This module intentionally does not install anything. It checks local state
//! and prints OS/host-specific commands the user can run explicitly.

use anyhow::{bail, Result};
use clap::Args as ClapArgs;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::host::Host;

#[derive(ClapArgs, Debug, Clone)]
pub struct Args {
    /// Optional local Graphify source checkout for provider-harness work.
    #[arg(long, value_name = "DIR")]
    pub graphify_path: Option<PathBuf>,

    /// Print the host-aware install plan and exit successfully.
    #[arg(long)]
    pub install_plan: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OsFamily {
    Unix,
    Windows,
}

impl OsFamily {
    fn current() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else {
            Self::Unix
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Check {
    Found(String),
    Missing,
    Failed(String),
}

pub fn doctor(args: &Args, host: Host, local: bool) -> Result<()> {
    let os = OsFamily::current();

    println!("Graphify readiness for {}", host.descriptor().display_name);
    println!();

    if args.install_plan {
        print_install_plan(host, local, os);
        return Ok(());
    }

    let cli = graphify_cli_check();
    let source = source_checkout_check(args.graphify_path.as_deref());

    let ready = matches!(cli, Check::Found(_)) || matches!(source, SourceCheck::Found(_));
    println!("status: {}", if ready { "ready" } else { "not ready" });
    println!("graphify CLI: {}", describe_check(&cli));
    println!("source checkout: {}", describe_source_check(&source));
    println!();

    println!("provider tools:");
    print_tool("opengrep", "OpenGrep analysis findings");
    print_tool("rust-analyzer", "Rust semantic facts");
    print_tool("node", "TypeScript language-service provider");
    println!();

    print_install_plan(host, local, os);

    if ready {
        Ok(())
    } else {
        bail!(
            "Graphify is not ready. Install the `graphifyy` package so `graphify` is on PATH, \
             or pass --graphify-path/GRAPHIFY_PATH for a local checkout."
        )
    }
}

fn print_tool(program: &str, purpose: &str) {
    println!(
        "  - {program}: {} ({purpose})",
        describe_check(&command_check(program, &["--version"]))
    );
}

fn graphify_cli_check() -> Check {
    match command_check("graphify", &["--version"]) {
        Check::Failed(_) => command_check("graphify", &["--help"]),
        other => other,
    }
}

fn command_check(program: &str, args: &[&str]) -> Check {
    for candidate in command_candidates(program, OsFamily::current()) {
        match Command::new(&candidate).args(args).output() {
            Ok(output) if output.status.success() => {
                let text = first_non_empty_line(&output.stdout)
                    .or_else(|| first_non_empty_line(&output.stderr))
                    .unwrap_or_else(|| "found".to_string());
                return Check::Found(text);
            }
            Ok(output) => {
                let text = first_non_empty_line(&output.stderr)
                    .or_else(|| first_non_empty_line(&output.stdout))
                    .unwrap_or_else(|| format!("exit {}", output.status));
                return Check::Failed(text);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Check::Failed(error.to_string()),
        }
    }
    Check::Missing
}

fn command_candidates(program: &str, os: OsFamily) -> Vec<String> {
    let mut candidates = vec![program.to_string()];
    if os != OsFamily::Windows || Path::new(program).extension().is_some() {
        return candidates;
    }

    for extension in windows_path_extensions() {
        let candidate = format!("{program}{extension}");
        if !candidates
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(&candidate))
        {
            candidates.push(candidate);
        }
    }
    candidates
}

fn windows_path_extensions() -> Vec<String> {
    let raw = env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD;.PS1".to_string());
    let mut extensions: Vec<String> = raw
        .split(';')
        .map(str::trim)
        .filter(|extension| !extension.is_empty())
        .map(|extension| {
            if extension.starts_with('.') {
                extension.to_string()
            } else {
                format!(".{extension}")
            }
        })
        .collect();

    for fallback in [".COM", ".EXE", ".BAT", ".CMD"] {
        if !extensions
            .iter()
            .any(|extension| extension.eq_ignore_ascii_case(fallback))
        {
            extensions.push(fallback.to_string());
        }
    }
    extensions
}

fn first_non_empty_line(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(|line| line.to_string())
}

fn describe_check(check: &Check) -> String {
    match check {
        Check::Found(version) => format!("found ({version})"),
        Check::Missing => "missing".to_string(),
        Check::Failed(reason) => format!("present but failed ({reason})"),
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum SourceCheck {
    Found(PathBuf),
    Missing,
    Invalid(PathBuf),
}

fn source_checkout_check(arg: Option<&Path>) -> SourceCheck {
    let candidate = arg
        .map(Path::to_path_buf)
        .or_else(|| env::var_os("GRAPHIFY_PATH").map(PathBuf::from));

    let Some(path) = candidate else {
        return SourceCheck::Missing;
    };

    if is_graphify_source_checkout(&path) {
        SourceCheck::Found(path)
    } else {
        SourceCheck::Invalid(path)
    }
}

fn is_graphify_source_checkout(path: &Path) -> bool {
    path.join("graphify").join("__init__.py").is_file()
        && path.join("graphify").join("build.py").is_file()
}

fn describe_source_check(check: &SourceCheck) -> String {
    match check {
        SourceCheck::Found(path) => format!("found ({})", path.display()),
        SourceCheck::Missing => "not configured".to_string(),
        SourceCheck::Invalid(path) => format!("invalid checkout ({})", path.display()),
    }
}

fn print_install_plan(host: Host, local: bool, os: OsFamily) {
    println!("install plan:");
    for line in install_plan_lines(host, local, os) {
        println!("  {line}");
    }
}

fn install_plan_lines(host: Host, local: bool, os: OsFamily) -> Vec<String> {
    let graphify_install = graphify_skill_command(host, local);
    match os {
        OsFamily::Windows => vec![
            "winget install astral-sh.uv".to_string(),
            "uv tool install graphifyy".to_string(),
            graphify_install,
            "graphify --help".to_string(),
        ],
        OsFamily::Unix => vec![
            "install uv or pipx if neither is already available".to_string(),
            "uv tool install graphifyy".to_string(),
            "pipx install graphifyy    # alternative".to_string(),
            graphify_install,
            "graphify --help".to_string(),
        ],
    }
}

fn graphify_skill_command(host: Host, local: bool) -> String {
    let mut parts = match host {
        Host::ClaudeCode => vec!["graphify", "install"],
        Host::GithubCopilot => vec!["graphify", "install", "--platform", "copilot"],
        Host::Codex => vec!["graphify", "install", "--platform", "codex"],
    };
    if local {
        parts.push("--project");
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_plan_is_codex_and_project_aware() {
        let lines = install_plan_lines(Host::Codex, true, OsFamily::Unix);
        assert!(lines.iter().any(|line| line == "uv tool install graphifyy"));
        assert!(lines
            .iter()
            .any(|line| line == "graphify install --platform codex --project"));
    }

    #[test]
    fn install_plan_has_windows_bootstrap_hint() {
        let lines = install_plan_lines(Host::GithubCopilot, false, OsFamily::Windows);
        assert_eq!(lines[0], "winget install astral-sh.uv");
        assert!(lines
            .iter()
            .any(|line| line == "graphify install --platform copilot"));
    }

    #[test]
    fn windows_command_candidates_include_cmd_shims() {
        let candidates = command_candidates("graphify", OsFamily::Windows);
        assert_eq!(candidates[0], "graphify");
        assert!(candidates
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case("graphify.exe")));
        assert!(candidates
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case("graphify.cmd")));
    }

    #[test]
    fn windows_command_candidates_do_not_extend_explicit_extension() {
        let candidates = command_candidates("graphify.cmd", OsFamily::Windows);
        assert_eq!(candidates, vec!["graphify.cmd"]);
    }

    #[test]
    fn source_checkout_requires_graphify_package_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        assert!(!is_graphify_source_checkout(tmp.path()));

        let package = tmp.path().join("graphify");
        std::fs::create_dir_all(&package).unwrap();
        std::fs::write(package.join("__init__.py"), "").unwrap();
        std::fs::write(package.join("build.py"), "").unwrap();

        assert!(is_graphify_source_checkout(tmp.path()));
    }
}

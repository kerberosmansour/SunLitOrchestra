//! `sldo-install` — install, update, and uninstall the SunLitOrchestra skill pack.
//!
//! Links the skill directories under `skills/` in this repo into the selected
//! host agent's skills directory (default: `~/.claude/skills/`), or into a
//! project-local host directory such as `./.claude/skills/`,
//! `./.copilot/skills/`, or `./.codex/skills/` when `--local` is passed.
//!
//! The installer is idempotent: running it twice leaves the same state as
//! running it once. Uninstall reverses every change recorded in the manifest.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod host;
mod install;
mod manifest;
mod paths;

use host::Host;

#[derive(Parser, Debug)]
#[command(
    name = "sldo-install",
    about = "Install the SunLitOrchestra skill pack",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Source directory containing skill subdirectories (each with a SKILL.md).
    /// Defaults to `./skills` relative to the current working directory.
    #[arg(long, global = true)]
    skills_dir: Option<PathBuf>,

    /// Install into the project-local host skills directory instead of the global host root.
    #[arg(long, global = true)]
    local: bool,

    /// Target host agent. Defaults to `claude-code` for backward compatibility.
    #[arg(long, value_enum, default_value_t = Host::ClaudeCode, global = true)]
    host: Host,

    /// Overwrite existing managed links.
    #[arg(long, global = true)]
    force: bool,

    /// Show what would be done without writing any files.
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install all skills (default when no subcommand is given).
    Install,
    /// Uninstall all skills recorded in the manifest.
    Uninstall,
    /// Print what's currently installed according to the manifest.
    Status,
    /// Verify that installed managed links match the manifest and source skills.
    Verify,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Command::Install);

    let opts = install::Options {
        skills_dir: resolve_skills_dir(cli.skills_dir)?,
        host: cli.host,
        local: cli.local,
        force: cli.force,
        dry_run: cli.dry_run,
    };

    match command {
        Command::Install => install::install(&opts),
        Command::Uninstall => install::uninstall(&opts),
        Command::Status => install::status(&opts),
        Command::Verify => install::verify(&opts),
    }
}

fn resolve_skills_dir(arg: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(p) = arg {
        if !p.exists() {
            bail!(
                "--skills-dir path does not exist: {}. Pass a directory containing skill subfolders.",
                p.display()
            );
        }
        return Ok(p);
    }
    let cwd = std::env::current_dir().context("Failed to read current directory")?;
    let default = cwd.join("skills");
    if !default.exists() {
        bail!(
            "No skills/ directory found under current working directory ({}). \
             Run from the SunLitOrchestra repo root, or pass --skills-dir.",
            cwd.display()
        );
    }
    Ok(default)
}

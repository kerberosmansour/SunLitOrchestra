//! `sldo-tla-sha` — compute and verify SHA-256 for pinned TLA+ tool artifacts.
//!
//! Two modes:
//!   - Default: read `tools.toml`, fetch every entry whose `sha256 = "UNSET"`,
//!     stream-hash the response, print a TOML patch on stdout.
//!   - `--verify`: re-fetch every entry with a populated `sha256` and confirm
//!     the computed hash matches. Exit non-zero on any mismatch.
//!
//! The binary never writes to `tools.toml`. Humans apply the patch in a commit.

use anyhow::{bail, Context, Result};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

use sldo_tla_sha::{
    fetch_and_hash, format_patch, host_of, is_host_allowed, ToolsToml, DEFAULT_MAX_BYTES,
};

#[derive(Parser, Debug)]
#[command(
    name = "sldo-tla-sha",
    about = "Compute SHA-256 for pinned TLA+ tool artifacts; emit a TOML patch.",
    version
)]
struct Cli {
    /// Path to `tools.toml`. Defaults to `skills/slo-tla/tools.toml` relative to CWD.
    #[arg(long, default_value = "skills/slo-tla/tools.toml")]
    tools_toml: PathBuf,

    /// Maximum response size (bytes) before streaming aborts. Protects against
    /// compromised CDNs serving oversized content.
    #[arg(long, default_value_t = DEFAULT_MAX_BYTES)]
    max_bytes: u64,

    /// Print the fetch plan without executing. No network I/O.
    #[arg(long)]
    dry_run: bool,

    /// Re-fetch populated sections and verify each stored sha256 matches.
    /// Exit non-zero on any mismatch.
    #[arg(long)]
    verify: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let tools = ToolsToml::load(&cli.tools_toml)?;

    if cli.verify {
        return run_verify(&tools, cli.max_bytes);
    }
    run_populate(&tools, cli.max_bytes, cli.dry_run)
}

fn run_populate(tools: &ToolsToml, max_bytes: u64, dry_run: bool) -> Result<()> {
    let unset = tools.unset_sections();

    if unset.is_empty() {
        eprintln!("{}", "no UNSET entries; nothing to populate.".dimmed());
        return Ok(());
    }

    if dry_run {
        println!("# dry-run: the following URLs WOULD be fetched (no network used).\n");
        for (section, entry) in &unset {
            // Sanity-check the URL up front even in dry-run.
            let host = host_of(&entry.url)?;
            let status = if is_host_allowed(&host) { "OK" } else { "FOREIGN" };
            println!(
                "would fetch: [{}]  url = {}  host = {} ({})",
                section, entry.url, host, status
            );
        }
        return Ok(());
    }

    let mut updates: Vec<(String, String)> = Vec::new();
    for (section, entry) in &unset {
        eprintln!(
            "{} [{}] fetching {} ...",
            "→".dimmed(),
            section.bold(),
            entry.url
        );
        let sha = fetch_and_hash(&entry.url, max_bytes)
            .with_context(|| format!("failed to fetch/hash [{}]", section))?;
        eprintln!("  sha256 = {}", sha.green());
        updates.push((section.to_string(), sha));
    }

    // Stable ordering for reproducibility.
    updates.sort_by(|a, b| a.0.cmp(&b.0));
    print!("{}", format_patch(&updates));
    Ok(())
}

fn run_verify(tools: &ToolsToml, max_bytes: u64) -> Result<()> {
    let populated = tools.populated_sections();
    let still_unset = tools.unset_sections();

    if !still_unset.is_empty() {
        let names: Vec<_> = still_unset.iter().map(|(n, _)| *n).collect();
        bail!(
            "cannot verify: {} section(s) still have sha256 = \"UNSET\" ({}). \
             Run `sldo-tla-sha` (without --verify) first, apply the patch, then re-run --verify.",
            still_unset.len(),
            names.join(", ")
        );
    }

    if populated.is_empty() {
        eprintln!("{}", "no populated sections to verify.".dimmed());
        return Ok(());
    }

    let mut mismatches: Vec<(String, String, String)> = Vec::new();
    for (section, entry) in &populated {
        eprintln!(
            "{} [{}] re-fetching {} ...",
            "→".dimmed(),
            section.bold(),
            entry.url
        );
        let actual = fetch_and_hash(&entry.url, max_bytes)
            .with_context(|| format!("failed to fetch/hash [{}]", section))?;
        if actual == entry.sha256 {
            println!("{} [{}]  sha256 matches", "PASS".green(), section);
        } else {
            println!(
                "{} [{}]  expected {}  got {}",
                "FAIL".red().bold(),
                section,
                entry.sha256,
                actual
            );
            mismatches.push((section.to_string(), entry.sha256.clone(), actual));
        }
    }

    if !mismatches.is_empty() {
        bail!("{} section(s) failed verification", mismatches.len());
    }
    Ok(())
}

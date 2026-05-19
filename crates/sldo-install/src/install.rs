//! Install, uninstall, status, and verify logic.
//!
//! Idempotency contract: calling `install` repeatedly with the same inputs
//! yields the same filesystem state. Calling `uninstall` reverses every
//! managed link and removes the manifest.
//!
//! Managed-link safety: we never overwrite a non-link target. Overwriting an
//! existing managed link requires `--force`. Uninstall only removes managed
//! links that point at the source recorded in the manifest — if a user has
//! replaced a link manually, we leave it alone and warn.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::host::Host;
use crate::manifest::{Entry, Manifest};
use crate::paths;

pub struct Options {
    pub skills_dir: PathBuf,
    pub host: Host,
    pub local: bool,
    pub force: bool,
    pub dry_run: bool,
}

/// Result of planning an install: what we'd do, without doing it.
#[derive(Debug, Default)]
struct Plan {
    to_create: Vec<PlanItem>,
    to_update: Vec<PlanItem>, // existing managed link pointing elsewhere, needs --force
    up_to_date: Vec<PlanItem>,
    conflicts: Vec<PlanItem>, // non-link target, cannot overwrite
}

#[derive(Debug, Clone)]
struct PlanItem {
    name: String,
    source: PathBuf,
    target: PathBuf,
}

/// A skill directory in the source tree: `<skills_dir>/<name>/SKILL.md` must exist.
fn discover_skills(skills_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let entries = fs::read_dir(skills_dir)
        .with_context(|| format!("Failed to read skills dir: {}", skills_dir.display()))?;

    let mut out = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        // Skip dotfiles and README-style siblings.
        if name.starts_with('.') {
            continue;
        }
        let skill_md = path.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }
        out.push((name, path));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

fn compute_root_and_manifest(opts: &Options) -> Result<(PathBuf, PathBuf)> {
    if opts.local {
        let cwd = std::env::current_dir().context("Failed to read current directory")?;
        Ok((
            paths::local_skills_root(&cwd, opts.host),
            paths::local_manifest_path(&cwd, opts.host),
        ))
    } else {
        let home = paths::home_dir()?;
        Ok((
            paths::global_skills_root(&home, opts.host),
            paths::global_manifest_path(&home),
        ))
    }
}

fn plan(opts: &Options) -> Result<(Plan, PathBuf, PathBuf)> {
    let (root, manifest_path) = compute_root_and_manifest(opts)?;
    let skills = discover_skills(&opts.skills_dir)?;

    let mut plan = Plan::default();
    for (name, source) in skills {
        let target = root.join(&name);
        let source_abs = fs::canonicalize(&source)
            .with_context(|| format!("Failed to canonicalize source {}", source.display()))?;
        let item = PlanItem {
            name: name.clone(),
            source: source_abs.clone(),
            target: target.clone(),
        };

        if target.exists() || is_managed_link_path(&target) {
            if managed_link_points_to(&target, &source_abs) {
                plan.up_to_date.push(item);
            } else if is_managed_link_path(&target) {
                plan.to_update.push(item);
            } else {
                // Real file or directory where we'd install — conflict.
                plan.conflicts.push(item);
            }
        } else {
            plan.to_create.push(item);
        }
    }

    Ok((plan, root, manifest_path))
}

pub fn install(opts: &Options) -> Result<()> {
    let (plan, root, manifest_path) = plan(opts)?;

    if !plan.conflicts.is_empty() {
        eprintln!("{}", "conflicts (existing non-link paths):".red().bold());
        for item in &plan.conflicts {
            eprintln!("  {} -> {}", item.target.display(), item.source.display());
        }
        bail!(
            "{} skill(s) have non-link paths at their install targets. \
             Remove or relocate them and re-run.",
            plan.conflicts.len()
        );
    }

    if !plan.to_update.is_empty() && !opts.force {
        eprintln!(
            "{}",
            "the following managed links point elsewhere and would be replaced by --force:"
                .yellow()
                .bold()
        );
        for item in &plan.to_update {
            eprintln!("  {}", item.target.display());
        }
        bail!(
            "{} managed link(s) already exist. Pass --force to overwrite.",
            plan.to_update.len()
        );
    }

    if opts.dry_run {
        print_plan(&plan, &root);
        return Ok(());
    }

    fs::create_dir_all(&root)
        .with_context(|| format!("Failed to create skills root: {}", root.display()))?;

    let mut manifest = Manifest::load(&manifest_path)?;

    for item in plan.to_create.iter().chain(plan.to_update.iter()) {
        // Remove pre-existing managed link (only managed links reach this branch).
        if is_managed_link_path(&item.target) {
            remove_managed_link_path(&item.target).with_context(|| {
                format!(
                    "Failed to remove old managed link: {}",
                    item.target.display()
                )
            })?;
        }
        create_managed_link(&item.source, &item.target).with_context(|| {
            format!(
                "Failed to link {} -> {}",
                item.target.display(),
                item.source.display()
            )
        })?;
        manifest.upsert(
            opts.host,
            item.name.clone(),
            item.target.clone(),
            item.source.clone(),
        );
        println!(
            "  {} {} {} {}",
            "+".green().bold(),
            item.name.bold(),
            "→".dimmed(),
            item.source.display().to_string().dimmed()
        );
    }

    for item in &plan.up_to_date {
        // Still ensure it's in the manifest (e.g., after manifest loss).
        if manifest.find(&item.name, opts.host).is_none() {
            manifest.upsert(
                opts.host,
                item.name.clone(),
                item.target.clone(),
                item.source.clone(),
            );
        }
        println!("  {} {} (already installed)", "=".dimmed(), item.name);
    }

    manifest.save(&manifest_path)?;

    if plan.to_create.is_empty() && plan.to_update.is_empty() {
        println!("{}", "no changes.".dimmed());
    } else {
        println!(
            "\ninstalled {} skill(s), {} already up to date.",
            plan.to_create.len() + plan.to_update.len(),
            plan.up_to_date.len()
        );
    }
    Ok(())
}

pub fn uninstall(opts: &Options) -> Result<()> {
    let (root, manifest_path) = compute_root_and_manifest(opts)?;
    let mut manifest = Manifest::load(&manifest_path)?;
    let host_entries: Vec<Entry> = manifest
        .entries_for_host(opts.host)
        .into_iter()
        .cloned()
        .collect();

    if host_entries.is_empty() {
        println!("nothing to uninstall for {}.", opts.host.id());
        return Ok(());
    }

    if opts.dry_run {
        println!("would remove for {}:", opts.host.id());
        for entry in &host_entries {
            println!("  - {}", entry.target.display());
        }
        println!("  manifest: {}", manifest_path.display());
        return Ok(());
    }

    let to_remove = host_entries;
    let mut bad = 0usize;
    for entry in to_remove {
        if let Err(error) = validate_entry_target_in_host_root(&entry, &root) {
            eprintln!("  {} {}: {}", "x".red().bold(), entry.name, error);
            bad += 1;
            continue;
        }
        match remove_managed_link(&entry) {
            Ok(true) => {
                println!("  {} {}", "-".yellow().bold(), entry.name);
                manifest.remove(&entry.name, opts.host);
            }
            Ok(false) => {
                eprintln!(
                    "  {} {} (skipped: target was modified by hand)",
                    "!".yellow(),
                    entry.name
                );
            }
            Err(e) => {
                eprintln!("  {} {}: {}", "x".red().bold(), entry.name, e);
                bad += 1;
            }
        }
    }

    if manifest.entries.is_empty() {
        let _ = fs::remove_file(&manifest_path);
    } else {
        manifest.save(&manifest_path)?;
    }

    if bad > 0 {
        bail!("{} skill(s) were rejected or failed uninstall", bad);
    }
    Ok(())
}

pub fn status(opts: &Options) -> Result<()> {
    let (_root, manifest_path) = compute_root_and_manifest(opts)?;
    let manifest = Manifest::load(&manifest_path)?;
    let host_entries = manifest.entries_for_host(opts.host);
    if host_entries.is_empty() {
        println!("no skills installed for {}.", opts.host.id());
        return Ok(());
    }
    println!(
        "installed skills for {} ({}):",
        opts.host.id(),
        host_entries.len()
    );
    for e in host_entries {
        println!(
            "  {} -> {}  [installed {}]",
            e.name,
            e.source.display(),
            e.installed_at
        );
    }
    Ok(())
}

pub fn verify(opts: &Options) -> Result<()> {
    let (root, manifest_path) = compute_root_and_manifest(opts)?;
    let manifest = Manifest::load(&manifest_path)?;
    let host_entries = manifest.entries_for_host(opts.host);
    if host_entries.is_empty() {
        println!(
            "manifest is empty for {} — nothing to verify.",
            opts.host.id()
        );
        return Ok(());
    }

    println!(
        "verifying {} entries for {}:",
        host_entries.len(),
        opts.host.id()
    );
    let mut bad = 0usize;
    for entry in host_entries {
        if let Err(error) = validate_entry_target_in_host_root(entry, &root) {
            eprintln!("  {} {}: {}", "x".red(), entry.name, error);
            bad += 1;
            continue;
        }
        if !is_managed_link_path(&entry.target) {
            eprintln!(
                "  {} {}: target is not a managed link",
                "x".red(),
                entry.name
            );
            bad += 1;
            continue;
        }
        let actual = match managed_link_target(&entry.target) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("  {} {}: link target failed: {}", "x".red(), entry.name, e);
                bad += 1;
                continue;
            }
        };
        if let Err(e) = fs::metadata(&entry.source) {
            eprintln!(
                "  {} {}: source path does not resolve: {} ({})",
                "x".red(),
                entry.name,
                entry.source.display(),
                e
            );
            bad += 1;
            continue;
        }
        let skill_md = entry.source.join("SKILL.md");
        if !skill_md.is_file() {
            eprintln!(
                "  {} {}: source path is missing SKILL.md: {}",
                "x".red(),
                entry.name,
                skill_md.display()
            );
            bad += 1;
            continue;
        }
        if actual != entry.source {
            eprintln!(
                "  {} {}: points to {} (expected {})",
                "x".red(),
                entry.name,
                actual.display(),
                entry.source.display()
            );
            bad += 1;
            continue;
        }
        println!("  {} {}", "ok".green(), entry.name);
    }
    if bad > 0 {
        bail!("{} skill(s) failed verification", bad);
    }
    Ok(())
}

#[cfg(unix)]
fn create_managed_link(source: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(source, target)?;
    Ok(())
}

#[cfg(windows)]
fn create_managed_link(source: &Path, target: &Path) -> Result<()> {
    match std::os::windows::fs::symlink_dir(source, target) {
        Ok(()) => Ok(()),
        Err(symlink_error) => create_windows_junction(source, target).with_context(|| {
            format!("Windows directory symlink failed ({symlink_error}); junction fallback failed")
        }),
    }
}

#[cfg(windows)]
fn create_windows_junction(source: &Path, target: &Path) -> Result<()> {
    use std::process::Command;

    let output = Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(target)
        .arg(source)
        .output()
        .context("failed to invoke Windows mklink junction fallback")?;

    if output.status.success() {
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    bail!(
        "mklink /J failed with status {}. stdout: {} stderr: {}",
        output.status,
        stdout.trim(),
        stderr.trim()
    )
}

/// Remove a managed link only if it still points at the source recorded in the
/// manifest. Returns Ok(true) if removed, Ok(false) if preserved (modified by
/// the user), Err on I/O failure.
fn remove_managed_link(entry: &Entry) -> Result<bool> {
    if !is_managed_link_path(&entry.target) {
        // Either user replaced it with a real file, or it was already removed.
        if !entry.target.exists() {
            return Ok(true); // already gone — count as removed
        }
        return Ok(false);
    }
    let actual = managed_link_target(&entry.target)?;
    if actual != entry.source {
        return Ok(false);
    }
    remove_managed_link_path(&entry.target)?;
    Ok(true)
}

fn managed_link_points_to(target: &Path, source: &Path) -> bool {
    managed_link_target(target)
        .map(|actual| actual == source)
        .unwrap_or(false)
}

fn managed_link_target(target: &Path) -> Result<PathBuf> {
    if !is_managed_link_path(target) {
        bail!("target is not a managed link");
    }

    if let Ok(canon) = fs::canonicalize(target) {
        return Ok(canon);
    }

    let link = fs::read_link(target)?;
    let resolved = if link.is_absolute() {
        link
    } else {
        target
            .parent()
            .map(|p| p.join(&link))
            .unwrap_or(link.clone())
    };
    Ok(fs::canonicalize(&resolved).unwrap_or(resolved))
}

fn is_managed_link_path(path: &Path) -> bool {
    path.is_symlink() || is_windows_reparse_point(path)
}

#[cfg(windows)]
fn is_windows_reparse_point(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    fs::symlink_metadata(path)
        .map(|metadata| metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0)
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn is_windows_reparse_point(_path: &Path) -> bool {
    false
}

#[cfg(unix)]
fn remove_managed_link_path(target: &Path) -> std::io::Result<()> {
    fs::remove_file(target)
}

#[cfg(windows)]
fn remove_managed_link_path(target: &Path) -> std::io::Result<()> {
    fs::remove_dir(target).or_else(|_| fs::remove_file(target))
}

fn validate_entry_target_in_host_root(entry: &Entry, root: &Path) -> Result<()> {
    if entry.target.starts_with(root) {
        return Ok(());
    }

    bail!(
        "target {} is outside the selected host root {}",
        entry.target.display(),
        root.display()
    )
}

fn print_plan(plan: &Plan, root: &Path) {
    println!("target root: {}", root.display());
    if !plan.to_create.is_empty() {
        println!("would create:");
        for i in &plan.to_create {
            println!("  + {} -> {}", i.target.display(), i.source.display());
        }
    }
    if !plan.to_update.is_empty() {
        println!("would update (requires --force):");
        for i in &plan.to_update {
            println!("  ~ {}", i.target.display());
        }
    }
    if !plan.up_to_date.is_empty() {
        println!("already up to date:");
        for i in &plan.up_to_date {
            println!("  = {}", i.name);
        }
    }
    if !plan.conflicts.is_empty() {
        println!("conflicts (non-link paths):");
        for i in &plan.conflicts {
            println!("  ! {}", i.target.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_skill(skills_dir: &Path, name: &str) -> PathBuf {
        let dir = skills_dir.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), "---\nname: test\n---\nbody").unwrap();
        dir
    }

    #[test]
    fn discover_skills_picks_up_dirs_with_skill_md() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("skills");
        fs::create_dir_all(&src).unwrap();
        make_skill(&src, "alpha");
        make_skill(&src, "beta");
        // A dir without SKILL.md should be ignored.
        fs::create_dir_all(src.join("no-skill-md")).unwrap();
        // A dotfile dir should be ignored.
        make_skill(&src, ".hidden");
        let mut found = discover_skills(&src).unwrap();
        let names: Vec<_> = found.drain(..).map(|(n, _)| n).collect();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn discover_skills_errors_on_missing_dir() {
        let res = discover_skills(Path::new("/nonexistent/never/ever"));
        assert!(res.is_err());
    }
}

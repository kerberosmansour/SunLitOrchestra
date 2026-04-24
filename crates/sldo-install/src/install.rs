//! Install, uninstall, status, and verify logic.
//!
//! Idempotency contract: calling `install` repeatedly with the same inputs
//! yields the same filesystem state. Calling `uninstall` reverses every
//! symlink and removes the manifest.
//!
//! Symlink safety: we never overwrite a non-symlink target. Overwriting an
//! existing symlink requires `--force`. Uninstall only removes symlinks that
//! point at the source recorded in the manifest — if a user has replaced a
//! symlink manually, we leave it alone and warn.

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::manifest::{Entry, Manifest};
use crate::paths;

pub struct Options {
    pub skills_dir: PathBuf,
    pub local: bool,
    pub force: bool,
    pub dry_run: bool,
}

/// Result of planning an install: what we'd do, without doing it.
#[derive(Debug, Default)]
struct Plan {
    to_create: Vec<PlanItem>,
    to_update: Vec<PlanItem>, // existing symlink pointing elsewhere, needs --force
    up_to_date: Vec<PlanItem>,
    conflicts: Vec<PlanItem>, // non-symlink target, cannot overwrite
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
        Ok((paths::local_skills_root(&cwd), paths::local_manifest_path(&cwd)))
    } else {
        let home = paths::home_dir()?;
        Ok((paths::global_skills_root(&home), paths::global_manifest_path(&home)))
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

        if target.exists() || target.is_symlink() {
            // is_symlink works even when the symlink is dangling.
            if target.is_symlink() {
                match fs::read_link(&target) {
                    Ok(link) => {
                        // Resolve relative symlinks against their parent to compare.
                        let resolved = if link.is_absolute() {
                            link
                        } else {
                            target
                                .parent()
                                .map(|p| p.join(&link))
                                .unwrap_or(link.clone())
                        };
                        let resolved_canon = fs::canonicalize(&resolved).unwrap_or(resolved);
                        if resolved_canon == source_abs {
                            plan.up_to_date.push(item);
                        } else {
                            plan.to_update.push(item);
                        }
                    }
                    Err(_) => plan.to_update.push(item),
                }
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
        eprintln!("{}", "conflicts (existing non-symlink paths):".red().bold());
        for item in &plan.conflicts {
            eprintln!("  {} -> {}", item.target.display(), item.source.display());
        }
        bail!(
            "{} skill(s) have non-symlink paths at their install targets. \
             Remove or relocate them and re-run.",
            plan.conflicts.len()
        );
    }

    if !plan.to_update.is_empty() && !opts.force {
        eprintln!(
            "{}",
            "the following symlinks point elsewhere and would be replaced by --force:"
                .yellow()
                .bold()
        );
        for item in &plan.to_update {
            eprintln!("  {}", item.target.display());
        }
        bail!(
            "{} symlink(s) already exist. Pass --force to overwrite.",
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
        // Remove pre-existing symlink (only symlinks reach this branch).
        if item.target.is_symlink() {
            fs::remove_file(&item.target).with_context(|| {
                format!("Failed to remove old symlink: {}", item.target.display())
            })?;
        }
        create_symlink(&item.source, &item.target).with_context(|| {
            format!(
                "Failed to symlink {} -> {}",
                item.target.display(),
                item.source.display()
            )
        })?;
        manifest.upsert(item.name.clone(), item.target.clone(), item.source.clone());
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
        if manifest.find(&item.name).is_none() {
            manifest.upsert(item.name.clone(), item.target.clone(), item.source.clone());
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
    let (_root, manifest_path) = compute_root_and_manifest(opts)?;
    let mut manifest = Manifest::load(&manifest_path)?;

    if manifest.entries.is_empty() {
        println!("nothing to uninstall (manifest is empty or missing).");
        return Ok(());
    }

    if opts.dry_run {
        println!("would remove:");
        for entry in &manifest.entries {
            println!("  - {}", entry.target.display());
        }
        println!("  manifest: {}", manifest_path.display());
        return Ok(());
    }

    let to_remove: Vec<Entry> = manifest.entries.clone();
    for entry in to_remove {
        match remove_managed_symlink(&entry) {
            Ok(true) => {
                println!("  {} {}", "-".yellow().bold(), entry.name);
                manifest.remove(&entry.name);
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
            }
        }
    }

    if manifest.entries.is_empty() {
        let _ = fs::remove_file(&manifest_path);
    } else {
        manifest.save(&manifest_path)?;
    }
    Ok(())
}

pub fn status(opts: &Options) -> Result<()> {
    let (_root, manifest_path) = compute_root_and_manifest(opts)?;
    let manifest = Manifest::load(&manifest_path)?;
    if manifest.entries.is_empty() {
        println!("no skills installed.");
        return Ok(());
    }
    println!("installed skills ({}):", manifest.entries.len());
    for e in &manifest.entries {
        println!("  {} -> {}  [installed {}]", e.name, e.source.display(), e.installed_at);
    }
    Ok(())
}

pub fn verify(opts: &Options) -> Result<()> {
    let (_root, manifest_path) = compute_root_and_manifest(opts)?;
    let manifest = Manifest::load(&manifest_path)?;
    if manifest.entries.is_empty() {
        println!("manifest is empty — nothing to verify.");
        return Ok(());
    }

    let mut bad = 0usize;
    for entry in &manifest.entries {
        if !entry.target.is_symlink() {
            eprintln!("  {} {}: target is not a symlink", "x".red(), entry.name);
            bad += 1;
            continue;
        }
        let link = match fs::read_link(&entry.target) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("  {} {}: read_link failed: {}", "x".red(), entry.name, e);
                bad += 1;
                continue;
            }
        };
        let resolved = if link.is_absolute() {
            link
        } else {
            entry
                .target
                .parent()
                .map(|p| p.join(&link))
                .unwrap_or(link.clone())
        };
        let canon = fs::canonicalize(&resolved).unwrap_or(resolved);
        if canon != entry.source {
            eprintln!(
                "  {} {}: points to {} (expected {})",
                "x".red(),
                entry.name,
                canon.display(),
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
fn create_symlink(source: &Path, target: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(source, target)
}

#[cfg(windows)]
fn create_symlink(source: &Path, target: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(source, target)
}

/// Remove a symlink only if it still points at the source recorded in the
/// manifest. Returns Ok(true) if removed, Ok(false) if preserved (modified by
/// the user), Err on I/O failure.
fn remove_managed_symlink(entry: &Entry) -> Result<bool> {
    if !entry.target.is_symlink() {
        // Either user replaced it with a real file, or it was already removed.
        if !entry.target.exists() {
            return Ok(true); // already gone — count as removed
        }
        return Ok(false);
    }
    let link = fs::read_link(&entry.target)?;
    let resolved = if link.is_absolute() {
        link
    } else {
        entry
            .target
            .parent()
            .map(|p| p.join(&link))
            .unwrap_or(link.clone())
    };
    let canon = fs::canonicalize(&resolved).unwrap_or(resolved);
    if canon != entry.source {
        return Ok(false);
    }
    fs::remove_file(&entry.target)?;
    Ok(true)
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
        println!("conflicts (non-symlink paths):");
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

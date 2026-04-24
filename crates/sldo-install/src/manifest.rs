//! Install manifest: records which skills are installed and where their
//! symlinks point, so `uninstall` can reverse exactly what `install` did.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// A single installed skill entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Skill directory name (e.g. `slo-ideate`, `get-api-docs`).
    pub name: String,
    /// Absolute path to the symlink we created.
    pub target: PathBuf,
    /// Absolute path to the source skill directory the symlink points at.
    pub source: PathBuf,
    /// ISO-8601 UTC timestamp of the install.
    pub installed_at: String,
}

/// The full manifest file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// Manifest schema version for forward compat.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    /// All installed skills.
    #[serde(default)]
    pub entries: Vec<Entry>,
}

fn default_schema_version() -> u32 {
    1
}

impl Manifest {
    /// Load a manifest from disk, returning an empty manifest when the file does not exist.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                schema_version: 1,
                entries: Vec::new(),
            });
        }
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest: {}", path.display()))?;
        let m: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest: {}", path.display()))?;
        Ok(m)
    }

    /// Persist the manifest to disk, creating parent directories.
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create manifest directory: {}", parent.display())
            })?;
        }
        let content = toml::to_string_pretty(self).context("Failed to serialize manifest")?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write manifest: {}", path.display()))?;
        Ok(())
    }

    /// Find an entry by skill name.
    pub fn find(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// Add or replace an entry, keyed by name. Timestamp is set now.
    pub fn upsert(&mut self, name: String, target: PathBuf, source: PathBuf) {
        let entry = Entry {
            name: name.clone(),
            target,
            source,
            installed_at: Utc::now().to_rfc3339(),
        };
        if let Some(existing) = self.entries.iter_mut().find(|e| e.name == name) {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }

    /// Remove an entry by name. Returns the removed entry if present.
    pub fn remove(&mut self, name: &str) -> Option<Entry> {
        if let Some(idx) = self.entries.iter().position(|e| e.name == name) {
            Some(self.entries.remove(idx))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_missing_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nope.toml");
        let m = Manifest::load(&path).unwrap();
        assert!(m.entries.is_empty());
        assert_eq!(m.schema_version, 1);
    }

    #[test]
    fn save_then_load_roundtrips() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sub").join("install.toml");
        let mut m = Manifest::default();
        m.schema_version = 1;
        m.upsert(
            "slo-ideate".into(),
            PathBuf::from("/home/u/.claude/skills/slo-ideate"),
            PathBuf::from("/repo/skills/slo-ideate"),
        );
        m.save(&path).unwrap();

        let loaded = Manifest::load(&path).unwrap();
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].name, "slo-ideate");
    }

    #[test]
    fn upsert_replaces_existing() {
        let mut m = Manifest::default();
        m.upsert("a".into(), PathBuf::from("/t1"), PathBuf::from("/s1"));
        m.upsert("a".into(), PathBuf::from("/t2"), PathBuf::from("/s2"));
        assert_eq!(m.entries.len(), 1);
        assert_eq!(m.entries[0].target, PathBuf::from("/t2"));
    }

    #[test]
    fn remove_returns_entry_and_deletes() {
        let mut m = Manifest::default();
        m.upsert("a".into(), PathBuf::from("/t"), PathBuf::from("/s"));
        let removed = m.remove("a");
        assert!(removed.is_some());
        assert!(m.entries.is_empty());
        assert!(m.remove("a").is_none());
    }
}

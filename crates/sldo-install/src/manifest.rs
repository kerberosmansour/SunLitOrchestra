//! Install manifest: records which skills are installed and where their
//! symlinks point, so `uninstall` can reverse exactly what `install` did.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::host::Host;

/// A single installed skill entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Skill directory name (e.g. `slo-ideate`, `get-api-docs`).
    pub name: String,
    /// Host id that owns this install (for example `claude-code`).
    #[serde(default = "default_host_id")]
    pub host: String,
    /// Absolute path to the symlink we created.
    pub target: PathBuf,
    /// Absolute path to the source skill directory the symlink points at.
    pub source: PathBuf,
    /// ISO-8601 UTC timestamp of the install.
    pub installed_at: String,
}

/// The full manifest file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// Manifest schema version for forward compat.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    /// All installed skills.
    #[serde(default)]
    pub entries: Vec<Entry>,
}

const CURRENT_SCHEMA_VERSION: u32 = 2;

fn default_schema_version() -> u32 {
    CURRENT_SCHEMA_VERSION
}

fn default_host_id() -> String {
    Host::ClaudeCode.id().to_string()
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            entries: Vec::new(),
        }
    }
}

impl Manifest {
    /// Load a manifest from disk, returning an empty manifest when the file does not exist.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest: {}", path.display()))?;
        let mut m: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest: {}", path.display()))?;
        if m.schema_version < CURRENT_SCHEMA_VERSION {
            m.schema_version = CURRENT_SCHEMA_VERSION;
        }
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

    /// Find an entry by skill name and host.
    pub fn find(&self, name: &str, host: Host) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| entry.name == name && entry.host == host.id())
    }

    /// Return the entries owned by a specific host.
    pub fn entries_for_host(&self, host: Host) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| entry.host == host.id())
            .collect()
    }

    /// Add or replace an entry, keyed by skill name + host. Timestamp is set now.
    pub fn upsert(&mut self, host: Host, name: String, target: PathBuf, source: PathBuf) {
        self.schema_version = CURRENT_SCHEMA_VERSION;
        let entry = Entry {
            name: name.clone(),
            host: host.id().to_string(),
            target,
            source,
            installed_at: Utc::now().to_rfc3339(),
        };
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|existing| existing.name == name && existing.host == host.id())
        {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }

    /// Remove an entry by name + host. Returns the removed entry if present.
    pub fn remove(&mut self, name: &str, host: Host) -> Option<Entry> {
        if let Some(idx) = self
            .entries
            .iter()
            .position(|entry| entry.name == name && entry.host == host.id())
        {
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
        assert_eq!(m.schema_version, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn save_then_load_roundtrips() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sub").join("install.toml");
        let mut m = Manifest::default();
        m.upsert(
            Host::ClaudeCode,
            "slo-ideate".into(),
            PathBuf::from("/home/u/.claude/skills/slo-ideate"),
            PathBuf::from("/repo/skills/slo-ideate"),
        );
        m.save(&path).unwrap();

        let loaded = Manifest::load(&path).unwrap();
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].name, "slo-ideate");
        assert_eq!(loaded.entries[0].host, "claude-code");
        assert_eq!(loaded.schema_version, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn upsert_replaces_existing_for_same_host_only() {
        let mut m = Manifest::default();
        m.upsert(
            Host::ClaudeCode,
            "a".into(),
            PathBuf::from("/t1"),
            PathBuf::from("/s1"),
        );
        m.upsert(
            Host::GithubCopilot,
            "a".into(),
            PathBuf::from("/t2"),
            PathBuf::from("/s2"),
        );
        m.upsert(
            Host::Codex,
            "a".into(),
            PathBuf::from("/t4"),
            PathBuf::from("/s4"),
        );
        m.upsert(
            Host::ClaudeCode,
            "a".into(),
            PathBuf::from("/t3"),
            PathBuf::from("/s3"),
        );

        assert_eq!(m.entries.len(), 3);
        assert_eq!(
            m.find("a", Host::ClaudeCode).unwrap().target,
            PathBuf::from("/t3")
        );
        assert_eq!(
            m.find("a", Host::GithubCopilot).unwrap().target,
            PathBuf::from("/t2")
        );
        assert_eq!(
            m.find("a", Host::Codex).unwrap().target,
            PathBuf::from("/t4")
        );
    }

    #[test]
    fn remove_returns_entry_and_deletes_only_selected_host() {
        let mut m = Manifest::default();
        m.upsert(
            Host::ClaudeCode,
            "a".into(),
            PathBuf::from("/claude"),
            PathBuf::from("/s1"),
        );
        m.upsert(
            Host::GithubCopilot,
            "a".into(),
            PathBuf::from("/copilot"),
            PathBuf::from("/s2"),
        );
        m.upsert(
            Host::Codex,
            "a".into(),
            PathBuf::from("/codex"),
            PathBuf::from("/s3"),
        );

        let removed = m.remove("a", Host::GithubCopilot);
        assert!(removed.is_some());
        assert_eq!(m.entries.len(), 2);
        assert!(m.find("a", Host::ClaudeCode).is_some());
        assert!(m.find("a", Host::Codex).is_some());
        assert!(m.remove("a", Host::GithubCopilot).is_none());
    }

    #[test]
    fn load_v1_manifest_defaults_entries_to_claude_code() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("install.toml");
        fs::write(
            &path,
            r#"schema_version = 1

[[entries]]
name = "slo-ideate"
target = "/home/u/.claude/skills/slo-ideate"
source = "/repo/skills/slo-ideate"
installed_at = "2026-01-01T00:00:00Z"
"#,
        )
        .unwrap();

        let loaded = Manifest::load(&path).unwrap();
        assert_eq!(loaded.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].host, "claude-code");
    }
}

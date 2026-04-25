//! Minimal Semgrep rule YAML schema for `check-coverage` arm counting.
//!
//! Strict-mode parse via `serde_yaml_ng`: rejects unknown fields, fails fast on
//! malformed input.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct RuleFile {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub id: String,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_yaml_ng::Value>,

    #[serde(default)]
    pub pattern: Option<serde_yaml_ng::Value>,
    #[serde(rename = "pattern-either", default)]
    pub pattern_either: Option<Vec<serde_yaml_ng::Value>>,
    #[serde(rename = "pattern-not-inside", default)]
    pub pattern_not_inside: Option<serde_yaml_ng::Value>,
    #[serde(rename = "pattern-inside", default)]
    pub pattern_inside: Option<serde_yaml_ng::Value>,
    #[serde(rename = "patterns", default)]
    pub patterns: Option<Vec<serde_yaml_ng::Value>>,

    #[serde(default)]
    pub mode: Option<String>,
    #[serde(rename = "pattern-sources", default)]
    pub pattern_sources: Option<Vec<serde_yaml_ng::Value>>,
    #[serde(rename = "pattern-sinks", default)]
    pub pattern_sinks: Option<Vec<serde_yaml_ng::Value>>,
    #[serde(rename = "pattern-sanitizers", default)]
    pub pattern_sanitizers: Option<Vec<serde_yaml_ng::Value>>,
    #[serde(rename = "fix", default)]
    pub fix: Option<String>,
    #[serde(rename = "fix-regex", default)]
    pub fix_regex: Option<serde_yaml_ng::Value>,
    #[serde(rename = "options", default)]
    pub options: Option<serde_yaml_ng::Value>,
    #[serde(rename = "paths", default)]
    pub paths: Option<serde_yaml_ng::Value>,
}

impl Rule {
    pub fn pattern_either_arm_count(&self) -> usize {
        self.pattern_either.as_ref().map(|v| v.len()).unwrap_or(0)
    }

    pub fn cwe_id(&self) -> Option<String> {
        let meta = self.metadata.as_ref()?;
        let cwe = meta.get("cwe")?.as_str()?;
        cwe.split(':').next().map(|s| s.trim().to_string())
    }
}

pub fn parse_file(path: &Path) -> Result<RuleFile> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("read rule file at {}", path.display()))?;
    let rule_file: RuleFile = serde_yaml_ng::from_str(&content)
        .with_context(|| format!("parse rule YAML at {}", path.display()))?;
    if rule_file.rules.is_empty() {
        return Err(anyhow!(
            "rule file {} has empty `rules:` list",
            path.display()
        ));
    }
    Ok(rule_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_rule(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
        let path = dir.path().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn parse_minimal_pattern_rule() {
        let dir = TempDir::new().unwrap();
        let path = write_rule(
            &dir,
            "r.yaml",
            "rules:\n  - id: test-rule\n    languages: [rust]\n    severity: WARNING\n    message: \"test\"\n    pattern: foo()\n",
        );
        let rf = parse_file(&path).unwrap();
        assert_eq!(rf.rules.len(), 1);
        assert_eq!(rf.rules[0].id, "test-rule");
        assert_eq!(rf.rules[0].pattern_either_arm_count(), 0);
    }

    #[test]
    fn parse_pattern_either_arms() {
        let dir = TempDir::new().unwrap();
        let path = write_rule(
            &dir,
            "r.yaml",
            "rules:\n  - id: pe-rule\n    languages: [rust]\n    severity: WARNING\n    message: \"test\"\n    pattern-either:\n      - pattern: foo()\n      - pattern: bar()\n      - pattern: baz()\n      - pattern: qux()\n",
        );
        let rf = parse_file(&path).unwrap();
        assert_eq!(rf.rules[0].pattern_either_arm_count(), 4);
    }

    #[test]
    fn parse_extracts_cwe_id() {
        let dir = TempDir::new().unwrap();
        let path = write_rule(
            &dir,
            "r.yaml",
            "rules:\n  - id: cwe-rule\n    languages: [rust]\n    severity: WARNING\n    message: \"test\"\n    pattern: foo()\n    metadata:\n      cwe: \"CWE-755: Improper Handling of Exceptional Conditions\"\n      category: security\n",
        );
        let rf = parse_file(&path).unwrap();
        assert_eq!(rf.rules[0].cwe_id().as_deref(), Some("CWE-755"));
    }

    #[test]
    fn rejects_unknown_field_strict() {
        let dir = TempDir::new().unwrap();
        let path = write_rule(
            &dir,
            "r.yaml",
            "rules:\n  - id: bad\n    languages: [rust]\n    severity: WARNING\n    message: \"test\"\n    pattern: foo()\n    bogus_field_attacker_supplied: \"value\"\n",
        );
        let err = parse_file(&path).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("bogus_field_attacker_supplied") || msg.contains("unknown field"),
            "expected unknown-field rejection; got: {msg}"
        );
    }

    #[test]
    fn rejects_empty_rules_list() {
        let dir = TempDir::new().unwrap();
        let path = write_rule(&dir, "r.yaml", "rules: []\n");
        let err = parse_file(&path).unwrap_err();
        assert!(format!("{err:#}").contains("empty"));
    }
}

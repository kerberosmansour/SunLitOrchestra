//! M4 structural-contract test (sap-imp runbook).
//!
//! Asserts the workflow-pinning + plugin-packaging invariants from M4:
//!
//! - F-ENG-5: every `uses:` in `.github/{workflows,actions}/**/*.{yml,yaml}` is
//!   SHA-pinned (40-char hex). Trigger-acceptable-set enumerated.
//! - F-SEC-3: `.claude-plugin/plugin.json` (if exists) contains no path-valued
//!   field with `..` segments or absolute paths.
//! - F-SEC-4: every workflow has a top-level explicit `permissions:` block.
//! - F-SEC-5: release workflow uses `git archive` (not tar/cp/zip of working dir).
//!
//! Decision doc invariants:
//!
//! - `docs/slo/design/host-capability-matrix.md` exists with capability table
//!   and a green-lit / not-green-lit / deferred decision row.

use regex::Regex;
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("skills").is_dir() && cwd.join("Cargo.toml").is_file() {
            return cwd;
        }
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("xtasks/sast-verify must live two levels below workspace root")
        .to_path_buf()
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

/// Walk all `.github/{workflows,actions}/**/*.{yml,yaml}` files. Returns
/// `(path, content)` pairs.
fn workflow_files() -> Vec<(PathBuf, String)> {
    let root = workspace_root();
    let mut out = Vec::new();
    for subdir in &["workflows", "actions"] {
        let dir = root.join(".github").join(subdir);
        if !dir.exists() {
            continue;
        }
        walk_yaml(&dir, &mut out);
    }
    out
}

fn walk_yaml(dir: &Path, acc: &mut Vec<(PathBuf, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.is_dir() {
            walk_yaml(&path, acc);
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name.ends_with(".yml") || name.ends_with(".yaml") {
            let content = read(&path);
            acc.push((path, content));
        }
    }
}

#[test]
fn every_workflow_uses_is_sha_pinned() {
    let workflows = workflow_files();
    let uses_re = Regex::new(r"^\s*-?\s*uses:\s*([a-zA-Z0-9._/-]+)@([a-zA-Z0-9._/-]+)").unwrap();
    let sha_re = Regex::new(r"^[a-f0-9]{40}$").unwrap();
    let mut failures: Vec<String> = Vec::new();

    for (path, content) in workflows {
        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = uses_re.captures(line) {
                let action = &caps[1];
                let r#ref = &caps[2];
                if !sha_re.is_match(r#ref) {
                    failures.push(format!(
                        "{}:{}  uses: {}@{} — expected 40-char SHA pin (per F-ENG-5)",
                        path.display(),
                        line_num + 1,
                        action,
                        r#ref
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "M4 SHA-pinning invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn every_workflow_has_explicit_permissions_block() {
    let workflows = workflow_files();
    let mut failures: Vec<String> = Vec::new();

    for (path, content) in workflows {
        // Skip composite-action.yml files (which use `runs:` instead of `permissions:` and `on:`).
        // Heuristic: composite actions live under `.github/actions/<name>/action.yml` and contain
        // a top-level `runs:` key.
        let is_composite = content
            .lines()
            .any(|l| l.trim_start().starts_with("runs:") && !l.contains("runs-on"));
        if is_composite {
            continue;
        }

        let has_permissions = content
            .lines()
            .any(|l| l.starts_with("permissions:") || l.starts_with("permissions :"));
        if !has_permissions {
            failures.push(format!(
                "{} lacks a top-level `permissions:` block (per F-SEC-4)",
                path.display()
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "M4 permissions-block invariant violated:\n  - {}",
        failures.join("\n  - ")
    );
}

#[test]
fn host_capability_matrix_exists_with_decision() {
    let path = workspace_root().join("docs/slo/design/host-capability-matrix.md");
    assert!(
        path.exists(),
        "docs/slo/design/host-capability-matrix.md missing — M4 requires it"
    );
    let content = read(&path);
    let has_matrix = content.contains("## Matrix") || content.contains("# Host capability matrix");
    assert!(has_matrix, "matrix section missing");
    let has_decision = content.contains("`green-lit`")
        || content.contains("`not green-lit`")
        || content.contains("`deferred`");
    assert!(
        has_decision,
        "decision row missing — must contain `green-lit` / `not green-lit` / `deferred`"
    );
}

#[test]
fn plugin_json_does_not_duplicate_skills_or_traverse_paths() {
    let path = workspace_root().join(".claude-plugin/plugin.json");
    if !path.exists() {
        return; // not green-lit branch — vacuous-pass
    }
    let content = read(&path);
    let v: JsonValue = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!(".claude-plugin/plugin.json parse error: {}", e));

    // Recursively walk the JSON for any string value that looks like a path.
    let mut violations: Vec<String> = Vec::new();
    walk_json_for_paths(&v, "$", &mut violations);

    assert!(
        violations.is_empty(),
        "M4 plugin.json path-safety invariant violated:\n  - {}",
        violations.join("\n  - ")
    );
}

fn walk_json_for_paths(v: &JsonValue, ptr: &str, violations: &mut Vec<String>) {
    match v {
        JsonValue::String(s) => {
            // Any string containing `..` segments or starting with `/`, `\`,
            // `.claude-plugin/skills/` is suspect.
            if s.split(['/', '\\']).any(|seg| seg == "..") {
                violations.push(format!("{} contains `..` traversal segment (`{}`)", ptr, s));
            }
            if Path::new(s).is_absolute() {
                violations.push(format!("{} is absolute (`{}`)", ptr, s));
            }
            if s.contains(".claude-plugin/skills/") {
                violations.push(format!(
                    "{} duplicates skills under `.claude-plugin/skills/` (`{}`)",
                    ptr, s
                ));
            }
        }
        JsonValue::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                walk_json_for_paths(item, &format!("{}[{}]", ptr, i), violations);
            }
        }
        JsonValue::Object(map) => {
            for (k, vv) in map {
                walk_json_for_paths(vv, &format!("{}.{}", ptr, k), violations);
            }
        }
        _ => {}
    }
}

#[test]
fn release_workflow_trigger_in_acceptable_set() {
    let path = workspace_root().join(".github/workflows/release-zip.yml");
    if !path.exists() {
        return; // not green-lit branch — vacuous-pass
    }
    let content = read(&path);

    // Acceptable triggers: tags:, release:, workflow_dispatch:, schedule:.
    // Forbidden: push: with default branches, pull_request:.
    let has_tags =
        content.contains("tags:") || content.contains("- 'v*'") || content.contains("v*");
    let has_release = content.contains("release:");
    let has_dispatch = content.contains("workflow_dispatch:");
    let has_schedule = content.contains("schedule:");
    let acceptable = has_tags || has_release || has_dispatch || has_schedule;
    assert!(
        acceptable,
        "release workflow has no trigger from acceptable set {{tags, release, workflow_dispatch, schedule}}"
    );

    // Forbidden: pull_request: (release flow should never be PR-triggered).
    assert!(
        !content.contains("pull_request:"),
        "release workflow uses pull_request: trigger — forbidden per F-ENG-5"
    );
}

#[test]
fn release_workflow_uses_git_archive() {
    let path = workspace_root().join(".github/workflows/release-zip.yml");
    if !path.exists() {
        return; // not green-lit branch — vacuous-pass
    }
    let content = read(&path);

    assert!(
        content.contains("git archive"),
        "release workflow does not use `git archive` (per F-SEC-5 — emit only tracked files)"
    );

    // Forbidden patterns (full-working-dir capture).
    let forbidden = ["tar -czf .", "tar -czvf .", "cp -r .", "zip -r . ."];
    for pat in forbidden {
        assert!(
            !content.contains(pat),
            "release workflow contains forbidden pattern `{}` — emit only tracked files via git archive (per F-SEC-5)",
            pat
        );
    }
}

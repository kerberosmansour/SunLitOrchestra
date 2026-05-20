//! M1 structural-contract tests for `/slo-sec-libs`.
//!
//! These tests assert the static reader contract. Runtime smoke checks are
//! performed manually against fixture declaration files.

use std::fs;
use std::path::{Path, PathBuf};

use sldo_common::toolflags;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn skill_path() -> PathBuf {
    repo_root().join("skills/slo-sec-libs")
}

#[test]
fn slo_sec_libs_skill_exists() {
    let root = skill_path();
    assert!(root.join("SKILL.md").is_file());
    assert!(root.join("scripts/read-declarations.py").is_file());
    assert!(root.join("references/methodology-m1-reader.md").is_file());
}

#[test]
fn skill_md_has_frontmatter() {
    let body = read(&skill_path().join("SKILL.md"));
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-sec-libs"));
    assert!(body.contains("description:"));
    assert!(body.contains("declarations-reader-only"));
}

#[test]
fn sec_libs_deny_flags_returns_webfetch_websearch() {
    let flags = toolflags::sec_libs_deny_flags();
    let combined = flags.join(",");
    assert!(combined.contains("WebFetch"));
    assert!(combined.contains("WebSearch"));
}

#[test]
fn python_script_imports_are_limited() {
    let script = read(&skill_path().join("scripts/read-declarations.py"));
    let allowed = [
        "argparse",
        "hashlib",
        "json",
        "os",
        "pathlib",
        "re",
        "subprocess",
        "sys",
        "unicodedata",
        "jsonschema",
    ];

    for line in script.lines() {
        let trimmed = line.trim();
        let module = if let Some(rest) = trimmed.strip_prefix("import ") {
            rest.split_whitespace().next()
        } else if let Some(rest) = trimmed.strip_prefix("from ") {
            rest.split_whitespace().next()
        } else {
            None
        };
        if let Some(module) = module {
            let root = module.split('.').next().unwrap();
            assert!(
                allowed.contains(&root),
                "read-declarations.py imports disallowed module `{root}` in line `{trimmed}`"
            );
        }
    }

    assert!(!script.contains("requests"));
    assert!(!script.contains("urllib"));
}

#[test]
fn argv_list_discipline_documented() {
    let skill = read(&skill_path().join("SKILL.md"));
    let methodology = read(&skill_path().join("references/methodology-m1-reader.md"));
    assert!(skill.contains("argv-list"));
    assert!(methodology.contains("argv-list"));
    assert!(skill.contains("subprocess.run") || methodology.contains("subprocess"));
}

#[test]
fn ten_mib_cap_documented_and_enforced() {
    let script = read(&skill_path().join("scripts/read-declarations.py"));
    let methodology = read(&skill_path().join("references/methodology-m1-reader.md"));
    assert!(script.contains("10 * 1024 * 1024"));
    assert!(methodology.contains("10 MiB"));
}

#[test]
fn symlink_check_runs_before_path_resolution() {
    let script = read(&skill_path().join("scripts/read-declarations.py"));
    let declaration_check = script
        .find("assert_no_symlink_path(declarations_arg_path)")
        .expect("declaration symlink check missing");
    let declaration_resolve = script
        .find("declarations_arg_path.resolve(strict=True)")
        .expect("declaration resolve missing");
    assert!(declaration_check < declaration_resolve);

    let schema_check = script
        .find("assert_no_symlink_path(schema_arg_path)")
        .expect("schema symlink check missing");
    let schema_resolve = script
        .find("schema_arg_path.resolve(strict=True)")
        .expect("schema resolve missing");
    assert!(schema_check < schema_resolve);
}

#[test]
fn strict_jsonschema_documented() {
    let script = read(&skill_path().join("scripts/read-declarations.py"));
    let methodology = read(&skill_path().join("references/methodology-m1-reader.md"));
    assert!(script.contains("jsonschema.Draft7Validator"));
    assert!(script.contains("--schema-path"));
    assert!(methodology.contains("strict jsonschema"));
}

#[test]
fn schema_url_and_sha_are_captured() {
    let script = read(&skill_path().join("scripts/read-declarations.py"));
    let methodology = read(&skill_path().join("references/methodology-m1-reader.md"));
    assert!(script.contains("https://cyclonedx.org/schema/bom-1.6.schema.json"));
    assert!(methodology.contains("https://cyclonedx.org/schema/bom-1.6.schema.json"));
    assert!(script.contains("1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f"));
    assert!(
        methodology.contains("1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f")
    );
}

#[test]
fn cache_discipline_documented() {
    let methodology = read(&skill_path().join("references/methodology-m1-reader.md"));
    assert!(methodology.contains("~/.cache/sldo/declarations/"));
    assert!(methodology.contains("git -C <cache-root> rev-parse HEAD"));
    assert!(methodology.contains("1 GiB"));
    assert!(methodology.contains("90 days"));
    assert!(methodology.contains("LRU"));
}

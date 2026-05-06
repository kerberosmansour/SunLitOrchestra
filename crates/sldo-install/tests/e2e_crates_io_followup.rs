//! Structural tests for the crates.io README refresh follow-up.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn crate_manifest(crate_name: &str) -> String {
    read(
        repo_root()
            .join("crates")
            .join(crate_name)
            .join("Cargo.toml"),
    )
}

fn crate_readme(crate_name: &str) -> String {
    read(
        repo_root()
            .join("crates")
            .join(crate_name)
            .join("README.md"),
    )
}

#[test]
fn published_crates_have_local_readmes() {
    for crate_name in ["sldo-common", "sldo-install", "sldo-research"] {
        let manifest = crate_manifest(crate_name);
        let readme = crate_readme(crate_name);

        assert!(
            manifest.contains("readme = \"README.md\""),
            "{crate_name} must publish its crate-local README"
        );
        assert!(
            !manifest.contains("readme = \"../../README.md\""),
            "{crate_name} must not publish the workspace README"
        );
        assert!(
            readme.starts_with(&format!("# {crate_name}\n")),
            "{crate_name} README must be crate-specific"
        );
        assert!(
            readme.contains(&format!("https://crates.io/crates/{crate_name}")),
            "{crate_name} README should link to its crates.io listing"
        );
    }
}

#[test]
fn publish_prep_bumps_workspace_and_internal_dependency_versions() {
    let root_manifest = read(repo_root().join("Cargo.toml"));
    assert!(root_manifest.contains("[workspace.package]\nversion = \"0.1.1\""));

    for crate_name in ["sldo-common", "sldo-install", "sldo-research"] {
        assert!(
            crate_manifest(crate_name).contains("version.workspace = true"),
            "{crate_name} should inherit the workspace package version"
        );
    }

    for crate_name in ["sldo-install", "sldo-research"] {
        assert!(
            crate_manifest(crate_name)
                .contains("sldo-common = { path = \"../sldo-common\", version = \"0.1.1\" }"),
            "{crate_name} should depend on the publish-ready sldo-common version"
        );
    }
}

#[test]
fn publish_metadata_uses_noreply_author() {
    let root_manifest = read(repo_root().join("Cargo.toml"));
    assert!(root_manifest
        .contains("Sherif Mansour <13433538+kerberosmansour@users.noreply.github.com>"));
    assert!(!root_manifest.contains("Sherif Mansour <cherifmansour@gmail.com>"));
}

#[test]
fn mailmap_maps_historical_email_to_noreply_alias() {
    let mailmap = read(repo_root().join(".mailmap"));
    assert!(mailmap.contains(
        "kerberosmansour <13433538+kerberosmansour@users.noreply.github.com> <cherifmansour@gmail.com>"
    ));
    assert!(mailmap.contains(
        "kerberosmansour <13433538+kerberosmansour@users.noreply.github.com> Kerberosmansour <13433538+kerberosmansour@users.noreply.github.com>"
    ));
}

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read(rel_path: &str) -> String {
    fs::read_to_string(repo_root().join(rel_path))
        .unwrap_or_else(|_| panic!("missing required file: {rel_path}"))
}

#[test]
fn test_living_docs_distinguish_catalog_from_overlays() {
    let catalog = read("docs/skill-pack-catalog.md");
    let claude = read("CLAUDE.md");
    let copilot = read("copilot-instructions.md");

    assert!(
        catalog.contains("canonical living catalog"),
        "catalog must identify itself as the canonical living catalog"
    );
    assert!(
        catalog.contains("CLAUDE.md") && catalog.contains("copilot-instructions.md"),
        "catalog must point readers to both host overlays"
    );
    assert!(
        claude.contains("Claude Code overlay") && claude.contains("docs/skill-pack-catalog.md"),
        "CLAUDE.md must identify itself as the Claude overlay and point back to the canonical catalog"
    );
    assert!(
        copilot.contains("GitHub Copilot overlay")
            && copilot.contains("docs/skill-pack-catalog.md"),
        "copilot overlay must identify itself as an overlay and point back to the canonical catalog"
    );
}

#[test]
fn test_capability_matrix_marks_headless_copilot_as_unsupported() {
    let capability = read("docs/design/agent-host-capabilities.md");

    assert!(
        capability.contains("Headless runtime automation"),
        "capability matrix must cover headless runtime automation"
    );
    assert!(
        capability.contains("GitHub Copilot") && capability.contains("Not supported yet"),
        "capability matrix must say headless GitHub Copilot automation is not supported yet"
    );
    assert!(
        capability.contains("interactive") || capability.contains("Interactive"),
        "capability matrix should distinguish interactive support from headless automation"
    );
}

#[test]
fn test_readme_links_to_getting_started_and_getting_started_has_first_run_sections() {
    let readme = read("README.md");
    let guide = read("docs/getting-started.md");

    assert!(
        readme.contains("docs/getting-started.md"),
        "README must link to the getting-started guide"
    );

    for heading in [
        "## Prerequisites",
        "## Install the skill pack",
        "## Run your first skill",
        "## Troubleshooting",
    ] {
        assert!(
            guide.contains(heading),
            "getting-started guide must contain heading: {heading}"
        );
    }
}

#[test]
fn test_architecture_doc_matches_surviving_workspace_members() {
    let readme = read("README.md");
    let architecture = read("docs/ARCHITECTURE.md");

    assert!(
        readme.contains("sldo-common")
            && readme.contains("sldo-research")
            && readme.contains("sldo-install")
            && readme.contains("xtasks/sast-verify"),
        "README must describe the surviving workspace members"
    );
    assert!(
        architecture.contains("sldo-common")
            && architecture.contains("sldo-research")
            && architecture.contains("sldo-install")
            && architecture.contains("xtasks/sast-verify"),
        "ARCHITECTURE.md must describe the surviving workspace members"
    );
    assert!(
        !readme.contains("CLI tools -- `sldo-plan`") && !architecture.contains("CLI tools -- `sldo-plan`"),
        "living docs must not describe removed CLI tools as active interfaces"
    );
}
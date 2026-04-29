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
fn test_slo_research_skill_no_longer_requires_batch_backend_for_interactive_flow() {
    let skill = read("skills/slo-research/SKILL.md");

    assert!(
        skill.contains("host-native research tools"),
        "interactive /slo-research guidance must describe a host-native research path"
    );
    assert!(
        skill.contains("optional Claude batch backend"),
        "skill must separate the optional Claude batch backend from the interactive path"
    );
    assert!(
        !skill.contains("You have one tool: the existing `sldo-research` Rust binary."),
        "interactive skill must not claim that sldo-research is the only tool"
    );
    assert!(
        !skill.contains("Shell out: `sldo-research"),
        "interactive skill must not mandate shelling out to sldo-research"
    );
}

#[test]
fn test_batch_backend_is_marked_optional_and_claude_specific() {
    let readme = read("README.md");
    let claude = read("CLAUDE.md");
    let copilot = read("copilot-instructions.md");
    let catalog = read("docs/skill-pack-catalog.md");
    let architecture = read("docs/ARCHITECTURE.md");

    for (label, contents) in [
        ("README", readme.as_str()),
        ("CLAUDE", claude.as_str()),
        ("Copilot overlay", copilot.as_str()),
        ("catalog", catalog.as_str()),
        ("architecture", architecture.as_str()),
    ] {
        assert!(
            contents.contains("optional Claude batch backend"),
            "{label} must call sldo-research an optional Claude batch backend"
        );
    }

    assert!(
        readme.contains("host-native research"),
        "README must describe /slo-research as host-native first"
    );
    assert!(
        catalog
            .to_lowercase()
            .contains("host-native interactive research"),
        "catalog must describe the interactive path as host-native first"
    );
    assert!(
        copilot.contains("without installing Claude"),
        "Copilot overlay must make the no-hidden-Claude interactive path explicit"
    );
}

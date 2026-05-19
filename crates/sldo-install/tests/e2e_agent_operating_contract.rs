//! Structural guards for the host-portable agent operating contract.
//!
//! The contract is intentionally small always-on context. Detailed procedures
//! stay in SLO skills and host-specific overlays.

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn read(rel_path: &str) -> String {
    fs::read_to_string(repo_root().join(rel_path))
        .unwrap_or_else(|_| panic!("missing required file: {rel_path}"))
}

fn nonblank_line_count(contents: &str) -> usize {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn frontmatter<'a>(contents: &'a str, rel_path: &str) -> &'a str {
    let without_open = contents
        .strip_prefix("---\n")
        .unwrap_or_else(|| panic!("{} must start with YAML frontmatter", rel_path));
    let close_pos = without_open
        .find("\n---")
        .unwrap_or_else(|| panic!("{} must close YAML frontmatter", rel_path));
    &without_open[..close_pos]
}

#[test]
fn operating_contract_exists_and_stays_small() {
    let contract = read("references/agent/operating-contract.md");

    assert!(
        contract.contains("# Agent Operating Contract"),
        "shared agent operating contract must identify itself"
    );
    for required in [
        "Ask When Ambiguous",
        "Smallest Safe Change",
        "Respect The Allow-List",
        "Verify Before Claiming",
        "Keep Host Boundaries Honest",
    ] {
        assert!(
            contract.contains(required),
            "operating contract missing required rule: {required}"
        );
    }
    assert!(
        nonblank_line_count(&contract) <= 120,
        "operating contract is too large for always-on agent context"
    );
}

#[test]
fn host_overlays_point_to_the_shared_contract() {
    for path in [
        "CLAUDE.md",
        "AGENTS.md",
        "copilot-instructions.md",
        ".github/copilot-instructions.md",
    ] {
        let contents = read(path);
        assert!(
            contents.contains("references/agent/operating-contract.md"),
            "{path} must point to the shared operating contract"
        );
        assert!(
            contents.contains("docs/skill-pack-catalog.md"),
            "{path} must point back to the canonical skill catalog"
        );
    }
}

#[test]
fn github_copilot_has_repo_wide_instruction_path() {
    let copilot = read(".github/copilot-instructions.md");

    assert!(
        copilot.contains("GitHub Copilot"),
        ".github/copilot-instructions.md must identify the Copilot host"
    );
    assert!(
        copilot.contains("copilot-instructions.md"),
        ".github/copilot-instructions.md must preserve the root Copilot overlay as the detailed companion"
    );
    assert!(
        copilot.contains("docs/slo/design/agent-host-capabilities.md"),
        "Copilot repo-wide instructions must route host-support claims through the capability matrix"
    );
    assert!(
        nonblank_line_count(&copilot) <= 120,
        ".github/copilot-instructions.md is too large for repo-wide always-on context"
    );
}

#[test]
fn living_docs_record_the_contract_as_a_shared_invariant() {
    let architecture = read("docs/ARCHITECTURE.md");
    let catalog = read("docs/skill-pack-catalog.md");

    for (label, contents) in [
        ("architecture", architecture.as_str()),
        ("skill catalog", catalog.as_str()),
    ] {
        assert!(
            contents.contains("references/agent/operating-contract.md"),
            "{label} must record the shared agent operating contract"
        );
    }
}

#[test]
fn m2_capability_docs_distinguish_official_roots_from_slo_compatibility_roots() {
    let capabilities = read("docs/slo/design/agent-host-capabilities.md");
    let matrix = read("docs/slo/design/host-capability-matrix.md");
    let combined = format!("{capabilities}\n{matrix}");

    for official_root in [".github/skills", ".agents/skills", ".github/agents"] {
        assert!(
            combined.contains(official_root),
            "capability docs must mention official host-native root `{official_root}`"
        );
    }
    for compatibility_root in [".copilot/skills", ".codex/skills"] {
        assert!(
            combined.contains(compatibility_root),
            "capability docs must preserve SLO installer compatibility root `{compatibility_root}`"
        );
    }
    assert!(
        combined.contains("compatibility root"),
        "capability docs must explicitly distinguish SLO installer compatibility roots from official host-native roots"
    );
    assert!(
        combined.contains("2026-05-19"),
        "capability docs must carry the M2 source refresh date"
    );
    assert!(
        combined.contains("No Copilot or Codex runtime harness is shipped today"),
        "capability docs must preserve the no-Copilot/no-Codex SLO runtime harness boundary"
    );
}

#[test]
fn m2_onboarding_docs_preserve_existing_installer_roots() {
    for path in [
        "README.md",
        "docs/getting-started.md",
        "skills/README.md",
        "crates/sldo-install/README.md",
    ] {
        let contents = read(path);
        assert!(
            contents.contains(".copilot/skills") && contents.contains(".codex/skills"),
            "{path} must keep existing SLO installer roots documented"
        );
        assert!(
            contents.contains("compatibility root"),
            "{path} must call those roots compatibility roots so readers do not confuse them with official project-skill roots"
        );
    }
}

#[test]
fn m3_copilot_custom_agent_profiles_exist_and_are_bounded() {
    for (name, expected_tools, portable_path) in [
        (
            "slo-runbook-review-lead",
            r#"tools: ["read", "search", "edit", "agent"]"#,
            "/slo-critique",
        ),
        (
            "slo-security-reviewer",
            r#"tools: ["read", "search"]"#,
            "/slo-critique",
        ),
        (
            "slo-design-reviewer",
            r#"tools: ["read", "search"]"#,
            "/slo-critique",
        ),
        (
            "slo-verification-lead",
            r#"tools: ["read", "search", "execute", "edit"]"#,
            "/slo-verify",
        ),
    ] {
        let rel_path = format!(".github/agents/{name}.agent.md");
        let contents = read(&rel_path);
        let fm = frontmatter(&contents, &rel_path);

        assert!(
            fm.contains(&format!("name: {name}")),
            "{rel_path} must keep the SLO role name"
        );
        assert!(
            fm.contains("description:"),
            "{rel_path} must include GitHub's required description field"
        );
        assert!(
            fm.contains("target: github-copilot"),
            "{rel_path} must be scoped to GitHub Copilot"
        );
        assert!(
            fm.contains(expected_tools),
            "{rel_path} must use the bounded tool set `{expected_tools}`"
        );
        assert!(
            contents.contains(portable_path) && contents.contains("canonical portable path"),
            "{rel_path} must point non-Copilot users to the canonical portable path"
        );
        assert!(
            contents.contains("not a SLO headless runtime harness"),
            "{rel_path} must not imply Copilot runtime-harness parity"
        );
        assert!(
            !contents.contains("../"),
            "{rel_path} must not contain path traversal fragments"
        );
        assert!(
            nonblank_line_count(&contents) <= 180,
            "{rel_path} is too large for a bounded custom-agent prompt"
        );
    }
}

#[test]
fn m3_copilot_profiles_preserve_review_output_boundaries() {
    let lead = read(".github/agents/slo-runbook-review-lead.agent.md");
    assert!(
        lead.contains("docs/slo/critique/<runbook-slug>.md"),
        "runbook review lead must write only the consolidated critique artifact"
    );
    assert!(
        lead.contains("Do not edit `skills/`"),
        "runbook review lead must not edit canonical skill prose"
    );

    for reviewer in ["slo-security-reviewer", "slo-design-reviewer"] {
        let rel_path = format!(".github/agents/{reviewer}.agent.md");
        let contents = read(&rel_path);
        let fm = frontmatter(&contents, &rel_path);
        assert!(
            fm.contains(r#"tools: ["read", "search"]"#),
            "{rel_path} must be read/search-only"
        );
        assert!(
            contents.contains("Do not write files"),
            "{rel_path} must return findings instead of writing artifacts directly"
        );
    }

    let verification = read(".github/agents/slo-verification-lead.agent.md");
    assert!(
        verification.contains("docs/slo/verify/<prefix>-m<N>.md"),
        "verification lead must write only the bounded verification report path"
    );
    assert!(
        verification.contains("DAST is N/A unless the runbook declares a runnable smoke service"),
        "verification lead must preserve the DAST smoke-service gate"
    );
}

#[test]
fn m3_capability_docs_record_profiles_without_runtime_parity_claims() {
    let capabilities = read("docs/slo/design/agent-host-capabilities.md");
    let matrix = read("docs/slo/design/host-capability-matrix.md");
    let combined = format!("{capabilities}\n{matrix}");

    for profile in [
        ".github/agents/slo-runbook-review-lead.agent.md",
        ".github/agents/slo-security-reviewer.agent.md",
        ".github/agents/slo-design-reviewer.agent.md",
        ".github/agents/slo-verification-lead.agent.md",
    ] {
        assert!(
            combined.contains(profile),
            "capability docs must name shipped Copilot profile `{profile}`"
        );
    }
    assert!(
        combined.contains("Codex") && combined.contains("no shipped SLO host-native custom-agent"),
        "capability docs must keep Codex on the portable fallback path"
    );
    assert!(
        combined.contains("No Copilot or Codex runtime harness is shipped today"),
        "capability docs must preserve the no-Copilot/no-Codex runtime harness boundary"
    );
}

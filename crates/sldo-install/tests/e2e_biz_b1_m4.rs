//! M4 structural-contract tests for biz-skill-pack Runbook B1.
//! Verifies /slo-marketing SKILL.md structure + b2b/b2c mode_arg + PECR routing + CLAUDE.md catalog edit.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const FOUR_PREDICATE_IDS: &[&str] = &[
    "gate-1-regulated",
    "gate-2-deal-value-over-5k",
    "gate-3-counterparty-has-lawyer-or-their-paper",
    "gate-4-gdpr-document",
];

const TWO_MODES: &[&str] = &["b2b", "b2c"];

const ALL_B1_GENERATORS: &[&str] = &[
    "slo-talk-to-users",
    "slo-gtm",
    "slo-product",
    "slo-marketing",
];

#[test]
fn slo_marketing_skill_md_has_required_frontmatter() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    assert!(skill.starts_with("---\n"));
    let after_open = &skill[4..];
    let close_idx = after_open.find("\n---\n").expect("frontmatter must close");
    let frontmatter = &after_open[..close_idx];
    assert!(frontmatter.contains("name: slo-marketing"));
    assert!(frontmatter.contains("description:"));
}

#[test]
fn slo_marketing_is_generator_archetype() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    assert!(skill.contains("Generator with") || skill.contains("Generator pattern") || skill.contains("archetype: generator"));
    // Generator: max one mention per advisor predicate id (allowing the explicit
    // gate-4 routing reference to count).
    for pid in FOUR_PREDICATE_IDS {
        let count = skill.matches(pid).count();
        assert!(count <= 2, "/slo-marketing cites `{pid}` {count} times; generator should reference advisor predicates ≤ 2 times (allowing explicit PECR / gate-4 routing)");
    }
}

#[test]
fn slo_marketing_documents_two_mode_args() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    for m in TWO_MODES {
        assert!(skill.contains(m), "/slo-marketing must enumerate mode `{m}`");
    }
    let output_paths = ["docs/biz-public/marketing/b2b-plan.md", "docs/biz-public/marketing/b2c-plan.md"];
    for p in &output_paths {
        assert!(skill.contains(p), "/slo-marketing must declare output path `{p}`");
    }
}

#[test]
fn slo_marketing_routes_direct_marketing_to_legal_triage() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    assert!(skill.contains("/slo-legal triage") || skill.contains("slo-legal triage"));
    assert!(skill.contains("PECR"));
    assert!(skill.contains("DUAA"));
    // Reference to ICO DUAA index
    assert!(skill.contains("ico-duaa-index.md"));
}

#[test]
fn slo_marketing_documents_asa_disclosure() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    let signals = ["ASA", "Advertising Standards", "#ad", "CAP Code"];
    let count = signals.iter().filter(|s| skill.contains(**s)).count();
    assert!(count >= 2, "/slo-marketing b2c must reference ASA disclosure / CAP Code (found {count})");
}

#[test]
fn slo_marketing_has_uk_only_error() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    assert!(skill.contains("v1 supports UK only"));
}

#[test]
fn slo_marketing_unknown_mode_rejection() {
    let skill = read(&repo_root().join("skills/slo-marketing/SKILL.md"));
    let signals = ["Unknown mode_arg", "unknown mode_arg"];
    let any = signals.iter().any(|s| skill.contains(s));
    assert!(any, "/slo-marketing must document rejection of unknown mode_arg");
}

#[test]
fn claude_md_catalogs_all_b1_generators() {
    let claude_md = read(&repo_root().join("CLAUDE.md"));
    for skill_name in ALL_B1_GENERATORS {
        let verb = format!("/{skill_name}");
        assert!(
            claude_md.contains(&verb),
            "CLAUDE.md must catalog B1 generator skill verb `{verb}` (M4 catalog edit)"
        );
    }
}

#[test]
fn references_biz_dir_still_not_discovered_after_b1_m4() {
    let skills_dir = repo_root().join("skills");
    assert!(skills_dir.is_dir());
    assert!(!skills_dir.join("biz").exists());
    assert!(!skills_dir.join("_biz-shared").exists());
    for skill_name in ALL_B1_GENERATORS {
        assert!(skills_dir.join(skill_name).join("SKILL.md").exists(), "skills/{skill_name}/SKILL.md must exist");
    }
}

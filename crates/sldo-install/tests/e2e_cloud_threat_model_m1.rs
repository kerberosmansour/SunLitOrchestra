// Copyright 2026 Sherif Mansour and SunLit Orchestra contributors.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
//! Structural-contract tests for the /slo-cloud-threat-model skill.
//!
//! These assert the static skill contract: file layout, frontmatter, the
//! offline-helper safety discipline, the bundled scenario catalog (AWS +
//! GitHub + Cloudflare + cross-provider), the IDs-only licensing boundary,
//! the seven canonical evals, and catalog registration. Runtime validation is
//! exercised separately by running `scenario_catalog.py validate`.

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

fn skill_path() -> PathBuf {
    repo_root().join("skills/slo-cloud-threat-model")
}

/// The declared, stable scenario order. Mirrors `SCENARIO_ORDER` in
/// scenario_catalog.py — the order is a stability contract.
const SCENARIO_ORDER: &[&str] = &[
    "aws-multi-account-baseline",
    "s3-public-bucket-hardening",
    "iam-least-privilege",
    "rds-encryption-at-rest",
    "lambda-secrets-access",
    "github-oidc-trust-cloud-account",
    "github-actions-supply-chain",
    "github-app-token-exposure",
    "github-self-hosted-runner",
    "cloudflare-zone-and-dns-foundation",
    "cloudflare-edge-waf-and-bot-protection",
    "cloudflare-origin-bypass-prevention",
    "cloudflare-protected-admin-access",
    "github-aws-oidc-deployment-identity",
];

#[test]
fn skill_files_exist() {
    let root = skill_path();
    for rel in [
        "SKILL.md",
        "scripts/scenario_catalog.py",
        "references/methodology.md",
        "references/citation-and-licensing.md",
        "references/threat-model-template.md",
        "references/scenario-catalog.md",
    ] {
        assert!(root.join(rel).is_file(), "missing {rel}");
    }
}

#[test]
fn skill_md_has_frontmatter_and_ids_only_contract() {
    let body = read(skill_path().join("SKILL.md"));
    assert!(body.starts_with("---\n"));
    assert!(body.contains("name: slo-cloud-threat-model"));
    assert!(body.contains("description:"));
    // IDs-only licensing boundary + refusal + CSA FAQ link.
    assert!(body.contains("identifiers and URLs only") || body.contains("identifier only"));
    assert!(body.contains("refuse"));
    assert!(body.contains("https://cloudsecurityalliance.org/artifacts/ccm-aicm-licensing-faq"));
    // Distinct from the #67 provider skill.
    assert!(body.contains("#67"));
    // argv-list subprocess discipline documented.
    assert!(body.contains("argv-list"));
}

#[test]
fn python_helper_imports_are_stdlib_only() {
    let script = read(skill_path().join("scripts/scenario_catalog.py"));
    let allowed = ["argparse", "json", "os", "pathlib", "re", "sys"];
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
                "scenario_catalog.py imports disallowed module `{root}` in `{trimmed}`"
            );
        }
    }
    assert!(!script.contains("requests"));
    assert!(!script.contains("urllib"));
    // Offline + no shell-string subprocess.
    assert!(!script.contains("subprocess"));
    assert!(!script.contains("os.system"));
    assert!(!script.contains("bash -c"));
}

#[test]
fn symlink_check_runs_before_path_resolution() {
    let script = read(skill_path().join("scripts/scenario_catalog.py"));
    let check = script
        .find("assert_no_symlink_path(scenarios_dir_arg)")
        .expect("scenarios-dir symlink check missing");
    let resolve = script
        .find("scenarios_dir_arg.resolve(strict=True)")
        .expect("scenarios-dir resolve missing");
    assert!(
        check < resolve,
        "symlink check must run before path resolution"
    );
}

#[test]
fn size_cap_documented_and_enforced() {
    let script = read(skill_path().join("scripts/scenario_catalog.py"));
    let methodology = read(skill_path().join("references/methodology.md"));
    assert!(script.contains("1 * 1024 * 1024"));
    assert!(methodology.contains("1 MiB"));
    assert!(methodology.contains("argv-list"));
}

#[test]
fn scenario_catalog_covers_aws_github_and_cloudflare() {
    let scenarios = skill_path().join("scenarios");
    let script = read(skill_path().join("scripts/scenario_catalog.py"));

    let mut providers = std::collections::BTreeSet::new();
    for id in SCENARIO_ORDER {
        let path = scenarios.join(format!("{id}.json"));
        assert!(path.is_file(), "missing scenario fixture {id}.json");
        let body = read(&path);
        assert!(body.contains(&format!("\"id\": \"{id}\"")));
        // The declared order is the stability contract — it must list this id.
        assert!(
            script.contains(&format!("\"{id}\"")),
            "scenario_catalog.py SCENARIO_ORDER missing `{id}`"
        );
        for provider in ["aws", "github", "cloudflare", "cross-provider"] {
            if body.contains(&format!("\"provider\": \"{provider}\"")) {
                providers.insert(provider);
            }
        }
    }

    // On-disk set equals the declared set (no extra, no missing).
    let on_disk: std::collections::BTreeSet<String> = fs::read_dir(&scenarios)
        .expect("scenarios dir missing")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .collect();
    let declared: std::collections::BTreeSet<String> =
        SCENARIO_ORDER.iter().map(|s| s.to_string()).collect();
    assert_eq!(on_disk, declared, "on-disk scenario set != declared order");

    for required in ["aws", "github", "cloudflare", "cross-provider"] {
        assert!(
            providers.contains(required),
            "scenario catalog missing a `{required}` provider scenario"
        );
    }
}

#[test]
fn scenarios_are_ids_only_no_verbatim_prose() {
    // The skill's enforceable IDs-only core: every stride control token is an
    // identifier, never a sentence. Spot-check the fixtures contain no obvious
    // licensed-prose deny-list trigger.
    let scenarios = skill_path().join("scenarios");
    for id in SCENARIO_ORDER {
        let body = read(scenarios.join(format!("{id}.json"))).to_ascii_lowercase();
        for needle in [
            "reproduced with permission of the cloud security alliance",
            "copyright center for internet security",
        ] {
            assert!(
                !body.contains(needle),
                "{id}.json contains licensed-prose marker `{needle}`"
            );
        }
    }
}

#[test]
fn citation_map_stays_in_sync_between_doc_and_script() {
    let doc = read(skill_path().join("references/citation-and-licensing.md"));
    let script = read(skill_path().join("scripts/scenario_catalog.py"));
    for framework in [
        "CCM",
        "CIS-AWS-v5.0.0",
        "CIS-GitHub-v1.2.0",
        "NIST-800-53-r5",
        "NIST-SSDF-v1.1",
        "ATLAS",
        "MITRE-ATTCK",
        "OpenSSF-Scorecard",
        "GitHub-Well-Architected",
        "OWASP-ASVS-v5.0",
        "ISO-27001-2022",
        "SOC2-TSC-2017",
        "Hulumi-Policy",
    ] {
        assert!(
            doc.contains(framework),
            "doc missing framework `{framework}`"
        );
        assert!(
            script.contains(framework),
            "script missing framework `{framework}`"
        );
    }
    // Hulumi policy rule IDs are cited as bare IDs (framework Hulumi-Policy).
    for rule in [
        "CF_DNS_1",
        "CF_DNSSEC_1",
        "CF_ORIGIN_1",
        "X_ORIGIN_1",
        "G_OIDC_1",
    ] {
        assert!(script.contains(rule), "script missing rule `{rule}`");
        assert!(doc.contains(rule), "doc missing rule `{rule}`");
    }
}

#[test]
fn seven_canonical_evals_present_with_required_shape() {
    for case in [
        "happy-path",
        "adversarial",
        "ambiguous-input",
        "high-risk-case",
        "missing-context",
        "outdated-information",
        "tool-failure",
    ] {
        let path = skill_path().join("evals").join(format!("{case}.md"));
        assert!(path.is_file(), "missing eval case {}", path.display());
        let body = read(&path);
        assert!(body.starts_with("---\n"));
        assert!(body.contains("skill: slo-cloud-threat-model"));
        assert!(body.contains("## Input"));
        assert!(body.contains("## Expected Behavior"));
        assert!(body.contains("## Must Not"));
    }
}

#[test]
fn registered_in_skill_pack_catalog() {
    let catalog = read(repo_root().join("docs/skill-pack-catalog.md"));
    let skill_count = fs::read_dir(repo_root().join("skills"))
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir() && entry.path().join("SKILL.md").is_file())
        .count();
    assert!(catalog.contains("/slo-cloud-threat-model"));
    assert!(catalog.contains(&format!("Shipped skills at HEAD: {skill_count}")));
    assert!(catalog.contains("11 power tools"));
}

#[test]
fn skill_md_does_not_link_to_examples() {
    let body = read(skill_path().join("SKILL.md"));
    assert!(
        !body.contains("](examples/")
            && !body.contains("](../examples/")
            && !body.contains("](../../examples/"),
        "SKILL.md must not link into examples/"
    );
}

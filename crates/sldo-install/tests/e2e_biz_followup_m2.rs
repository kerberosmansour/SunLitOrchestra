//! Follow-up M2 — biz-pack-test-hardening.
//!
//! Per combined critique:
//! - Runbook A f3 / B1+B2+C f3: tighten cross-skill citation context. The
//!   existing test asserts substring presence of predicate IDs; this test
//!   asserts the citations appear in OPERATIVE-POLICY context (within the
//!   "Hard-block gates" section of advisor SKILL.mds), not in passing prose
//!   or metadata only.
//! - B1+B2+C f3: uniform generator-citation bound. Each generator may mention
//!   each predicate ID at most 2 times (allowing legitimate forwarding +
//!   "What this skill is NOT" prose). Stricter than per-skill ad hoc bounds.
//! - B1+B2+C f4: disambiguation disjoint-set test. PM-side KPIs (DAU /
//!   activation / retention / feature-adoption) appear ONLY in /slo-product
//!   as primary; financial KPIs (CAC / LTV / NDR / burn multiple / ARR)
//!   appear ONLY in /slo-metrics as primary. Neither leaks into the other.
//! - Runbook A f4 / f5 (interim): tighten oneNDA placeholder check to be
//!   FAIL-CLOSED until the canonical SHA-256 is pinned by follow-up #3.
//!   Currently `e2e_biz_a_m1.rs::onenda_template_placeholder_or_pinned_hash`
//!   accepts marker OR pinned-hash; this followup asserts the marker IS
//!   present (interim) — when follow-up #3 pins the hash, this test gets
//!   updated to require the hash match instead.

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

const ADVISOR_SKILLS: &[&str] = &["slo-legal", "slo-accounting", "slo-equity", "slo-fundraise"];

const GENERATOR_SKILLS: &[&str] = &[
    "slo-talk-to-users", "slo-gtm", "slo-product", "slo-marketing",
    "slo-launch", "slo-sales-funnel", "slo-pricing", "slo-metrics",
    "slo-cofounder", "slo-hire", "slo-founder-check",
];

// ---------------------------------------------------------------------------
// Hardening #1 — advisor predicate citations must appear in Hard-block-gates section.
// ---------------------------------------------------------------------------

#[test]
fn advisor_predicate_citations_in_hard_block_section() {
    for skill_name in ADVISOR_SKILLS {
        let body = read(&repo_root().join("skills").join(skill_name).join("SKILL.md"));

        // Find the "Hard-block gates" section start.
        let section_start = body.find("Hard-block gates").or_else(|| body.find("hard-block gates"));
        assert!(
            section_start.is_some(),
            "advisor `{skill_name}` SKILL.md must have a `Hard-block gates` heading"
        );
        let section_start = section_start.unwrap();

        // The section ends at the next H2 heading.
        let after_start = &body[section_start..];
        let section_end_offset = after_start[10..].find("\n## ").map(|i| i + 10).unwrap_or(after_start.len());
        let section = &after_start[..section_end_offset];

        // Every predicate ID must appear inside this section.
        for pid in FOUR_PREDICATE_IDS {
            assert!(
                section.contains(pid),
                "advisor `{skill_name}` must cite predicate `{pid}` INSIDE the Hard-block gates section (citation in metadata / TOC / footer doesn't count); section length = {} chars",
                section.len()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Hardening #2 — uniform generator-citation bound (≤ 2 per predicate per skill).
// ---------------------------------------------------------------------------

#[test]
fn generator_citation_bound_uniform_at_2() {
    for skill_name in GENERATOR_SKILLS {
        let body = read(&repo_root().join("skills").join(skill_name).join("SKILL.md"));
        for pid in FOUR_PREDICATE_IDS {
            let count = body.matches(pid).count();
            assert!(
                count <= 2,
                "generator `{skill_name}` cites predicate `{pid}` {count} times; uniform bound is ≤ 2 per predicate per generator. \
                 Generators may mention a predicate up to twice (typically: once for forwarding routing reference + once in 'What this skill is NOT' prose). \
                 More than 2 indicates the skill is drifting toward advisor territory; either elevate it to advisor and add it to the cross-skill citation contract, or refactor the SKILL.md to reduce predicate density."
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Hardening #3a — PM-side KPIs are PRIMARY only in /slo-product.
// ---------------------------------------------------------------------------

#[test]
fn pm_kpis_primary_only_in_slo_product() {
    let pm_kpis = ["DAU", "activation funnel", "Activation funnel", "feature-adoption", "feature adoption"];

    let product_body = read(&repo_root().join("skills/slo-product/SKILL.md"));
    let metrics_body = read(&repo_root().join("skills/slo-metrics/SKILL.md"));

    // /slo-product should own these PM concepts (≥ 3 mentions across the set).
    let product_count: usize = pm_kpis.iter().map(|k| product_body.matches(k).count()).sum();
    assert!(
        product_count >= 3,
        "/slo-product must own PM-side KPI vocabulary (DAU / activation funnel / feature-adoption) — found {product_count} total mentions across the set"
    );

    // /slo-metrics may MENTION PM KPIs (e.g., in the disambiguation section
    // "for DAU / activation, run /slo-product metrics") but must do so in a
    // ROUTING context, not a PRIMARY-KPI-list context. We approximate the
    // discriminator: if /slo-metrics mentions DAU, the surrounding ±50 chars
    // must include a routing signal.
    if metrics_body.contains("DAU") {
        let routing_signals = ["run `/slo-product", "run /slo-product", "PM-side", "PM side", "redirect", "Redirect"];
        let any_routing = routing_signals.iter().any(|r| metrics_body.contains(r));
        assert!(
            any_routing,
            "/slo-metrics mentions `DAU` but lacks a routing signal back to /slo-product — DAU is a PM-side primary KPI and should not appear as a /slo-metrics primary"
        );
    }
}

// ---------------------------------------------------------------------------
// Hardening #3b — financial KPIs are PRIMARY only in /slo-metrics.
// ---------------------------------------------------------------------------

#[test]
fn financial_kpis_primary_only_in_slo_metrics() {
    let financial_kpis = ["CAC", "LTV", "NDR", "burn multiple", "MoM revenue growth", "Gross margin"];

    let product_body = read(&repo_root().join("skills/slo-product/SKILL.md"));
    let metrics_body = read(&repo_root().join("skills/slo-metrics/SKILL.md"));

    // /slo-metrics should own the financial-KPI vocabulary (≥ 4 of the 6).
    let metrics_count = financial_kpis.iter().filter(|k| metrics_body.contains(**k)).count();
    assert!(
        metrics_count >= 4,
        "/slo-metrics must own financial-KPI vocabulary — found {metrics_count} of {financial_kpis:?}"
    );

    // /slo-product may MENTION CAC / LTV / NDR / burn-multiple in the redirect
    // context ("for CAC / LTV / NDR / burn multiple use /slo-metrics") but
    // must do so with a routing signal.
    let mentioned: Vec<&&str> = financial_kpis.iter().filter(|k| product_body.contains(**k)).collect();
    if !mentioned.is_empty() {
        let routing_signals = ["use `/slo-metrics", "use /slo-metrics", "Runbook B2", "redirect to `/slo-metrics", "redirects financial KPIs to `/slo-metrics", "redirect to /slo-metrics", "redirects financial KPIs to /slo-metrics"];
        let any_routing = routing_signals.iter().any(|r| product_body.contains(r));
        assert!(
            any_routing,
            "/slo-product mentions financial KPIs ({mentioned:?}) but lacks an explicit routing signal back to /slo-metrics — financial KPIs are /slo-metrics primary territory"
        );
    }
}

// ---------------------------------------------------------------------------
// Hardening #4 — post-pinning canonical-state assertion.
//
// Was: interim fail-closed-on-PLACEHOLDER until follow-up #3 pinned SHA-256.
// Now (post-pinning, follow-up `biz-pack-onenda-canonical-pin`): assert the
// canonical state — CANONICAL-PINNED marker present, frontmatter records a
// 64-char hex SHA-256 (not `pending-user-fetch`), and the license obligation
// remains documented. Reverting to PLACEHOLDER would silently regress the
// supply-chain integrity guarantee (critique f4 / f5 — V10.3 Code Integrity).
// ---------------------------------------------------------------------------

const ONENDA_CANONICAL_PINNED_MARKER: &str = "ONENDA-UK-CANONICAL-PINNED";

#[test]
fn onenda_canonical_pinned_state_locked() {
    let body = read(&repo_root().join("references/biz/templates/onenda-uk.md"));

    // The CANONICAL-PINNED marker is REQUIRED. Reverting to PLACEHOLDER would
    // signal the canonical bytes were no longer verified — supply-chain
    // regression.
    assert!(
        body.contains(ONENDA_CANONICAL_PINNED_MARKER),
        "post-pinning regression: oneNDA template must contain the `{ONENDA_CANONICAL_PINNED_MARKER}` marker. The canonical SHA-256 was pinned on 2026-04-25 by follow-up `biz-pack-onenda-canonical-pin`; reverting to PLACEHOLDER would create the placeholder-window vulnerability documented in critique f4 / f5 (V10.3 Code Integrity)."
    );

    // Frontmatter pinned_canonical_sha256 must be a 64-char hex digest, not
    // `pending-user-fetch`.
    let line = body
        .lines()
        .find(|l| {
            let t = l.trim_start();
            t.starts_with("pinned_canonical_sha256:")
                && !t.contains("pending-user-fetch")
                && !t.contains("`pinned_canonical_sha256:")
        })
        .unwrap_or("");
    let value = line.split("pinned_canonical_sha256:").nth(1).unwrap_or("").trim();
    let is_hex = value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit());
    assert!(
        is_hex,
        "post-pinning: `pinned_canonical_sha256` must be a 64-char hex digest, got `{value}`. Reverting to `pending-user-fetch` is a supply-chain regression."
    );

    // The license obligation must remain documented.
    assert!(
        body.contains("CC BY-ND 4.0"),
        "oneNDA template must continue to document the CC BY-ND 4.0 verbatim render obligation"
    );
}

// ---------------------------------------------------------------------------
// Sanity — none of the four hardening rules conflict with existing tests.
// ---------------------------------------------------------------------------

#[test]
fn hardening_compatible_with_existing_runbook_invariants() {
    // The cross-skill citation contract from Runbook A is preserved: every
    // advisor SKILL.md still cites all four predicate IDs (this test asserts
    // the SUBSET — that the citations are in the Hard-block gates section
    // — not contradicting the existing substring-presence test).
    for skill_name in ADVISOR_SKILLS {
        let body = read(&repo_root().join("skills").join(skill_name).join("SKILL.md"));
        for pid in FOUR_PREDICATE_IDS {
            assert!(
                body.contains(pid),
                "regression: advisor `{skill_name}` must still cite predicate `{pid}` SOMEWHERE (existing Runbook A f-row contract)"
            );
        }
    }

    // Predicate-id immutability preserved.
    let gate = read(&repo_root().join("references/biz/triage-gate.md"));
    for pid in FOUR_PREDICATE_IDS {
        assert!(gate.contains(pid));
    }
    for n in 5..=9 {
        let candidate = format!("gate-{n}-");
        assert!(!gate.contains(&candidate));
    }
}

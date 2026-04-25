//! Follow-up M1 — biz-pack-cost-baseline-refresh.
//!
//! Critique f2 (Runbook A): no test asserts cost-baseline placeholders are
//! gone. Original JPP Law swap was flawed — JPP Law doesn't publish GBP
//! figures publicly. This follow-up pivots to a mixed baseline:
//!   1. SeedLegals subscription tiers (public, retrievable) — primary.
//!   2. Founder's solicitor quote (private, populated at first-use).
//!   3. Explicit `pricing_provenance: placeholder-pending` for deferred state.
//!
//! Test asserts ONE of the three states is unambiguously declared.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

#[test]
fn cost_baseline_has_real_figures_or_explicit_placeholder() {
    let body = read(&repo_root().join("references/biz/cost-baseline-jpp-law-2026.md"));

    // Path 1: SeedLegals public tier section has ≥ 5 GBP figures.
    let gbp_count = body.matches("£").count();
    let seedlegals_path = body.contains("SeedLegals subscription tiers")
        || body.contains("SeedLegals public-tier")
        || body.contains("https://seedlegals.com/pricing/");
    let path_1_ok = gbp_count >= 5 && seedlegals_path;

    // Path 2: "Your firm" section populated (no `<...>` placeholder tokens
    // remaining inside the your_firm: yaml block).
    let path_2_ok = if body.contains("your_firm:") {
        // Find the your_firm section and check it has no `<...>` placeholder tokens.
        let after = body.split("your_firm:").nth(1).unwrap_or("");
        // Take the next ~20 lines as the your_firm block.
        let block: String = after.lines().take(20).collect::<Vec<_>>().join("\n");
        !block.contains("<e.g.")
            && !block.contains("<YYYY-MM-DD>")
            && !block.contains("<GBP")
            && !block.contains("<name of partner")
    } else {
        false
    };

    // Path 3: explicit `pricing_provenance: placeholder-pending` in frontmatter
    // AND a "REQUIRES POPULATION BEFORE PRODUCTION USE" body marker.
    let path_3_ok = body.contains("pricing_provenance: placeholder-pending")
        && body.contains("REQUIRES POPULATION BEFORE PRODUCTION USE");

    assert!(
        path_1_ok || path_2_ok || path_3_ok,
        "cost-baseline-jpp-law-2026.md must satisfy at least one of:\n  \
         (1) SeedLegals public-tier section with ≥ 5 GBP figures (got gbp_count={gbp_count}, seedlegals_path={seedlegals_path}),\n  \
         (2) 'Your firm' section populated (no <...> placeholder tokens),\n  \
         (3) Explicit pricing_provenance: placeholder-pending + 'REQUIRES POPULATION BEFORE PRODUCTION USE' body marker.\n\
         None of the three paths satisfied — the file is in a broken state."
    );
}

#[test]
fn cost_baseline_documents_provenance_field() {
    let body = read(&repo_root().join("references/biz/cost-baseline-jpp-law-2026.md"));

    // The frontmatter must declare a pricing_provenance field with a known value.
    let known_values = [
        "pricing_provenance: mixed-public-and-quote-pending",
        "pricing_provenance: solicitor-quoted",
        "pricing_provenance: placeholder-pending",
    ];
    let any = known_values.iter().any(|v| body.contains(v));
    assert!(
        any,
        "cost-baseline-jpp-law-2026.md must declare `pricing_provenance:` with one of {known_values:?}"
    );
}

#[test]
fn cost_baseline_seedlegals_section_present_with_known_tiers() {
    let body = read(&repo_root().join("references/biz/cost-baseline-jpp-law-2026.md"));

    // SeedLegals tiers a founder might cite — at least 4 of these named tier labels must appear.
    let tiers = ["Access", "Funding — Start", "Funding — Raise", "Funding — Scale", "Options scheme", "SEIS/EIS Advance Assurance"];
    let count = tiers.iter().filter(|t| body.contains(**t)).count();
    assert!(
        count >= 4,
        "cost-baseline must enumerate at least 4 SeedLegals tier labels (found {count} of {tiers:?})"
    );

    // The retrieval-date convention is preserved (regression on M1 contract).
    assert!(
        body.contains("retrieved: 2026-04-25") || body.contains("retrieved: "),
        "cost-baseline must carry a retrieved: date in frontmatter"
    );
}

#[test]
fn cost_baseline_your_firm_template_includes_required_fee_categories() {
    let body = read(&repo_root().join("references/biz/cost-baseline-jpp-law-2026.md"));

    // The "Your firm" template must include the v1 fee categories so a founder
    // populating it captures the right line items.
    let categories = ["nda:", "contractor_sow:", "ip_assignment:", "terms_and_conditions_b2b:"];
    for cat in &categories {
        assert!(
            body.contains(cat),
            "your_firm template must include `{cat}` line so founder records the v1 doc-type fees"
        );
    }
}

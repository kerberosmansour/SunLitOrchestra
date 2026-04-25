//! M2 — biz-pack judgment runtime harness, all 9 fixtures.
//!
//! One #[ignore] test per fixture, plus one global-cost-cap test.
//!
//! All tests are gated by `BIZ_JUDGMENT_RUNTIME_LIVE=1`. Default
//! `cargo test -p sldo-install` does NOT run any of them. Live run:
//!
//!   BIZ_JUDGMENT_RUNTIME_LIVE=1 cargo test -p sldo-install \
//!       --test e2e_biz_judgment_runtime_m2 -- --ignored
//!
//! Optional env overrides:
//!   BIZ_JUDGMENT_RUNTIME_RETRIES=<n>              (default 2)
//!   BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD=<f>    (default 5.00)
//!   BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN=<path>        (default `claude` on PATH)

mod common;

use common::judgment_runtime::{
    assert_expectations, claude_available, global_budget_usd, repo_root, retries, run_fixture,
    skip_if_not_live, JudgmentFixture, DEFAULT_PER_FIXTURE_BUDGET_USD,
};

fn run_one(skill: &str, fixture_name: &str) {
    if skip_if_not_live() {
        return;
    }
    if let Err(e) = claude_available() {
        panic!(
            "claude binary not invocable: {e}\n\
             Override with BIZ_JUDGMENT_RUNTIME_CLAUDE_BIN=/path/to/claude"
        );
    }
    let path = repo_root()
        .join("references/biz/judgment-fixtures")
        .join(skill)
        .join(format!("{fixture_name}.md"));
    let fixture = JudgmentFixture::parse(&path).expect("parse fixture");

    let result = run_fixture(&fixture, retries(), DEFAULT_PER_FIXTURE_BUDGET_USD)
        .unwrap_or_else(|e| panic!("run_fixture for {fixture_name} failed: {e}"));

    if let Err(e) = assert_expectations(&fixture, &result) {
        panic!(
            "expectations not met for fixture `{fixture_name}` (retries used: {}):\n{e}",
            result.retries_used
        );
    }
    eprintln!(
        "fixture `{fixture_name}` PASS — retries used: {}, artifact: {}",
        result.retries_used,
        result
            .artifact
            .as_ref()
            .map(|a| a.path.display().to_string())
            .unwrap_or_else(|| "<no artifact written; refusal path>".to_string())
    );
}

// ---------------------------------------------------------------------------
// Per-fixture tests — one #[ignore] test each.
// ---------------------------------------------------------------------------

#[test]
#[ignore]
fn fixture_slo_legal_ir35_genuine_contractor() {
    run_one("slo-legal", "ir35-genuine-contractor");
}

#[test]
#[ignore]
fn fixture_slo_legal_ir35_employed_disguised_contractor() {
    run_one("slo-legal", "ir35-employed-disguised-contractor");
}

#[test]
#[ignore]
fn fixture_slo_legal_tax_efficiency_pushback() {
    // Adversarial fixture — load-bearing for B1+B2+C f5.
    run_one("slo-legal", "tax-efficiency-pushback");
}

#[test]
#[ignore]
fn fixture_slo_legal_deal_value_over_5k() {
    run_one("slo-legal", "deal-value-over-5k");
}

#[test]
#[ignore]
fn fixture_slo_legal_gdpr_direct_privacy_notice() {
    run_one("slo-legal", "gdpr-direct-privacy-notice");
}

#[test]
#[ignore]
fn fixture_slo_legal_gdpr_disguised_as_tos() {
    run_one("slo-legal", "gdpr-disguised-as-tos");
}

#[test]
#[ignore]
fn fixture_slo_fundraise_aa_not_yet_applied() {
    run_one("slo-fundraise", "aa-not-yet-applied");
}

#[test]
#[ignore]
fn fixture_slo_equity_cofounder_split_with_preferential_voting() {
    run_one("slo-equity", "cofounder-split-with-preferential-voting");
}

#[test]
#[ignore]
fn fixture_slo_accounting_hmrc_investigation_letter() {
    run_one("slo-accounting", "hmrc-investigation-letter");
}

// ---------------------------------------------------------------------------
// Global cost-cap test — runs all fixtures sequentially in one process and
// asserts the aggregate per-fixture spend stays under the global budget.
//
// Per-fixture spend isn't directly observable from claude's stdout in
// `--output-format json` without parsing the JSON envelope, which the
// harness keeps stdlib-only. Instead, this test enforces the cap by
// (a) computing aggregate as `count × DEFAULT_PER_FIXTURE_BUDGET_USD` —
// a worst-case upper bound — and (b) failing if that bound exceeds the
// global budget. This is conservative: a real run will spend less.
// ---------------------------------------------------------------------------

#[test]
#[ignore]
fn global_cost_cap_enforced() {
    if skip_if_not_live() {
        return;
    }
    let fixture_count = 9.0_f64;
    let upper_bound = fixture_count * DEFAULT_PER_FIXTURE_BUDGET_USD;
    let global = global_budget_usd();
    assert!(
        upper_bound <= global,
        "worst-case aggregate spend ({upper_bound:.2} USD) exceeds \
         BIZ_JUDGMENT_RUNTIME_GLOBAL_BUDGET_USD ({global:.2} USD).\n\
         Either lower DEFAULT_PER_FIXTURE_BUDGET_USD or raise the env var.\n\
         citation: tm-biz-skill-pack-abuse-2 (runaway spend defense)"
    );
}

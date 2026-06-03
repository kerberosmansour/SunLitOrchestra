# Verification Report — svl Milestone 5

Milestone: LOOPS docs + `/slo-ship` secure-release checklist + dogfood. Target kind: docs + skill prose + runbook self-application. Runtime QA = structural suite + **full workspace** test + Pass 4.

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| LOOPS-ENGINEERING names per-stage security output | happy path | `svl_m5::loops_engineering_names_security_output_per_stage` | pass | Secure Value Loop overlay table |
| LOOPS-BUSINESS security-visible proof of safety cross-ref | happy path | `svl_m5::loops_business_has_security_cross_ref` | pass | |
| `/slo-ship` secure-release checklist + closed `ship_state` enum | happy path | `svl_m5::ship_has_secure_release_checklist_and_closed_ship_state` | pass | 5 ship_state values |
| SBOM/provenance conditional (never hard gate for markdown) | invalid input | `svl_m5::ship_sbom_provenance_is_conditional` | pass | "when applicable" + "not_applicable" + "released artifact" |
| Runbook dogfoods a FILLED §5B (no placeholders) | happy path | `svl_m5::runbook_dogfoods_a_filled_secure_value_contract` | pass | DW-001 + abuse IDs + readiness flag |
| **Full workspace** green | regression | `cargo test --workspace` | pass | 122 result groups, 0 failed |

## Bugs found (caught during M5 full-workspace verify — fixed, regression-covered)

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| DW-004 | medium | M3 workspace version bump (0.1.2→0.1.3) was incomplete — internal dep version strings + `PUBLISH_READY_VERSION` left at 0.1.2 | `e2e_crates_io_followup::publish_prep_bumps_workspace_and_internal_dependency_versions` (pre-existing lockstep test — caught it) | fixed |
| DW-005 | low | skill-prose additions tripped structural caps: `/slo-plan` >80 (hard), `/slo-verify` >200 (soft) | `e2e_eng_imp_m4::{slo_plan_skill_md_decomposed, soft_line_cap_runs_for_every_skill_md}` (pre-existing caps — caught it) | fixed |

Both were caught by **pre-existing** structural tests during the M5 full-workspace run — exactly the safety net the runbook relies on. Fixes: DW-004 → bumped the two dep strings + the test constant to 0.1.3 (allow-list extended with rationale); DW-005 → extracted `/slo-plan` detail to `references/secure-value-contract.md` (77 lines now) + added `# soft-cap-exception` to `/slo-verify`.

## Pass 4 — Security

| Check | Result | Evidence |
|---|---|---|
| Secrets scan over M5 edits | pass | clean |
| `tm-secure-value-loop-abuse-1` (injection) — ship/resume fence surfaces | pass | named in SECURE-VALUE-LOOP §7; `/slo-ship` quotes ledger rows → fence rule applies |
| SAST (clippy) | pass | no NEW warnings; the one `unused import: Path` is pre-existing in `e2e_biz_followup_m5.rs` (untouched, out of scope §4.7) |
| DAST / container / IaC | N/A — no service/image/IaC | |

## Pass 5 / Pass 6
N/A — no AI component; not value-bearing.

## Coverage gaps
- DW-001 (cargo deny licenses), DW-002 (GitHub labels) remain operator/file-issue actions for ship time (confirmation-gated). DW-003 (crates.io 0.1.3 publish) accepted-risk until a deliberate release.

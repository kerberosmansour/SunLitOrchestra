# Verification Report — experiment-rigor Milestone 4

End-to-end synthetic dogfood, threat-model extension, and public-document integration. This milestone tests one tracked gallery Book and documentation contracts; no real benchmark, production code, service, or UI was added.

## Pass 0 — Outcome Contract

| Outcome slice | Exercise | Result | Evidence |
|---|---|---|---|
| Newcomer can trace the complete method | `gallery_walks_the_complete_rigorous_path` + manual README/LOOPS walk | pass | gallery contains PF-1, separate discovery/validation, per-arm results, ablations, failures, replication context, limits, confidence, packet, and typed route |
| Synthetic provenance is honest | `gallery_does_not_overstate_synthetic_evidence` | pass | every number is labeled synthetic fixture evidence; not real-world validation; not independently replicated; generalization gap explicit |
| Mutation and leakage threats are durable | threat-control test | pass | Markdown/JSON contain IDs 7–8 with amendment/stale/rerun and held-out/no-tuning/confidence-downgrade controls |
| Threat serialization is current and parseable | `threat_ids_are_contiguous_and_provenance_is_current` + `jq -e` | pass | IDs 1–8 contiguous/active; producer `slo-execute`; 2026-07-13; runbook input recorded; JSON parses |
| Public entry points tell one story | `public_docs_describe_the_same_rigorous_loop` + one-section test | pass | README, architecture, loop, catalog, security, and overview describe the rigorous path; exactly one loop section remains |
| Stable human-controlled Book survives | frozen-spine route test + existing M1–M5 suites | pass | §0–§11 order, routes, skill surfaces, scratch/no-production boundary, and never-auto-invoke rule remain |

The gallery is an executable documentation fixture: it demonstrates the method and calibrated reasoning but does not prove the fictional product result. Its `confirmatory` label is explicitly limited to the declared synthetic fixture scope and is not `engineering_ready`.

## Pass 1 — Happy Path

Starting from README and the canonical loop document, a reviewer can open the gallery and follow Protocol Freeze → Discovery Record → held-out/no-tuning Validation Record → Ablation Matrix/Failure Taxonomy → confirmatory-only confidence → RecommendationPacket → `promote_to_idea` human suggestion. Result: pass.

## Pass 2 — Invalid, Empty, And Compatibility Paths

- Synthetic evidence is not described as real-world or independent replication.
- The missing real corpus, retained runner, production policy, user-value evidence, and independent rerun remain limitations/uncertainty.
- Existing threat IDs 1–6 retain identity/status; new IDs append as 7–8.
- Legacy Books remain degraded/unconfirmed by the M1–M3 contracts.
- The gallery's idea route remains valid without pretending the packet is engineering-ready.

Result: pass.

## Pass 3 — Partial Failure, Bounds, And Recovery

- Post-result mutation requires ProtocolAmendment, stales validation, and requires rerun.
- Discovery/validation leakage invalidates evidence and downgrades confidence until a clean held-out/no-tuning rerun.
- Scope remains one gallery, two new threat IDs, and one canonical public loop section.
- The fixture exposes residual failures and generalization gaps rather than hiding them behind its headline.

Result: pass.

## Pass 4 — Security

| Check | Result | Note |
|---|---|---|
| Bundle A — docs/contract assessment | pass | protocol integrity, evidence separation, literal-data controls, route gating, and suggestion-only boundary are documented and structurally tested |
| Threat JSON parse/provenance | pass | actual innovation JSON parsed by the M4 test and `jq`; IDs 1–8 lockstep with Markdown; producer/input provenance updated honestly |
| Secret/PII scan | pass | existing `innovation_loop_m1_spine` gallery scan and M5 example scan green |
| Full security/regression suite | pass | full `cargo test -p sast-verify --tests` green |
| SCA | N/A | no dependency or lockfile change |
| DAST/authn/authz/IaC/container | N/A | no service, identity, cloud, or image surface |

## Pass 5 — AI Tolerance

Narrative wording may vary, but the method chain, synthetic label, confidence/route, frozen IDs, and human-control gate are deterministic. Missing evidence lowers confidence or blocks close; synthetic output may never be represented as real. The seven-test end-to-end suite is the deterministic eval. Result: pass.

## Pass 6 — Measurement And Telemetry

Immediate leading metric is satisfied: one gallery Book completes the rigorous chain and all four new structural-contract suites pass. There is no runtime telemetry surface. Lagging validation remains a manual audit of the next three real promoted Books and whether their receiving artifact needs a clarification round. Result: pass for immediate evidence; follow-up retained for lagging evidence.

## Regression Evidence

| Command/check | Result |
|---|---|
| `cargo test -p sast-verify --test innovation_loop_rigor_m4_end_to_end` | pass — 7/7 |
| innovation-loop M1/M2/M3/M4/M5 targeted suites | pass — 9/9, 5/5, 4/4, 6/6, 5/5 |
| `cargo test -p sast-verify --test slo_tm_m1_schema` | pass — 6/6 |
| M1–M3 rigor suites | pass — 6/6, 8/8, 8/8 in full suite |
| `cargo fmt --all -- --check` | pass after formatting the new M4 test |
| `cargo clippy -p sast-verify --test innovation_loop_rigor_m4_end_to_end -- -D warnings` | pass |
| `cargo test -p sast-verify --tests` | pass — full suite |
| `jq -e . docs/slo/design/innovation-loop-threat-model.slo.json` | pass |
| `rg -c '^## Innovation Sandbox loop$' docs/LOOPS-ENGINEERING.md` | pass — `1` |
| `git diff --check` | pass |

## Bugs Found

None. The initial semantic-red run failed all seven intended groups. During green-up, one stable phrase was line-wrapped in the gallery and the first format check found standard Rust layout differences; both were corrected and all affected/full gates rerun green.

## Coverage Gaps

- The gallery runner and synthetic corpus are not retained executable artifacts; the packet records the historical fixture method and explicitly lists independent reproduction as a gap.
- No real-world corpus, user study, or live-model compliance run exists.
- The generic `slo_tm_m1_schema` test validates the shared schema fixture; the M4 test separately parses and validates the actual innovation threat artifact's IDs/provenance/controls.

These gaps are visible in the gallery confidence and do not block the documentation/process outcome claimed by this runbook.

## Disposition

M4 verified. The rigorous path is dogfooded without overstating synthetic evidence; threat IDs 7–8 and current provenance are durable; public docs converge on one shipped loop; all compatibility, security, targeted, and full gates pass. Ready for `/slo-retro M4` and completed-runbook archival.

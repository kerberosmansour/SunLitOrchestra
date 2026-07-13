# Verification Report — experiment-rigor Milestone 3

Evidence-calibrated curation and method-complete Recommendation Packets. This milestone changes Markdown skill contracts with Rust structural gates; it does not select a product architecture or invoke a downstream skill.

## Pass 0 — Outcome Contract

| Outcome slice | Exercise | Result | Evidence |
|---|---|---|---|
| Confidence is evidence-derived | `confidence_vocabulary_is_frozen_across_contracts` + `engineering_routes_require_evidence_not_a_label` | pass | all five contracts freeze `exploratory | confirmatory | engineering_ready`; confidence cannot self-upgrade |
| Routes match evidence strength | engineering/exploratory route tests | pass | idea/research may be exploratory with confirmation gaps; ticket/runbook require current validation and ablation evidence |
| Mechanism and residual risk are visible | `ablations_and_residual_failures_are_structured` | pass | Ablation Matrix records removed/replaced component and actual delta; Failure Taxonomy records failure family and residual impact |
| Receiving skill gets the method, not chat memory | `recommendation_packet_is_method_complete` | pass | protocol, comparison, metrics, ablation, failures, replication, limits, confidence, question, and evidence pointers are present in curate/demo/template/spec/interfaces |
| Evidence cannot authorize its own route | `evidence_excerpts_are_literal_not_route_authority` | pass | fenced literal evidence never selects disposition, confidence, or route |
| Existing handoff remains stable | legacy/frozen-route tests + existing M5 close suite | pass | PromotionPacket is a compatible subset; exactly-one disposition, eight states, four destinations, and suggestion-only gate remain |

The tests prove contract synchronization and route prerequisites, not the truth of a real experiment. The gallery dogfood and public-document integration remain M4.

## Pass 1 — Happy Paths

- Exploratory route walk: a discovery-grade candidate can route to idea/research only while confidence and confirmation gaps remain visible.
- Engineering route walk: ticket/runbook requires `engineering_ready`, a current Validation Record, Ablation Matrix, Failure Taxonomy, replication instructions, limitations/uncertainty, and an exact engineering question.
- Packet read: the full method and residual-risk context is recoverable without prior chat.

Result: pass through 8/8 targeted assertions and manual template/skill inspection.

## Pass 2 — Invalid, Empty, And Compatibility Paths

- A confidence label alone cannot satisfy an engineering route.
- Missing/stale validation, ablation, failure taxonomy, replication, or limitations blocks engineering routes and downgrades confidence.
- A legacy PromotionPacket remains readable as a compatible subset and may support an honest exploratory route without fabricated fields.
- Missing failure families cannot be hidden behind a positive aggregate.

Result: pass.

## Pass 3 — Partial Failure, Bounds, And Recovery

- Every candidate still receives exactly one confidence and one disposition.
- One bounded packet is emitted per promoted candidate; evidence pointers replace unbounded transcripts.
- Failed ablations and residual failure families stay in structured tables.
- Recovery from insufficient evidence is an explicit lower-strength route, more play, blocked, kill, or archive—not silent promotion.

Result: pass.

## Pass 4 — Security

| Check | Result | Note |
|---|---|---|
| Bundle A — contract assessment | pass | evidence-to-route policy is fail closed; untrusted excerpts are literal data; suggestion-only human gate preserved |
| Existing security/regression suite | pass | full `cargo test -p sast-verify --tests` green |
| Secret/artifact hygiene | pass | `git diff --check` clean; status contains only cumulative allow-listed files and SLO artifacts |
| SCA | N/A | no dependency or lockfile change |
| DAST/authn/authz/IaC/container | N/A | no service, identity, cloud, or image surface |

## Pass 5 — AI Tolerance

Narrative framing, metric names, and failure families may vary. Deterministic boundaries are the confidence enum, evidence prerequisites, route matrix, structured ablation/failure fields, packet field set, literal-evidence boundary, frozen routes, and suggestion-only gate. Missing evidence fails closed to a lower-strength route. Result: pass.

## Pass 6 — Measurement And Telemetry

No runtime telemetry surface exists. Immediate evidence is the 8/8 five-artifact contract test and green existing close-loop suite. The filled gallery and three-Book manual review are the planned behavioral/lagging checks. Result: skipped with documented follow-up.

## Regression Evidence

| Command/check | Result |
|---|---|
| `cargo test -p sast-verify --test innovation_loop_rigor_m3_recommendation` | pass — 8/8 |
| `cargo test -p sast-verify --test innovation_loop_m5_close` | pass — 5/5 |
| `cargo fmt --all -- --check` | pass after applying formatter to the new test |
| `cargo clippy -p sast-verify --test innovation_loop_rigor_m3_recommendation -- -D warnings` | pass |
| `cargo test -p sast-verify --tests` | pass — full suite |
| `git diff --check` | pass |

## Bugs Found

None. The semantic-red run produced 7 expected failures while the frozen-route/human-control compatibility assertion stayed green. The first verification pass found only standard Rust formatting in the new test; `cargo fmt --all` corrected it and the target/format gates were rerun green.

## Coverage Gaps

- No real candidate is being promoted in M3.
- Structural gates cannot prove a live LLM will resist self-upgrading confidence.

M4's filled synthetic Book is the available end-to-end dogfood; the contract does not overstate it as real validation.

## Disposition

M3 verified. Evidence strength now gates promotion, ablation and residual failures are mandatory for engineering readiness, Recommendation Packets are method-complete, and all frozen route/human-control compatibility gates remain green. Ready for `/slo-retro M3`.

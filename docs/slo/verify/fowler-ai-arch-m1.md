# Verification Report — fowler-ai-arch Milestone 1

## What was exercised

| Scenario | Category | How exercised | Result | Evidence |
|---|---|---|---|---|
| Architect documents reversibility output | happy path | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `architect_documents_reversibility_output` inspects `skills/slo-architect/SKILL.md`. | pass | Output path, hard-to-change decisions, and rollback/migration proof are asserted. |
| Architect documents code-map output | happy path | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `architect_documents_brownfield_code_map_output` inspects `skills/slo-architect/SKILL.md`. | pass | Code-map path plus four-object, exemplar, anti-exemplar, and dangerous-seam language are asserted. |
| Existing outputs preserved | backward compatibility | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `existing_architect_outputs_remain_documented` checks all five prior output names. | pass | `ARCHITECTURE.md`, stack decision, interfaces, `SECURITY.md`, and threat model remain documented. |
| Output count wording updated | backward compatibility | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `architect_output_count_wording_is_updated` rejects stale wording. | pass | `Five files` is absent and `Seven files` is present. |
| Greenfield N/A path exists | empty state | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `brownfield_code_map_has_greenfield_na_path` checks exact N/A wording. | pass | `N/A — greenfield; no existing codebase to map` is documented. |
| User source notes poisoned | abuse case | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `architect_preserves_user_string_fence_rule` checks the existing fence defense. | pass | `~~~text` and user-provided string language remain visible. |
| Architect eval coverage | happy/high-risk | Ran `cargo test -p sldo-install --test e2e_fowler_ai_arch_m1`; test `architect_evals_cover_reversibility_and_ambiguous_brownfield_context` reads both eval files. | pass | Happy path expects reversibility/code map; high-risk case covers ambiguous brownfield context. |

## Pass 4 Security / Static Evidence

| Stack / tool | Command | Result | Evidence |
|---|---|---|---|
| Rust supply chain | `cargo audit --stale` | pass | No RustSec advisories reported for `Cargo.lock`. |
| Rust license policy | `cargo deny check` | skipped | No project deny config exists; default config rejected existing common licenses, so this is not a milestone finding. |
| Rust variant analysis | `ast-grep scan --rule-dirs skills/slo-verify/rules/rust --json-compact-with-summary` | skipped | `ast-grep` is not installed. |
| Any-stack Semgrep | `semgrep scan --config=p/security-audit --metrics=off --quiet --json --output /tmp/fowler-ai-arch-m1-semgrep.json skills/slo-architect crates/sldo-install/tests/e2e_fowler_ai_arch_m1.rs` | pass | Semgrep JSON reported `0` results. |
| DAST | N/A | N/A | Markdown skill-pack milestone; no smoke service or OpenAPI surface. |
| Biz PII scan | N/A | N/A | Repository has no `docs/biz-public/` subtree. |

## Bugs found

| id | severity | scenario | regression test | status |
|----|----------|----------|-----------------|--------|
| N/A | N/A | No M1 bugs found. | N/A | N/A |

## Environment

- OS: Darwin 25.4.0 arm64.
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`.
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.
- Browser: N/A — no UI surface.

## Coverage gaps

- No runtime agent harness invoked `/slo-architect`; the milestone is verified through deterministic Markdown structural tests.
- `cargo fmt --all -- --check` remains red on pre-existing files outside M1's allow-list.

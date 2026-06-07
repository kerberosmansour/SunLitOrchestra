# Completion Summary — innovation-loop Milestone 1

## Goal completed
A founder can run `/slo-experiment <slug>` and get a complete, contract-shaped `docs/slo/experiments/<slug>/EXPERIMENT.md` (§0–§11) seeded with the metadata, tracker, 10 global rules, Safety Rails, §2A Judgment Timing Rule, Phase Contract pattern, Definition-of-Learned blocks, and frozen vocabularies — with a structural-contract test guaranteeing the template + skill shape, runtime slug validation (S1), and `~~~text` fences (S2).

## Files changed
- **NEW** `docs/slo/templates/experiment-book-template_v_1.md` — the Experiment Book v1 / Creative Experiment Contract.
- **NEW** `skills/slo-experiment/SKILL.md` — the umbrella skill (slug guard `^[a-z0-9][a-z0-9-]*$`, fence discipline, open/resume).
- **NEW** `xtasks/sast-verify/tests/innovation_loop_m1_spine.rs` — 9-assertion structural-contract test.
- `docs/skill-pack-catalog.md` — Innovation-Sandbox flow section + count 41→42.
- `docs/LOOPS-ENGINEERING.md` — "Start here" row + Innovation Sandbox loop section.
- `docs/ARCHITECTURE.md` — `/slo-experiment` dashed→solid; M1 status.
- `CLAUDE.md`, `copilot-instructions.md`, `AGENTS.md` — short loop pointers (overlays stay overlays).
- `.gitignore` — `/experiments/` (anchored) scratch root.
- `crates/sldo-install/tests/e2e_cloud_threat_model_m1.rs` + `crates/sldo-install/tests/e2e_slo_nettacker.rs` — count assertion 41→42 (both count-pinning tests; second one added to allow-list mid-run with approval).

## Tests added
- `innovation_loop_m1_spine.rs` — 9 tests: frontmatter+name, output-path safety, slug-validation mandate (S1), sections-in-order, frozen vocabularies, Judgment-Timing-Rule+Safety-Rails, Definition-of-Learned+seeds, `~~~text` fences (S2), PII/secret scan.

## Runtime validations added
- The structural-contract test is the runtime gate (no app to boot). Plus the dogfood smoke: open a Book from the template, confirm §0–§11, confirm slug validity, run the test against a real Book (non-vacuous PII scan), remove, confirm clean tree.

## Static analysis and formatter evidence
- `cargo fmt -p sast-verify -- --check`: clean.
- `cargo clippy -p sast-verify --all-targets -- -D warnings`: my test file clean; 3 PRE-EXISTING errors outside the allow-list → DW-001 (`file_github_issue`).
- `cargo test -p sast-verify`: 169 passed / 0 failed. Baseline crates: all green.

## Compatibility checks performed
- `discover_skills()` unchanged (install dry-run lists `slo-experiment`).
- `/slo-critique` SHA-256 baseline unmoved.
- Catalog reconciles to 42; both count-pinning tests re-pointed.
- Books under `docs/slo/experiments/` remain git-tracked (after the `.gitignore` anchor fix); scratch under `/experiments/` ignored.

## Invariants/assertions added
- See the M1 Evidence Log / lessons: frozen §0–§11, 8 exit states, 5 status values, 5 modes, slug regex, fences, output-path allow-list.

## Resource bounds added or verified
- Book sections bounded to 12; exit vocabulary to 8 — template tamper → test red.

## Documentation updated
- ARCHITECTURE.md, LOOPS-ENGINEERING.md, skill-pack-catalog.md, 3 host overlays, SECURITY.md (merged at architect time).

## .gitignore changes
- Added anchored `/experiments/` (scratch root); load-bearing leading slash so Books under `docs/slo/experiments/` stay tracked.

## Test artifact cleanup verified
- `git status` clean after the dogfood smoke; only intended M1 files present.

## Deferred follow-ups
- DW-001 clippy debt → `/slo-retro` files via `slo-process` lane.
- M2 will need sentinel-string framing for `/slo-play` divergence (critique E1).

## Known non-blocking limitations
- The structural test guarantees presence/absence of sentinels, not tonal divergence of `/slo-play` (owned by the M5 dogfood + human read).
- `skills/slo-experiment/references/` would not be SHA-pinned by `sldo-install` (shared residual across all skills; none added in M1).

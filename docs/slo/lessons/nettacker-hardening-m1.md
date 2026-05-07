# Lessons - nettacker-hardening M1

## What Changed

- Added BDD-style structural tests for the lab-derived Nettacker workflow hardening.
- Encoded passive HTTP baseline and SPA/wildcard triage before URL-probe modules.
- Added response-header no-hit cross-check guidance.
- Added `.sldo/nettacker` confidentiality precondition checks before writing assessment artifacts.
- Added Docker runner architecture metadata and teardown handoff instructions.

## Debugging / Inspection Notes

- The first new structural-test run failed for the expected missing strings, confirming the red phase was real.
- Full workspace tests passed after the docs update.
- Repo-wide `cargo fmt --check` and broad package clippy exposed unrelated pre-existing issues; targeted checks on the changed Rust test passed.

## Rules For The Next Milestone

- Keep one-lab-run findings scoped as "observed" unless the exact Nettacker module YAML or official docs prove a universal rule.
- For security-tool skills, prefer structural tests for safety gates so prose drift is caught in CI.
- If `.sldo/` contains legitimate tracked files, check the specific confidential artifact subtree, not the whole `.sldo` namespace.

## Deferred Follow-Ups

- Consider a future Nettacker module-authoring milestone that inspects upstream module YAML directly and documents exact status-code/body-matching behavior per module version.

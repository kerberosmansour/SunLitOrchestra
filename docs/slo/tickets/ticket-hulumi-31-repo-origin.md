# Hulumi31 — repository origin for current runbook read lists

Status: in_progress. Source: https://github.com/kerberosmansour/hulumi/issues/31
Target: kerberosmansour/SunLitOrchestra. Lane: micro, docs-only.

The issue predates v4. Current canonical pointers target v4; v3 is a historical
artifact pinned by FNV/length tests. Implement the future-authoring intent in v4
and its byte-identical skill mirror. Do not repin legacy hashes or modify frozen
Hulumi runbooks. This is the conservative compatibility adaptation of the older
version-specific acceptance text, not a claim that v3 was modified.

## Contract

Goal: authors distinguish target-repository and upstream-context read paths before
opening files. Each reference names repository origin, revision and repo-relative
path. Upstream context requires a pinned commit; unavailable/ambiguous provenance
must be resolved before reading and cannot grant write scope.
Inputs/outputs: Markdown contract/template authoring only. No runtime interfaces,
dependency, lock, installer, frozen snapshot, deployment or host-support change.
No refactor. Forbidden: silently assume local paths, expand edit permission from
read-only context, edit old runbooks/v3 bytes, or claim headless host runtime tests.
Data Public. AI variance/measurement/outcome/security runtime contract: N/A for
this docs-only authoring correction; existing allow-list enforcement remains.

Exact file allow-list:
- docs/slo/templates/runbook-template_v_4_template.md
- skills/slo-plan/references/runbook-template_v_4_template.md
- docs/slo/tickets/ticket-hulumi-31-repo-origin.md

Read before change: AGENTS and operating-contract; getting-started/catalog/host
capabilities; v3/v4 templates and mirrors; existing v3 frozen-hash and v4 mirror/
canonical-pointer tests. User's dirty architect/execute skills are preserved.

## Validation / evidence

Baseline: isolated slo/hulumi31-repo-origin at f46e67a9183400abb0ef6f42fffef5924680bebc.
Current read-list row has only an ambiguous explicit-list placeholder; canonical
v4 selection and v3 frozen bytes are source-confirmed. No dependency is changed
or installed, and no new contrived Rust test is authored for a reversible prose edit.

Verify actual copied template bytes, all existing frozen v3 FNV/length constants,
Markdown table shape, reviewed cross-repository example and exact diff/cleanliness.
The existing upstream CI handles installer/template compatibility; do not claim
its runtime result before it runs. Update the source issue's single workpad and
open the concrete upstream PR. Issue31 closeout must explicitly explain the v4
adaptation rather than claim its old v3 artifact was edited.

Actual results: v4 now has explicit Target repository metadata and the read-list
origin/revision/repo-relative-path convention with target/upstream examples and
read-only scope limits. Both v4 copies are byte-identical. Both unchanged v3 copies
retain all32369 bytes and frozen FNV0xe9ee05c51944d857; no hash is repinned. Reviewed
all existing v4 required-field markers and additive compatibility; table row shapes
and git diff --check pass. No dependencies changed/installed and no runtime code
was compiled. Existing upstream CI and PR review remain pending.

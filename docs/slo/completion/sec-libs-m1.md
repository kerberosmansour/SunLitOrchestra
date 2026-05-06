# Completion Summary — sec-libs Milestone 1

## Goal completed

`/slo-sec-libs` now has an M1 declarations reader. It validates CycloneDX 1.6 declaration files offline, enforces the 10 MiB pre-parse cap, verifies an optional pinned official schema file by SHA-256, extracts component controls/capabilities/claims into a structured catalog, and documents the cache/SHA/symlink discipline needed before M2 matching.

## Files changed

- `skills/slo-sec-libs/SKILL.md` (new)
- `skills/slo-sec-libs/scripts/read-declarations.py` (new)
- `skills/slo-sec-libs/references/methodology-m1-reader.md` (new)
- `crates/sldo-common/src/toolflags.rs` (modified)
- `crates/sldo-install/tests/e2e_sec_libs_m1.rs` (new)
- `docs/skill-pack-catalog.md` (modified)
- `docs/ARCHITECTURE.md` (modified)
- `docs/slo/future/RUNBOOK-SLO-SEC-LIBS.md` (tracker/evidence update)
- `docs/slo/lessons/sec-libs-m1.md` (new)
- `docs/slo/completion/sec-libs-m1.md` (new)

## Tests added

- `crates/sldo-install/tests/e2e_sec_libs_m1.rs` — 10 structural-contract tests for the skill subtree, toolflags, import allow-list, argv-list discipline, 10 MiB cap, symlink-before-resolve discipline, strict jsonschema, schema URL/SHA capture, and cache methodology.

## Validation performed

- `cargo test -p sldo-common toolflags::tests::sec_libs`
- `cargo test -p sldo-install --test e2e_sec_libs_m1`
- Reader smoke against Hulumi declarations: 4 components, 2 claims.
- Reader smoke against SunLitSecurityLibraries declarations: 11 components, 11 claims.
- Malformed JSON fixture refused with parser location.
- 11 MiB fixture refused before parse.

## Known limitations

- System Python lacked `jsonschema`; smoke validation used a temporary venv. The skill pre-flight documents `pip install jsonschema` as the install hint.
- Cache population is not implemented in M1. The reader verifies a cache when `--expected-source-sha` is provided, but it does not clone or evict cache entries yet.
- M1 emits catalogs only. Matching starts in M2 and filing starts in M3.

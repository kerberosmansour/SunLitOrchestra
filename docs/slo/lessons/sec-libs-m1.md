# Lessons Learned — sec-libs Milestone 1

## What changed

- Added `skills/slo-sec-libs/SKILL.md` as the M1 declarations-reader skeleton.
- Added `skills/slo-sec-libs/scripts/read-declarations.py`, an offline Python reader for CycloneDX 1.6 declarations.
- Added `skills/slo-sec-libs/references/methodology-m1-reader.md` with schema pinning, cache layout, SHA verification, symlink discipline, size/depth caps, and cache-eviction policy.
- Added `crates/sldo-common::toolflags::sec_libs_allow_flags()` and `sec_libs_deny_flags()`.
- Added `crates/sldo-install/tests/e2e_sec_libs_m1.rs` with 10 structural-contract tests.
- Updated `docs/skill-pack-catalog.md` and `docs/ARCHITECTURE.md` so `/slo-sec-libs` is discoverable at HEAD.

## Design decisions and why

- **Offline reader with optional pinned schema path.** The script accepts `--schema-path`, verifies the official CycloneDX 1.6 schema SHA-256, validates against it, then validates against the reader extraction schema. This keeps runtime network access out of the skill while preserving the pinned-schema contract.
- **Reader schema after official schema.** Official CycloneDX validation proves the file is a valid 1.6 BOM. The reader schema proves the subset M1 needs is present and shaped for extraction.
- **No automatic cache deletion.** The runbook discussed refusing and wiping bad cache content. M1 refuses a cache SHA mismatch and surfaces the problem, but does not delete directories. Deleting cache content can become a later cache-management helper when cache population exists.
- **NFKC rejection on component identity fields.** Component `bom-ref`, `group`, and `name` are rejected if Unicode normalization changes them. That keeps homoglyph surprises out of later matching.

## Test patterns that worked well

- Structural tests assert the new skill files, toolflag denial, import allow-list, schema pin, 10 MiB cap, strict jsonschema language, and cache-discipline docs.
- Runtime smoke used a temporary jsonschema venv because the system Python did not have `jsonschema` installed.
- Live smoke against Hulumi and SunLitSecurityLibraries gave a useful early signal that both source declaration files satisfy the official schema and the reader extraction schema.

## Missing tests that should exist later

- Cache SHA mismatch behavior should get a fixture once cache population lands.
- Symlink-path refusal should get a runtime tempdir fixture in a future hardening pass; M1 has a static guard that checks symlink validation happens before `Path.resolve()`.
- The 200-level JSON depth cap is documented and implemented but not yet covered by a focused test.

## Rules for M2

- Treat the reader catalog as the only source of capability truth. Do not let matcher prose invent capabilities from model memory.
- Use `bom_ref` as the stable join key for claims and components.
- Preserve `controls` and `capabilities` as arrays; M2 should not parse the original property strings again.
- When multiple components match, surface the tiebreaker evidence rather than hiding it in prose.

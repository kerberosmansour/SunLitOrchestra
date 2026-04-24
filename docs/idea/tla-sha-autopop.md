---
name: tla-sha-autopop
created: 2026-04-24
status: ideation
tla_required: false
---

# Auto-populate `tools.toml` SHA-256 on first maintainer run

## The pain
Sherif just finished M5 of the skill-pack rebuild on 2026-04-24. The `/slo-tla` skill can fetch `tla2tools.jar` on first use, but only if `skills/slo-tla/tools.toml` has a real SHA-256. Today it ships with `sha256 = "UNSET"` as a deliberate honesty signal — we do not commit a hash we haven't computed. Result: the first maintainer to use the skill has to hand-populate the SHAs in a commit before the skill can work for any downstream user. Silent dependency.

## Five capabilities the user described without realizing
- Detect `sha256 = "UNSET"` at skill bootstrap and don't try to download until it's fixed.
- Provide a one-command helper to compute SHAs for every `[package]` in `tools.toml`.
- Optionally verify against upstream-published `.sha256` sibling files when those exist.
- Record who populated the hash and when (provenance).
- Keep the process transparent — every SHA in tools.toml should be traceable back to a specific upstream release and a specific human-or-CI computation.

## Approach A — manual helper script (conservative)
- **Effort**: 0.5 person-weeks
- **Wedge**: a Bash/Rust helper, `sldo-tla-sha`, that reads `tools.toml`, fetches each `url`, computes SHA-256, and prints the patch to apply. Human applies the patch in a commit.
- **Risks**: still manual; humans may skip the step.

## Approach B — first-run self-heal
- **Effort**: 1 person-week
- **Wedge**: `/slo-tla` detects `UNSET` at startup, offers to compute + update `tools.toml` automatically, then asks the user to commit the change.
- **Risks**: self-modifying skill config feels weird; has to be very careful not to silently edit `UNSET` to a bad hash.

## Approach C — CI workflow
- **Effort**: 1.5 person-weeks
- **Wedge**: a GitHub Action that runs on changes to `tools.toml`, populates missing SHAs, commits back with provenance metadata.
- **Risks**: requires GitHub Actions infra; slow feedback loop; doesn't help a user running `/slo-tla` offline.

## Recommendation
Approach A. It's a 0.5-week shippable improvement, keeps the honesty signal (`UNSET` remains a valid starting state), and has zero magic. B feels too clever for the marginal improvement. C is infrastructure that this repo doesn't have yet. Ship A; revisit B only if the manual step actually gets skipped in practice.

## Open questions for /slo-research
1. Do any of the upstream releases we pin (TLC, Apalache) publish `.sha256` sibling files alongside their artifacts? If yes, we should cross-check against those, not just compute locally.
2. Is there a risk of our helper computing SHA against a mirrored CDN that returns a different artifact than upstream? (Probably no — direct GitHub release URLs — but worth checking.)

# Manual smoke-test checklist — Runbook B1 M1 (`/slo-talk-to-users` + `/slo-verify` Pass 4 PII scan)

## Prerequisites

- [ ] All Runbook A tests still 42/42 green.
- [ ] B1 M1 structural tests: 8/8 green.
- [ ] `sldo-install --dry-run` shows `slo-talk-to-users` (19 skills total).

## Smoke fixtures

1. **Pre-interview** — `/slo-talk-to-users pre-interview` for fixture user "Sarah Patel from Acme Logistics, Head of Ops". Verify: artifact at `docs/biz/users/2026-04-25-sarah-patel.md`; `tier: confidential`; `mode_arg: pre-interview`; body has hypothesis grid + Mom Test question set + anti-patterns; write-time warning fired (target dir is git-tracked, has remote).
2. **Post-interview extraction** — `/slo-talk-to-users post-interview` with paste of fixture transcript. Verify: artifact at `docs/biz/users/2026-04-25-sarah-patel.md` (same file, refreshed) OR a new file; pain-extraction grid filled; specific recent bad day captured; hypothesis disposition row per pre-interview hypothesis.
3. **PII-scan fires on `docs/biz-public/` leak** — manually copy the post-interview artifact to `docs/biz-public/users/leaked.md`. Run `/slo-verify M1` (against any milestone). Verify: Pass 4 PII-scan flags the email + name patterns; non-zero scan exit; regression-test flow triggers per Pass 4 contract.
4. **Override mechanism** — create a `docs/biz-public/users/anonymised-case-study.md` with `pii_scan_override: true` and `tier_override_reason: anonymised pseudonyms — Alice / Bob / Carol — used in case study`. Run Pass 4. Verify: scan reports the override + reason; does NOT fail.
5. **Non-UK founder** — `/slo-talk-to-users pre-interview --jurisdiction us` (or any non-UK arg). Verify: canonical "v1 supports UK only" error.

## Notes

- _<empty until smoke run>_

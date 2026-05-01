# Lessons — biz-b1 M2 (`/slo-gtm`)

## What changed

- NEW `skills/slo-gtm/SKILL.md` — pure generator, no mode arg, output `docs/biz-public/gtm/strategy.md`.
- NEW `crates/sldo-install/tests/e2e_biz_b1_m2.rs` — 7 tests, all green.
- NEW `docs/slo/verify/biz-b1-m2-smoke.md` — 5 fixtures.
- ARCHITECTURE.md row added.

## Design decisions

- **3-segment cap is a soft limit** with override. Founders who genuinely need 4+ segments can document why; default rejection is anti-pattern guidance, not a hard refusal.
- **Hybrid motion requires primary + secondary explicitly**. "Hybrid" without specifying which sub-motion runs first is a cop-out the skill rejects.
- **PECR routing is mandatory before strategy doc is written** when direct marketing is in the channel set. Ties back to gate-4-gdpr-document via `/slo-legal triage`.

## Recommendations for M3

`/slo-product` is the first generator with a mode arg (`roadmap | metrics | okrs`). The structural test must assert the three modes are documented AND that an unknown mode is rejected. Output paths differ per mode — three separate output paths, all under `docs/biz-public/product/`.

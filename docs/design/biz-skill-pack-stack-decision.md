# Stack Decision — biz-skill-pack

## Chosen stack

- **Primary:** Markdown `SKILL.md` files under `skills/slo-<name>/` for the 15 new skills (advisor cluster + discovery/strategy generators + execution generators + team). Zero new Rust code, zero new runtime dependencies.
- **Shared scaffolding:** Markdown reference files under `references/biz/` at the repo root. Cited by skill prose; never imported as code. Outside `skills/` so `crates/sldo-install/src/install.rs:44-71`'s `discover_skills()` walker (which requires `<name>/SKILL.md`) ignores it.
- **External anchors (cited URLs, not fetched at runtime):** onenda.org, gov.uk HMRC manual VCM34080 / VCM3000 / VCM31000, gov.uk CEST tool, ico.org.uk DUAA pages, legislation.gov.uk (ERA 1996 s86, DUAA 2025), jpplaw.co.uk fixed-fee startup page (cost baseline). The pack does not enable WebFetch / WebSearch.
- **Verification surface (M1):** the existing `cargo test` baseline (`-p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`) gains a new test under `sldo-install` that asserts (a) `references/biz/` is not symlinked into `~/.claude/skills/` by `sldo-install` and (b) the oneNDA template body bytes in `references/biz/templates/onenda-uk.md` match the canonical SHA-256.

## Reason

The wedge is `/slo-legal` v1 — four UK templates (NDA, Contractor SOW, IP Assignment, T&Cs) plus a triage gate that hard-blocks `draft` for regulated / >£5,000 / counterparty-with-lawyer / GDPR contexts (research synthesis, paragraphs 1–4). The work is overwhelmingly LLM-native judgment: drafting prose contracts, advising on contractor-vs-employee triage, translating a legal doc into plain English, framing a fundraise pitch around HMRC qualification rules. The deterministic parts — the four hard-block predicates, the £5,000 threshold, the JPP Law cost baseline, the artifact-frontmatter schema, the oneNDA byte-verbatim check — are short enough to live as referenced markdown that every advisor skill cites, and the one byte-level invariant (oneNDA hash match) is enforced by a single test rather than a runtime engine. **The design must keep this work in Markdown skills + reference docs because (a) the research synthesis confirms the LLM-native judgment surface, (b) `crates/sldo-install/src/install.rs:44-71` confirms `references/biz/` outside `skills/` has zero installer interaction, and (c) the existing slo-security-embedding precedent (Phase 1: zero new Rust) shipped successfully on the same pattern.**

## Rejected alternatives

- **Per-skill self-contained (Option C from the dossier)** — rejected: triage gate logic, jurisdiction matrix, and £5,000 threshold would be duplicated in four advisor skills. Drift across legal / accounting / equity / fundraise after the first revision is the predictable failure mode, and the synthesis "PECR vs Article 13" finding would have to be patched in four places. Acceptable only if the pack stays at 1–2 advisor skills, which it does not.
- **Rust backend crate (`sldo-biz`)** — rejected: most of the work is LLM-native judgment, not algorithmic. Encoding "is this contractor SOW above the £5,000 threshold given scope, payment cadence, and IP scope?" in Rust if-statements is brittle and worse than the LLM. Locks the project into maintaining a legal-rules engine that has to refresh annually as JPP Law pricing and HMRC manuals shift. Slips the wedge from ≤1 week to ~6 weeks. Architecturally correct only if SLO's strategic direction moves away from LLM-native reasoning, which it has not (mirror of the slo-security-embedding rejection).
- **`skills/_biz-shared/` underscore convention** — rejected: `crates/sldo-install/src/install.rs:44-71` does not filter leading underscore; it filters only `.`-prefixed names. A `skills/_biz-shared/` directory would be silently ignored by the installer only if it lacked a `SKILL.md`, leaving an undocumented convention that breaks the moment someone adds a `SKILL.md`. `references/biz/` outside `skills/` is the unambiguous alternative.
- **`--jurisdiction us` / `--jurisdiction eu` config flags in M1** — rejected: research synthesis paragraph 3 — no surveyed prior art (Stripe Atlas / Clerky / SeedLegals) uses shared-prose-with-jurisdiction-flag; SeedLegals took six months to localise France. v1 ships UK only with an explicit "v1 UK only" error surface. US/EU is a v2 architectural pivot under a fresh `/slo-architect` pass, not a config flag.
- **DOCX rendering via `python-docx` or `docxtpl`** — rejected: SLO's convention is Markdown in the repo; DOCX export is not on the critical path. `docxtpl` is LGPL-3.0, which would taint a Rust workspace. Founders who want DOCX can convert with their own tooling.

## Non-negotiables (downstream cannot change these without migration)

- **Markdown-only.** This feature adds no Rust code. Any future runbook that introduces a compiled crate for biz logic starts from a fresh `/slo-architect` pass — this decision does not authorize it.
- **`references/biz/` location.** Shared scaffolding stays at `references/biz/` outside `skills/`. Moving it under `skills/` requires a migration entry that updates the installer source and the skill citations in lockstep.
- **Two-tier output convention.** `docs/biz/` (gitignored, confidential drafts) and `docs/biz-public/` (git-tracked, placeholder-only). Collapsing to one tier requires a migration entry plus an explicit founder waiver of the leakage risk.
- **oneNDA verbatim rendering with byte-hash check.** CC BY-ND 4.0 forbids derivatives. `/slo-legal draft nda` MUST render the canonical oneNDA bytes; `/slo-verify` regression test on the hash is non-negotiable. Switching to a different open-standard NDA requires a migration entry that updates the hash and the citation in lockstep.
- **No WebFetch / WebSearch in biz skills.** Citations only; no model-driven runtime fetching. Reversing this requires a fresh `/slo-architect` pass with a new threat-model row covering exfiltration via attacker-influenced URL.
- **Cost-baseline anchor: JPP Law.** `references/biz/cost-baseline-jpp-law-2026.md` is the citable, auditable source for ROI claims. Replacing with another firm requires a migration entry plus an updated retrieval date.
- **Existing `cargo test` baseline unchanged.** New tests for biz-skill-pack live inside the existing `sldo-install` crate (oneNDA hash check) or as workspace `[[test]]` entries that are added to `Cargo.toml`. The five-crate explicit test set in CLAUDE.md is preserved.

## Deferred decisions (not blocked by this feature; picked at the milestone or runbook that needs them)

- **Whether `/slo-talk-to-users` outputs structured JSON next to its Markdown summary.** Decided in Runbook B M1 when the skill is authored. Not a v1-of-the-pack concern.
- **Whether `/slo-fundraise` integrates with HMRC Advance Assurance form pre-fill (PDF/HTML form-fill).** Out of scope for v1; deferred to a future runbook with its own architect pass. v1 produces the brief the founder takes to their accountant.
- **Whether the pack adds a `--year <YYYY>` flag for cost-baseline refresh.** Defer to the first annual refresh cycle when JPP Law publishes 2027-28 pricing. Until then, retrieval-date stamps in the reference doc are the source of truth.
- **Whether `/slo-cofounder` produces a relationship-history log (multi-session memory).** Defer to Runbook C; v1 of `/slo-cofounder` is a single-session generator.

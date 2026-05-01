# Stack Decision — slo-security-embedding

## Chosen stack

- **Primary (Phase 1):** Markdown `SKILL.md` edits under `skills/slo-*/`. Zero new Rust code, zero new runtime dependencies.
- **Secondary (Phase 2 — `/slo-threat-model`, future runbook):** Python 3.11+ subprocess invoking **OWASP SecOpsTM v1.1.0** (pip-installable; bundles pytm + ChromaDB + HuggingFace embeddings; optional Ollama for offline LLM). Pure-LLM OTM 0.2.0 emission is the fallback when SecOpsTM will not install on the target platform.
- **Secondary (Phase 3 — `/slo-security-test`, future runbook):** `cargo audit`, `cargo deny` (already in the Rust toolchain), **Semgrep CE v1.160.0+** (pip or brew), **ast-grep v0.42.0+** (cargo install or brew), optional CodeQL CLI for nightly/weekly cadence. All emit SARIF.
- **Secondary (Phase 4 — `/slo-sec-libs`, future runbook):** `gh` CLI (already required by `/slo-ship`), Python jsonschema for CycloneDX 1.6+ declarations reader, SLO-owned intake repo (`kerberosmansour/slo-security-intake` or equivalent — to be created as a one-time bootstrap step before Phase 4 runs).
- **Schemas consumed (read-only):** OTM 0.2.0 (threat models), CycloneDX 1.6/1.7 Declarations (capability manifests from Hulumi + SunLitSecureLibraries), SARIF 2.1.0 (static-analysis findings from Semgrep / ast-grep / CodeQL).

## Reason

The wedge is `/slo-architect` emitting `SECURITY.md` + `docs/slo/design/<slug>-threat-model.md` as the upstream artifact every downstream skill cites (research synthesis §"Threat-model engine must emit into a schema, not a free-form document"). Shipping this as Markdown edits to existing skills keeps Phase 1 to ~3 weeks and preserves an upgrade path to a Rust-backed implementation later. The research synthesis is unambiguous that reasoning work (STRIDE sweep, abuse cases, mapping a requirement to `secure_authz` vs `secure_data`) is LLM-native and brittle in deterministic code, while operational work (running `cargo audit`, filing upstream issues, invoking ZAP) is shell-native and dangerous in the LLM — so the skill-pack + shell-out split (research Option A) is the right default. The Rust CycloneDX tooling gap (`cyclonedx-bom` 0.8.1 tracks spec 1.5 only; no 1.6+ declarations support in any Rust crate per the synthesis) would force either a hand-rolled emitter or a Node subprocess — but since SLO is the *consumer* of declarations (Hulumi and SunLitSecureLibraries emit, SLO reads), this gap collapses to a reader problem solvable with Python jsonschema in Phase 4. The design must handle threat-model output validation at emit time because LLM-driven STRIDE is documented as unstable across runs (Pure Storage 2025; FuzzingLabs 2025; ThreatCompute CCSW 2025), so OTM 0.2.0 schema validation is non-negotiable once Phase 2 ships.

## Rejected alternatives

- **Option B — OSCAL-anchored compliance-first stack** — rejected: no mature Rust OSCAL writer; OSCAL Component Definition v1.1.3 authoring cost per library is heavier than CycloneDX declarations; the research synthesis shows SLO's solo-operator audience does not pull FedRAMP/government-grade evidence regularly enough to justify the tax. Migratable later if a user comes forward with SP 800-53 Moderate requirements.
- **Option C — Rust-pure `sldo-sec` crate backing every skill** — rejected: reinvents what LLMs already do well (STRIDE, abuse cases), inherits the LLM-instability risk without the deterministic validator the research synthesis demands, and materially slips the wedge (~5–6 weeks vs ~3). Architecturally correct only if SLO's strategic direction moves away from LLM-native reasoning, which it has not.
- **Node subprocess for CycloneDX 1.6 emission** — rejected: SLO consumes declarations rather than emitting them; Hulumi and SunLitSecureLibraries own emission. A Node runtime would add a third language to the SLO install footprint (Rust + Python + Node) without a reader-side benefit.
- **Upstreaming CycloneDX 1.6 support to `cyclonedx-bom`** — rejected for this feature: valuable but out of scope; the wedge is embedding security into SLO, not modernizing a third-party Rust crate.

## Non-negotiables (downstream cannot change these without migration)

- **Markdown-only Phase 1.** Phase 1 ships zero Rust code. Any future runbook that introduces a compiled `sldo-sec` crate starts from a fresh architect pass — this decision does not authorize it.
- **SKILL.md frontmatter fields.** Each edited skill continues to advertise `name:` and `description:` per Claude Code's skill-loader contract. No renames in Phase 1.
- **Artifact locations.** `SECURITY.md` at the target repo root. `docs/slo/design/<slug>-threat-model.md` per feature. `docs/slo/critique/<slug>.md` unchanged from today's `/slo-critique` shape.
- **Skill invocation verbs are stable.** `/slo-architect`, `/slo-plan`, `/slo-critique`, `/slo-verify`, `/slo-execute`, `/slo-ideate`, `/slo-ship`, `/slo-retro` names cannot change; users invoke these daily. Adding new skills (`/slo-threat-model`, `/slo-security-test`, `/slo-sec-libs`) is additive and allowed.
- **Baseline test command unchanged.** `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` remains the declared baseline. Phase 1 must not introduce tests that live outside these crates.
- **SLO-owned intake repo as default upstream-filing channel.** `/slo-sec-libs` Phase 4 files into the intake repo. Third-party filing (Hulumi, SunLitSecureLibraries) requires an explicit `--file-upstream` flag with a 40-issues/hr client-side cap.
- **Schema validation gates.** When SecOpsTM or a pure-LLM path emits OTM, the output is validated against `otm_schema.json` before any file is written to disk. Invalid OTM is an error, not a warning.

## Deferred decisions (not blocked by this feature; picked at the phase that needs them)

- **SecOpsTM install path on macOS arm64.** Phase 2 begins with a spike install; if SecOpsTM will not install cleanly, the pure-LLM fallback becomes Phase 2 primary and SecOpsTM becomes optional.
- **CycloneDX Property Taxonomy namespace.** Phase 4 starts with a vendored SLO namespace (e.g., `cdx:sunlit:crypto:*`). If the public Taxonomy gains equivalents, Phase 4's reader adopts them and deprecates the vendored terms.
- **Whether `/slo-security-test` runs DAST (OWASP ZAP, Dastardly) in Phase 3.** Deferred to Phase 3 architect pass — DAST is only valuable when the target repo has a runnable service. For markdown-only targets it is noise.

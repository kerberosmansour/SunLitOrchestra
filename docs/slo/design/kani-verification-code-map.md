# Code Map — kani-verification (brownfield)

## Four-object summary

1. **`skills/slo-tla/`** — the structural sibling `/slo-kani` mirrors. Owns: `SKILL.md` (suitability gate, prereq cascade, method-dispatch table, hard gates, handoff), `tools.toml` (pinned artifact + SHA), `references/methodology-*.md` (per-phase loaded), `evals/*.md` (seven scenarios). Copy this shape; swap TLA+/TLC concepts for Kani/`cargo kani`.
2. **`skills/slo-architect/SKILL.md`** — Step 5 sets `tla_required`. M3 adds a parallel "Decide `kani_required`" step + the candidate-module shortlist. Frontmatter-key list at the top of the Outputs section is where `kani_required` is documented.
3. **`docs/slo/templates/runbook-template_v_4_template.md`** — §5 "High-Level Design for State Modeling / Formal Verification" (line ~254). M3 adds the Kani proof-obligation sub-block here, additive to the TLA+ prose.
4. **`xtasks/sast-verify/tests/sap_imp_m5_agents.rs`** — the structural-contract test exemplar (frontmatter assertions, output-path safety, SHA baseline). M1's `/slo-kani` structural test is modeled on it.

## Exemplar code to copy

- `skills/slo-tla/SKILL.md` — the prereq cascade (numbered, "do not skip, do not try it anyway"), the suitability gate ("X is not the right tool here is a legitimate output"), the "Common Gates And Anti-Patterns" block (refuse to mark verified when bound not stated / counterexample suppressed / naive variant passes silently). This honesty discipline is the single most important thing to carry over.
- `skills/slo-tla/tools.toml` — pinned-URL + SHA-256 shape; adapt to `cargo install --locked kani-verifier@<pin>` + `cargo kani setup`.
- `docs/slo/design/sast-rulegen-skill-pack-overview.md` — the overview frontmatter + "Frontmatter rationale" section shape (already followed in this run's overview).
- `crates/sldo-install/src/install.rs::discover_skills()` — confirms a new `skills/slo-kani/SKILL.md` is auto-discovered; no installer change needed.

## Anti-exemplar code not to copy

- The **removed legacy `sldo-plan` / `sldo-run` CLIs** (gone in the 2026-04 cleanup) — do NOT reintroduce a Rust binary to "drive Kani." The skill drives the subprocess directly; a binary would reverse the canonical-interface decision and add crates.io version-discipline burden ([[project_crates_io_published]]).
- Any pattern that lets the **LLM's narration override a tool verdict** — the threat model's central AI risk. The verdict comes from `cargo kani` output, never from prose.
- `tla_required: true for simple CRUD to look rigorous` (architect anti-pattern) — the Kani equivalent: do not set `kani_required: true` for a non-Rust target or a Rust target with no unsafe/arithmetic/invariant kernels. False positives waste a milestone.

## Dangerous seams

- **`docs/slo/templates/runbook-template_v_4_template.md`** — large, consumed by `/slo-plan` and possibly asserted by structural tests. Inspect for any test that pins §5's line count or headings before editing. Additive sub-block only.
- **`skills/slo-architect/SKILL.md` frontmatter-key list** — `/slo-critique` and `/slo-plan` read these keys. Adding `kani_required` must keep the existing four keys' types and defaults intact.
- **`xtasks/sast-verify/tests/`** — a new structural test must not perturb the existing `sap_imp_m5_agents.rs` SHA baseline; add a sibling test file, do not edit the baseline.
- **Kani's nightly-toolchain requirement** — `cargo kani` needs a specific Rust toolchain via `cargo kani setup`. Keeping the demo crate in a *separate repo* (M4 decision) isolates this from the main workspace's `cargo test -p sldo-common -p sldo-install -p sldo-research` baseline. Do not add the demo crate as a workspace member.

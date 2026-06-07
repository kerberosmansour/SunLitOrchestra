# Code Map — innovation-loop (brownfield)

The Innovation Sandbox loop is a Markdown-skill-pack addition to an existing
skill-pack repo. There is no application code to map; the "code" is the skill +
template + test conventions the new pack must mirror. This map points the
executing agent at the exact files to copy from and the seams to respect.

## Four-object summary

1. **Skill files** — `skills/slo-<name>/SKILL.md`. The portable unit. Discovered
   by `discover_skills()` purely on the presence of `SKILL.md`. Frontmatter
   (`name`, `description`) + prose body (Inputs → Output → Method → Handoff →
   Loops footer). The 8 new skills are new instances of this object.
2. **Templates** — `docs/slo/templates/*.md`. The output contract a skill emits
   (e.g. `/slo-plan` emits the v4 runbook). The new
   `experiment-book-template_v_1.md` is a new instance, modelled structurally on
   `runbook-template_v_4_template.md` but inverted to "Definition of Learned".
3. **Structural-contract tests** — `xtasks/sast-verify/tests/*.rs`. Pure-Rust
   tests that parse `SKILL.md` / agent frontmatter and assert shape +
   output-path safety. The new test is a sibling appended here.
4. **Registries** — `docs/skill-pack-catalog.md`, `docs/LOOPS-ENGINEERING.md`,
   and the host overlays (`CLAUDE.md` / `copilot-instructions.md` / `AGENTS.md`).
   Markdown indexes the new pack must be registered in.

## Exemplar code to copy

- **Skill style + frontmatter + Loops footer** → [skills/slo-ideate/SKILL.md](../../../skills/slo-ideate/SKILL.md).
  Copy: the folded `description` with trigger phrases + "do not use when…", the
  `# /slo-x — tagline` heading, the Inputs/Output/Method/When-to-stop/Handoff/
  Anti-patterns spine, and the closing `**Loops**:` footer line.
- **Interactive, one-section-at-a-time authoring discipline** → [skills/slo-plan/SKILL.md](../../../skills/slo-plan/SKILL.md).
  Copy: the "refuses to generate the whole thing in one shot" stance and the
  per-section confirm-before-advancing loop. The phase skills should fill ONE
  Experiment-Book section per invocation, confirm, and hand off — never author
  the whole Book at once.
- **Section-targeted template with a frozen tracker + honest statuses** →
  [docs/slo/templates/runbook-template_v_4_template.md](../templates/runbook-template_v_4_template.md).
  Copy: the Milestone-Tracker shape (→ Experiment Tracker), the numbered-section
  layout, and the honest-exit-status idiom (→ the 8-state exit vocabulary).
- **Structural-contract test shape** → [xtasks/sast-verify/tests/sap_imp_m5_agents.rs](../../../xtasks/sast-verify/tests/sap_imp_m5_agents.rs).
  Copy: `serde_yaml_ng::from_str` frontmatter parse, required-field assertions,
  the output-path safety checks (no absolute paths, no `..`, whitelist-prefix),
  and (optionally) a SHA-256 baseline pin for the template.
- **Two-tier confidential/public + write-time warning idiom** →
  `skills/slo-talk-to-users/SKILL.md` and the `docs/biz/` `.gitignore` rule
  (`.gitignore:48-52`). Copy: the data-classification field + write-time warning
  when a target dir is git-tracked with a remote and the tier is confidential —
  reused for the Experiment Book's `§0` classification + scratch-dir ignore.

## Anti-exemplar code not to copy

- **The legacy `sldo-plan` / `sldo-run` CLIs** — removed in the 2026-04 cleanup.
  Do NOT re-introduce a binary to "drive" the loop; the skills are the canonical
  interface. (`CLAUDE.md`: "All other Rust code … was removed … the skills are
  the canonical interface now.")
- **The parked `sldo-tauri` desktop UI** — do not wire the loop to any UI surface.
- **`references/biz/` as a discovery location** — it is shared scaffolding, NOT a
  skill directory (`discover_skills()` ignores repo-root `references/`). New
  per-skill references go under `skills/slo-<name>/references/`, not repo-root.
- **v3 runbook template** — historical; mirror v4, not v3.

## Dangerous seams (inspect before editing)

- **`crates/sldo-install/src/install.rs` `discover_skills()` (≈ lines 45–73)** —
  the install gate. The new skills need NO code change here (gate = presence of
  `SKILL.md`), but read it before assuming so, and do not add a skip-rule that
  would exclude a new directory.
- **`xtasks/sast-verify/tests/` baselines** — some tests pin SHA-256 of specific
  `SKILL.md` files (e.g. `/slo-critique`). Appending a new test must not perturb
  existing baselines; run the full sast-verify gate after adding it.
- **`docs/skill-pack-catalog.md` count line** — "Shipped skills at HEAD: 41" and
  the `ls skills/ | grep -v README` reconciliation. Adding 8 skills must update
  this count to 49 (and the per-section tally), or the catalog self-check drifts.
- **`docs/LOOPS-ENGINEERING.md` anti-process-theatre rule** — a new loop section
  must point at a concrete user-visible outcome the Sprint loop does not already
  produce. The Innovation Sandbox loop qualifies (pre-idea → promotable candidate
  or composted dead-end); state that outcome explicitly when adding the section.
- **Host overlays must stay overlays** — register the pack in the catalog
  (canonical) and add only a short pointer in `CLAUDE.md` / `copilot-instructions.md`
  / `AGENTS.md`; do not let an overlay become a competing catalog.

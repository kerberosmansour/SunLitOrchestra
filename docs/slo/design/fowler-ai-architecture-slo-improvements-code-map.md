# Brownfield Code Map — Fowler AI Architecture SLO Improvements

## Four-Object Summary

| Object | Role | Why it matters |
|---|---|---|
| Skill orchestrators | `skills/<skill>/SKILL.md` files that decide when and how agents act | Primary installed interface; must remain concise and host-neutral. |
| Skill-local references | `skills/<skill>/references/*.md` files loaded on demand | Holds methodology detail without bloating SKILL.md context. |
| Shared references | `references/templates/`, `references/security/`, `references/biz/`, `references/sast/` | Cross-skill discipline and authority files. |
| Structural tests | Rust tests under `crates/sldo-install/tests/` and `xtasks/sast-verify/tests/` | Deterministic enforcement for Markdown contracts. |

## Exemplar Code / Docs To Copy

| Path | Why it is an exemplar |
|---|---|
| `skills/slo-plan/references/methodology-milestone-authoring.md` | Good example of moving detailed procedure out of SKILL.md while keeping the orchestrator concise. |
| `references/templates/restate-and-confirm.md` | Good compact shared-discipline reference with clear trigger and failure mode. |
| `docs/slo/future/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md` | Prior self-hosted skill-pack improvement runbook with explicit file allow-lists and structural tests. |
| `docs/slo/templates/runbook-template_v_4_template.md` | Current canonical v4 Contract Block and evidence-log shape. |
| `skills/slo-critique/personas/eng.md` | Persona-specific review logic with concrete scenario discipline. |

## Anti-Exemplars / Do Not Copy

| Path / pattern | Why not to copy |
|---|---|
| Broad SKILL.md growth without methodology references | Violates the context-window balance documented in `docs/PARADIGM-OVER-ENGINEERING-FOR-SIMPLICITY.md`. |
| Template-only behavior changes | Installed skills may not consume repo mirror templates; update skill-local references and SKILL.md triggers too. |
| New free-form Contract Block vocabulary | Existing skills rely on fixed rows and fixed data-classification vocabulary. |
| Any one-shot runbook generation pattern | `/slo-plan` is designed around milestone contracts and critique before execution. |

## Dangerous Seams

| Seam | Risk | Guardrail |
|---|---|---|
| Skill path names | Renames break installer discovery and user invocation | No skill path renames. |
| Contract Block row names | Silent drift breaks critique/verify expectations | Additive rows only; structural tests assert row presence. |
| Host runtime claims | Codex/Copilot are interactive only today | Cite `docs/slo/design/agent-host-capabilities.md`; no new runtime promise. |
| AI source notes | Prompt injection or hallucinated transcript details | Treat user notes as input; source claims from external pages or repo-local artifacts. |

## Coverage Gaps To Close

- No structural tests currently assert the proposed reversibility/code-map artifacts.
- No existing `/slo-verify` pass exercises AI nondeterminism tolerance.
- Ticket-sized SLO contracts do not yet mirror exemplar/refactoring/AI-tolerance rows.

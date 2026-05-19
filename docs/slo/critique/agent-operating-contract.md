# Critique - agent-operating-contract

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| F-CEO-1 | ceo | defer | M2 / M3 | Copilot install-root and custom-agent parity are real product decisions, but M1 correctly avoids mixing them into the always-on contract work. | A future agent tries to "fix Copilot" by changing installer roots and agent profiles in the same PR as instruction wiring; review becomes too wide and a compatibility break hides inside docs churn. | Keep M1 docs/test-only. Expand M2 only after M1 passes and explicitly decide whether `.copilot/skills` remains a compatibility root alongside GitHub's documented `.github/skills` project root. |
| F-ENG-1 | eng | auto-fix | M1 BDD | The structural test should guard both existence and size of the always-on files, otherwise the operating contract can become the same long generic rule file this runbook is trying to avoid. | A well-meaning contributor adds 300 lines of style advice to `.github/copilot-instructions.md`; Copilot receives noisy always-on context and stops using the skill catalog efficiently. | Include nonblank-line caps in `e2e_agent_operating_contract.rs` for `references/agent/operating-contract.md` and `.github/copilot-instructions.md`. |
| F-SEC-1 | security | ask | M1 Contract Block | The abuse scenario says stale runtime claims should be rejected, but M1's allow-list does not include the host capability matrices. | A future edit adds "Copilot custom agents now mean multi-agent dispatch is supported" to `.github/copilot-instructions.md`; no matrix row changes, so downstream users cannot tell whether the claim is grounded. | In M1, keep runtime claims out of the new always-on files and link to the capability docs. In M2, update capability matrices with current source dates before making stronger claims. |
| F-DESIGN-1 | design | N/A | whole runbook | N/A - no UI surface in this runbook. | N/A | N/A |
| F-ENG-2 | eng | auto-fix | M3 structural tests | Copilot profile parity needs tests over the GitHub-native profile files, not only over the capability docs. | A future edit adds `.github/agents/slo-security-reviewer.agent.md` with broad tools or no fallback path; docs say parity exists but the agent can edit arbitrary files. | Add M3 structural tests that assert the four `.github/agents/*.agent.md` files exist, use bounded tools, point to `/slo-critique` or `/slo-verify`, and preserve the no-runtime-harness wording. |
| F-SEC-2 | security | auto-fix | M3 custom-agent profiles | The verification profile can execute commands, so its prompt must keep execution tied to milestone-declared checks and the DAST smoke-service gate. | Copilot selects `slo-verification-lead` and the profile starts running broad scanners or networked DAST without a runnable target in the runbook. | In the profile body, restrict execution to milestone-declared commands, require scope expansion before widening, and state that DAST is N/A without a runnable smoke service. |
| F-CEO-2 | ceo | auto-fix | M3 docs | Porting Copilot profiles could be misread as Codex parity unless Codex's fallback is stated near the new Copilot rows. | A Codex user sees `.github/agents/` in the catalog and expects Codex subagent profiles to load automatically, then treats a missing feature as a product bug. | Update capability docs and catalog to say Codex has no shipped SLO host-native custom-agent equivalent and uses `/slo-critique` / `/slo-verify` directly. |

## Resolution Notes

- F-ENG-1 accepted into M1 test design.
- F-SEC-1 resolved by keeping M1 wording route-to-matrix only; no runtime parity claims in the new Copilot file.
- F-CEO-1 remains deferred as the explicit M2/M3 sequencing rule.
- F-ENG-2 accepted into M3 test design.
- F-SEC-2 accepted into `slo-verification-lead` profile boundaries.
- F-CEO-2 accepted into M3 capability docs and catalog updates.

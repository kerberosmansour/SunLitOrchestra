---
name: slo-second-opinion
description: >
  Use this skill when you want an independent review of a plan, diff, or
  decision — "get a second opinion", "what would Codex say", "cross-check this
  with another model". Shells out to a different provider CLI (Codex or Gemini)
  with the same context and surfaces the disagreement, not a vote. Do not use
  as a tiebreaker — it's a disagreement finder, not an arbitrator.
---

# /slo-second-opinion — surface cross-model disagreement

You are not here to resolve which model is "right." You are here to surface where the current host agent and another provider see the same thing differently, so the user can read both perspectives and decide.

## Inputs

- A target: a runbook, a PR diff, a specific design doc, or an arbitrary question.
- Optional: a focus area ("security", "scope", "performance").

## Prereq cascade

Try in this order, use the first one on PATH:

1. `which codex` — OpenAI Codex CLI.
2. `which gemini` — Google Gemini CLI.

If neither is present:

> No second-opinion provider found on PATH. Install one:
> - Codex CLI: https://github.com/openai/codex-cli
> - Gemini CLI: https://github.com/google-gemini/generative-ai-cli
> Then re-run this skill.

Exit non-zero. Do not silently fall back to asking the current host to "pretend to be Codex" — that defeats the purpose.

## Method

1. Frame the prompt. Identify the exact artifact under review (runbook section, diff hunk, design paragraph). Include the constraint ("be adversarial", "focus on X", "don't rewrite — review").
2. Dispatch to the detected provider with the framed prompt.
3. Capture the response.
4. Produce a diff-of-findings table. The "current host said" column captures whatever the agent running this skill produced; the "Provider said" column captures the second-opinion provider's raw response.

| finding | current host said | Provider said | Overlap / disagreement |
|---------|-------------------|---------------|------------------------|

5. Surface the disagreements, not the overlaps. Overlaps are signal ("both models flagged X, probably real"); disagreements are where the user earns decision value.

## Gates — do not emit when

- The provider CLI returned an obvious error (network, auth, rate limit). Surface the error, don't paper over.
- The provider's response is a restatement of the target rather than a review.
- Both models agree completely (the skill's value is surfacing delta — if there is none, say so in one line).

## Anti-patterns

- Concluding "Codex disagrees, therefore the current host was right" or vice versa. This skill provides a comparison, not a verdict.
- Hiding the provider's response behind a host-worded summary. Show the raw provider output alongside the current host's, let the user read both.
- Rate-limiting disagreements ("we found 3 things that differ, showing top 2"). Show all — disagreement is rare and worth seeing.

## Handoff

After the disagreement is surfaced, the user decides. No downstream skill to suggest.

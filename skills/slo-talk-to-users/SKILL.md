---
name: slo-talk-to-users
description: >
  Use this skill when a UK seed-stage founder needs to prepare for a user
  interview, draft an interview script, or extract structured signal from a
  completed interview. Generator pattern (single-mode, no four-mode contract).
  Output: `docs/biz/users/<date>-<name>.md` (confidential — interview transcripts
  contain real persons' names, emails, employer / role detail). Founder's repo
  `.gitignore` MUST exclude `docs/biz/` — skill emits a write-time warning if
  the target dir is git-tracked AND a remote exists. UK only in v1; non-UK
  emits the canonical "v1 supports UK only" error from `references/biz/
  jurisdiction-uk.md`. The first PII-shaped generator in the biz pack — paired
  with the `/slo-verify` Pass 4 PII-pattern scan that lands in this same
  milestone (Runbook B1 M1).
---

# /slo-talk-to-users — interview prep + extraction generator

You are a YC partner who has watched too many founders treat user interviews as a feature-validation exercise instead of a pain-discovery exercise. This skill produces two artifact shapes:

1. **Pre-interview**: a structured script + open-ended question set that maps to the founder's hypotheses without leading the user. Cites Mom Test discipline (no leading questions; ask about specific recent days, not hypothetical futures).
2. **Post-interview**: an extraction template that captures the user's pain, workarounds, willingness to pay, and one specific recent bad day with quotes.

Generator pattern — single-mode, no four-mode contract. No hard-block gates (this skill produces interview docs, not regulated documents). The advisor cluster's `references/biz/triage-gate.md` predicates do NOT apply here.

## Output

One artifact per invocation at:

- **`docs/biz/users/<YYYY-MM-DD>-<kebab-slug-of-name>.md`** (confidential, gitignored).

Frontmatter follows [references/biz/artifact-schema.md](../../references/biz/artifact-schema.md):

```yaml
---
name: <YYYY-MM-DD>-<kebab-name>
created: <YYYY-MM-DD>
tier: confidential
archetype: generator
skill: slo-talk-to-users
jurisdiction: uk
mode_arg: pre-interview | post-interview
---
```

Body is a Markdown checklist + question set + extraction grid (see "Body shape" below).

## Founder repo discipline (load-bearing)

Real interview transcripts contain:

- Real persons' names ("I spoke with Sarah Patel from Acme Logistics")
- Real email addresses (in follow-up notes)
- Real employer / role / sector detail
- Real revenue / spend / team-size figures

The founder's repo MUST exclude `docs/biz/` from version control. This skill writes a **WRITE-TIME WARNING** when ALL of these conditions hold simultaneously:

- The target write directory is inside a git-tracked repository.
- A `remote.origin.url` is configured.
- The artifact's `tier` is `confidential`.

The warning text:

> ⚠️ **STOP — confidential artifact write to a git-tracked repo with a remote.** Path: `docs/biz/users/<artifact>.md`. Add `docs/biz/` to your repo's `.gitignore` BEFORE you commit. Pushing this artifact to a public remote leaks deal-sensitive interview content (real persons' names, emails, sector details) to anyone running GitHub code search. Second-line defense: `/slo-verify` Pass 4 PII-pattern scan over `docs/biz-public/`. See [SECURITY.md](../../SECURITY.md) "Biz skill pack — additional rules" → "Founder personal data — handling discipline".

This warning fires REGARDLESS of whether the artifact is being written to `docs/biz/` (correct location) or `docs/biz-public/` (incorrect — should not happen for `tier: confidential`).

## Pre-interview body shape

When invoked before an interview, produce:

### Hypothesis grid

| Hypothesis | Signal that confirms | Signal that disconfirms |
|---|---|---|

The founder fills in 3-5 hypotheses they want to test. The skill produces the column headers + asks for the hypotheses if not provided.

### Mom Test question set (no leading questions)

A canonical 8-12 question set drawn from Rob Fitzpatrick's *The Mom Test* (2013):

1. "Tell me about the last time you tried to [achieve outcome X]." — anchored in a SPECIFIC recent event, not hypothetical.
2. "What was hard about that?" — pain-discovery; the user's words, not the founder's frame.
3. "Why was that hard?" — laddering down to root cause.
4. "What did you do about it?" — actual workaround, not hypothetical.
5. "What would have to be true for you to switch to a new tool?" — willingness-to-switch signal.
6. "Have you looked for a tool to do this?" — discovery channel + alternatives signal.
7. "If you found one, what would it have to do?" — feature priorities, in user's framing.
8. "What's not working with what you do today?" — gap signal.
9. (optional, late) "Would you pay for it?" — only after pain is established; too early and it's fishing.
10. (optional) "Who else has this problem? Could you introduce me?" — referral + market-size signal.

Skill prose CALLS OUT THE ANTI-PATTERNS:

- "Would you use [my product]?" → leading; replace with "what do you do today".
- "Wouldn't it be great if X?" → confirmation-bias bait; replace with "tell me about the last time".
- "How much would you pay?" → hypothetical; replace with "what do you spend today on this".

### Logistics

- Time box: 25-40 min.
- Recording: ASK PERMISSION (skill includes the consent script — short, GDPR-compliant under legitimate-interest for legitimate market research).
- Note-taking: name + role + sector + interview date in frontmatter; quotes in body.

## Post-interview body shape

When invoked after an interview, produce the extraction template:

### Subject

- Name, role, sector, employer (real values — confidential tier).
- Interview date.
- Channel (cold outreach? warm intro? referrer name).

### Pain extraction (Mom Test grid)

| Question | User's answer (paraphrased) | Direct quote | Pain signal (1-5) |
|---|---|---|---|

For each Q from the question set, capture the user's answer + one direct quote (verbatim) + a subjective pain signal score (1 = "they don't have this problem", 5 = "they would pay tomorrow").

### Specific recent bad day

One paragraph: when did this last bite them? What did they do? How much time / money / customer-trust did it cost? Direct quote where possible.

### Willingness to pay

- Current spend on alternatives (real £ figure if disclosed).
- Switching cost (effort, vendor lock-in, contracts).
- Quote on price sensitivity if elicited.

### Hypothesis disposition

For each hypothesis from the pre-interview grid:

- **Confirmed** — what evidence?
- **Disconfirmed** — what evidence?
- **Ambiguous** — what would resolve it?

### Follow-up

- Did they offer an introduction to others? (referral name + permission?)
- Did they ask to be kept updated? (newsletter signal — beware GDPR PECR consent for marketing — route any PECR-direct-marketing question to `/slo-legal triage` per [`references/biz/ico-duaa-index.md`](../../references/biz/ico-duaa-index.md)).
- Action items for the founder.

## UK-only jurisdiction

UK only in v1. Same canonical error from [`references/biz/jurisdiction-uk.md`](../../references/biz/jurisdiction-uk.md): "**v1 supports UK only; US/EU is a v2 architectural pivot — see [docs/slo/design/biz-skill-pack-overview.md](../../docs/slo/design/biz-skill-pack-overview.md) for the v2 design rationale.**"

Cross-border interviews (UK founder talking to a US user) are permitted because the SKILL operates on the UK founder's intent / process; the user's location doesn't change the skill's behaviour. Non-UK FOUNDER context is what triggers the rejection.

## No WebFetch / WebSearch

Same as the rest of the biz pack. Founder pastes interview content; the model does not fetch external URLs at runtime. External anchors (Mom Test source, ICO DUAA pages) are emitted as citations only.

## Refusal patterns (in priority order)

1. Unknown mode-arg (must be `pre-interview` or `post-interview`) → "Unknown mode_arg `<value>`. /slo-talk-to-users accepts `pre-interview` or `post-interview`."
2. Non-UK founder jurisdiction → canonical UK-only error.
3. Founder asks the skill to draft a marketing email or recruitment outreach to interviewees → REFUSE. Marketing outreach + GDPR consent is `/slo-marketing` + `/slo-legal triage` territory; this skill is for the interview itself, not the outreach to set it up.
4. Founder asks to write the interview into `docs/biz-public/` despite real PII → REFUSE; the skill writes to `docs/biz/users/` only for `tier: confidential`. Anonymisation (replace real names with pseudonyms) is the founder's task; once anonymised, they may move to `docs/biz-public/users/` manually with `pii_scan_override: true` + `tier_override_reason` per the artifact schema.

## Handoff

After `pre-interview`: suggest the founder block 30 min on the calendar + send the consent script. After `post-interview`: suggest `/slo-gtm` (M2 — once shipped) to translate aggregated user signal into ICP / segmentation, OR `/slo-product metrics` (M3) to translate the post-interview pain signal into PM-side activation / retention metrics.

## What this skill is NOT

- Not a recruitment / outreach tool — that's `/slo-marketing` (M4) + `/slo-legal triage` (gate-4-gdpr-document for direct-marketing PECR considerations).
- Not a feature-validation tool — interviews surface PAIN, not feature votes. If the founder is asking "should I build X?", redirect to `/slo-ideate` for the wedge question + this skill for grounding.
- Not a CRM. The artifact is a single interview's record. Aggregation across interviews is the founder's spreadsheet / Notion / etc.
- Not jurisdiction-aware on user side — UK founder running this skill on a US user is fine; non-UK FOUNDER triggers the rejection.

---

**Loops**: User-interview loop — see [docs/LOOPS-BUSINESS.md#user-interview-loop](../../docs/LOOPS-BUSINESS.md#user-interview-loop).

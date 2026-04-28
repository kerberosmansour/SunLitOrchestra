---
name: business-skill-improvements
runbook: docs/RUNBOOK-BUSINESS-SKILL-IMPROVEMENTS.md
critiqued: 2026-04-28
personas_run: [ceo, eng, security]
design_persona_skipped: yes — no UI surface
---

# Critique — business-skill-improvements

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| B-1 | ceo | hold-scope | M1 / source-verification budget | "Time-box per row to ~5 minutes" — for a 29-row regulator enum + 3 statute anchor files (each with 5-15 sections) + HMRC VCM refresh + ICO DUAA refresh, this is ~10-15 hours of manual verification. Real cost. | Author starts M1; spends 4 hours on regulator enum; budget-pressure leads to "approximate verify" on statute anchors. Discipline drifts. | Hold the budget rule; **explicit acceptance** that M1 is the longest milestone. Add follow-up rule: "if a row cannot be verified within budget, mark `last_checked: pending-verification` and flag for next sprint — do not weaken." Already in Notes; reinforce in BDD. **hold-scope** + **auto-fix** to M1 BDD. |
| B-2 | ceo | reduce-scope | M4 / Mom Test quotation | Fitzpatrick *The Mom Test* (2013) quotation is constrained by copyright. Snippet-permitted excerpts only OR paraphrase with attribution. | Agent attempts verbatim quote of 5+ pages; copyright violation; book is widely considered fair-use-excerpted but high-bandwidth verification required. | Cut to attribution-only (cite ISBN + page numbers without verbatim long-quote). Mom Test question structure (the schema) is the discipline; the actual question text can paraphrase Fitzpatrick's framing without verbatim copying. **auto-fix** applied to M4 contract. |
| B-3 | ceo | ask | M2 conversational discipline | Conversational intake works for well-paced founders; for time-pressed founders ("I have 15 min between calls — just give me the contract"), the conversation feels frictional. | Founder cancels mid-elicitation because they're frustrated by the question pace; agent must decide: complete intake or fall back? | Add an optional `--rapid-intake` flag that batches F1-F6 into a single 6-question prompt (still structured, just faster). Conversational is default; rapid is opt-in for founders who explicitly trade off. **ask** user to confirm. |
| B-4 | eng | auto-fix | M2 / sister contract verbatim test | F1/F4/F5 verbatim across sisters — byte-compare assertion sensitive to whitespace differences (trailing spaces, line-ending flavor). | Agent edits one sister contract; trailing whitespace differs from source; test fails for cosmetic reason; agent re-saves with `:set noeol` quirk; new error class. | Normalize whitespace in the test (strip trailing + normalize line-ending) before byte-compare. **auto-fix** applied to M2 E2E test description. |
| B-5 | eng | ask | M3 / Python snippet provenance | Snippet emitted is stdlib-only — but no licensing declaration. Founder copies snippet into their own repo without license. | Founder commits the snippet to a private repo; later open-sources the repo; snippet's license is ambiguous. | Add MIT license header to every emitted snippet (one-line `# SPDX-License-Identifier: MIT`). **ask** user — MIT or other? |
| B-6 | eng | hold-scope | M5 / `baseline_ref:` field as optional | Schema extension is "additive optional" — but a generator artifact that cites a baseline file SHOULD have the field. "Optional" makes it unenforceable. | A future generator skill writes an artifact citing CAC numbers without `baseline_ref:` frontmatter; structural-contract test passes (field is optional); founder reads stale numbers and quotes them. | "Optional in the schema; required by generator skill cross-skill citation tests." Schema flexibility lets older artifacts parse; new test enforces emission. **hold-scope** — clarify in M5 contract. |
| B-7 | eng | auto-fix | M2 / `legal-intake-form.md` rename | Rename is in scope but the runbook doesn't document the cross-reference update path (issue #19's existing comment links to the old `legal-intake-form.md` URL). | Rename happens; existing GitHub comment URLs 404; reviewer can't follow the link. | Pre-flight step: enumerate every cross-reference in the repo (and explicitly NOTE that the existing GitHub issue comments point to old URLs — those won't auto-update; document as known minor breakage acceptable for this rename). **auto-fix** applied to M2 pre-flight + Notes. |
| S-1 | security | auto-fix | M2 / F3 deal-value evaluation | F3 has `deal_value_basis` enum + `deal_value_known_with_confidence` enum. But founder could lie ("yes, total ex-VAT" when monthly). | Founder under-states deal value to avoid triage routing. | Add explicit-comprehension question to F3: skill restates "is £8000 over 4 months £2000/mo or £8000 total?" before locking the value. Already in restate-and-confirm but make F3-specific. **auto-fix** applied to legal-intake-contract.md F3. |
| S-2 | security | hold-scope | M1 statute citations going stale between refreshes | Annual refresh cadence + skill stale-warning at +12 months + refusal at +24 months. Statute amendments mid-cycle (DUAA 2025 commenced 2026-02; potential 2026 Q4 amendment) miss the cycle. | DUAA 2026 amendment commences mid-cycle; SLO files cite stale text for ~12 months. | Add "monitor `legislation.gov.uk` RSS feed for amendments to cited Acts" as a deferred follow-up. **hold-scope** — annual cadence is acceptable; mid-cycle monitoring is a stretch goal. |
| S-3 | security | hold-scope | M2 conversational intake — adversarial founder | The conversational intake is designed for honest founders. An adversarial founder (researcher probing the skill) could craft answers to bypass gates. | Researcher answers F1=`uk-england-wales`, F4=`false`, F5=`false`, F2=`no/no`, F3=`£4500/total/precise` — all gates pass; obtains a draft. Probes the artifact for vulnerabilities. | This is acceptable behavior — the skill is producing a draft for a UK founder claiming honest inputs. The artifact carries `lawyer_review_recommended: true` and the LAWYER REVIEW header. Adversarial-research scenarios don't need additional gating. **hold-scope** — current behavior is correct. |
| S-4 | security | auto-fix | M1 / regulator enum source verification | Test asserts `statute_url` is `legislation.gov.uk` or `gov.uk` — but doesn't assert URL HEALTH (link doesn't 404). | Author captures URL at runbook-author time; URL gets reorganized later; future agents follow dead link. | Add a "URL health" check (lightweight: HEAD request) as a separate test that runs in CI but not blocking (URLs occasionally rate-limit). Document as `cargo test --features check-url-health`. **auto-fix** applied to M1 BDD as a optional check. |
| S-5 | security | ask | M3 / SAFE math verification | Numeric verification re-derives every cell; mismatch refuses. But what about pre-rounded cells (e.g., share count rounded to whole shares — re-derivation might produce 4999.99 vs the rounded 5000)? | Founder-friendly rounding produces small mismatches; verification refuses; founder loses confidence. | Specify rounding tolerance in the verification block (e.g., ±0.01% or 1 share). **ask** user — what tolerance? |

## Auto-fix corrections applied

- B-1: M1 BDD reinforces "if cannot verify within budget, mark pending-verification, do NOT weaken".
- B-2: M4 contract — Mom Test attribution-only, no long-form verbatim.
- B-4: M2 E2E test normalizes whitespace before byte-compare.
- B-7: M2 pre-flight enumerates cross-references; Notes flag known minor breakage of pre-existing GitHub issue comment URLs.
- S-1: F3 in legal-intake-contract.md gains explicit-comprehension question.
- S-4: M1 BDD adds optional URL-health check as `--features check-url-health`.

## Asks for project-owner decision

- **B-3**: add `--rapid-intake` flag for time-pressed founders?
- **B-5**: Python snippet license — MIT?
- **S-5**: SAFE math verification rounding tolerance — ±0.01%? ±1 share?

## Final disposition

**Accept with minor edits + asks**. M1 verification budget (B-1) is the largest scope risk; auto-fix locks the discipline.

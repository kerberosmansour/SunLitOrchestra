---
name: loops-and-lessons-closure
runbook: docs/slo/completed/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md
critiqued: 2026-04-28
personas_run: [ceo, eng, security]
design_persona_skipped: yes — no UI surface
---

# Critique — loops-and-lessons-closure

Four-persona adversarial review per [`/slo-critique`](../../skills/slo-critique/SKILL.md). Personas run sequentially; design skipped (no UI surface).

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| L-1 | ceo | reduce-scope | M4 / pre-flight | Carry-forward surface could become noise — every milestone start lists every open prior-retro issue. After 3 runbooks shipped, a session opens with "12 open issues, none urgent". | Founder runs `/slo-execute M5` of R3 third runbook in series; pre-flight surfaces 12 cross-runbook issues, founder eye-glazes and ignores all. The signal is lost in the volume. | Cap surface to N items per milestone; prefer items labeled `priority: high` OR `affects: <this-prefix>` if M3's marker scheme supports it. **ask** the user to pick a cap (lean: 3-5). |
| L-2 | ceo | hold-scope | M2 vs Issue #18 inventory | Issue #18 leaves loop inventory open: "fundraise loop? cofounder loop? hiring loop? legal-triage loop?" — M2 hard-codes 4 loops. | Project owner adds "fundraise loop" while M2 is in progress; M2 must absorb scope mid-flight or skip the addition. | Hold scope at 4 documented loops in M2; new loops are append-only follow-up issues post-M2. Document this scope rule explicitly. **auto-fix** applied to M2 Notes section. |
| L-3 | eng | ask | M3 spike step | Marker-choice spike (`gh search` reliability for title prefix vs label vs body sentinel) could produce a marker that GitHub deprecates 6 months later. | M3 picks `[retro]` title prefix; GitHub Issues changes search-tokenization in late 2026; `gh search "[retro]"` starts returning false positives; dedupe breaks. | Add a "spike result review at +6 months" follow-up in `references/biz/` cadence calendar. **defer** to runbook close. |
| L-4 | eng | auto-fix | M3 contract block | `LESSONS-BACKLOG.md` row schema is named in fallback flow but not specified. Future contributors will invent diverging row formats. | Contributor A files a row as `\| date \| classification \| body \|`; contributor B uses `\| classification \| skill \| body \|`. Cross-team reads break. | Specify the row schema in M3 `references/issue-filing-discipline.md`. **auto-fix** applied below. |
| L-5 | eng | ask | M4 carry-forward query | Query reads `gh issue list --label retro-derived --search 'in:title <prefix>'` (or equivalent). What if `gh` is rate-limited or auth's token has insufficient scope? | Founder running on a metered network; pre-flight times out after 30s waiting on `gh`; agent stalls or fails. | Add explicit timeout (5s) + fallback (skip carry-forward, log "gh unavailable; carry-forward skipped"). **ask** user for the timeout value. |
| S-1 | security | auto-fix | M3 BDD abuse cases | `gh search` dedupe — variant: malicious issue title with zero-width chars or homoglyphs evades dedupe. | Contributor X files 100 retro-derived issues with `[retroʿ]` (Hebrew letter substituted for `o`) — none match `[retro]` literal search; dedupe fails; rate-limit cap (40/hr) absorbs the storm but L-5's carry-forward query still surfaces them. | Add adversarial BDD row: "title with non-ASCII codepoints in marker → dedupe normalizes (NFKC) before comparison; refuse if normalization changes the marker." **auto-fix** applied to M3 BDD table. |
| S-2 | security | hold-scope | M3 + M4 confused-deputy via `.git/config` | Tampered `.git/config` redirecting origin remote could land filings in attacker repo. M3 BDD row addresses; M4 carry-forward read does NOT (currently no `--repo` flag is the existing defense). | Adversary modifies `.git/config` `remote.origin.url` to `https://github.com/attacker/foo.git`; `gh issue list` reads from that repo; carry-forward content is attacker-controlled. | Already mitigated by NO `--repo` flag inheriting `/slo-sast` M5 SEC-8. Confirm M4's `gh issue list` invocation also follows. **hold-scope** — defense documented; verify in M4 BDD. |
| S-3 | security | auto-fix | M4 abuse case `tm-loops-abuse-7` | Auto-extend allow-list attempt is currently labeled `tm-loops-abuse-7` in BDD but not enumerated in design overview's STRIDE sweep. | Drift between runbook and design overview confuses future agents reading both. | Add `tm-loops-abuse-7` to design overview's "New abuse cases" section. **auto-fix** applied to design overview. |
| S-4 | security | ask | All milestones — argv-list discipline | argv-list is mandated; structural-contract test asserts it's documented (grep). But the test does not assert no shell-string interpolation in actual `gh` invocation patterns *executed* by the skill at runtime. | Future SKILL.md revision adds "for convenience, just shell out to `bash -c \"gh issue create ...\"` — looks fine in prose; structural-contract test misses; runtime executes shell-string. | Add a runtime-fixture test (or stronger structural test scanning for shell-string patterns in SKILL.md fenced code blocks). **ask** user — high-effort vs high-value. |

## Auto-fix corrections applied

(See R1 runbook commits.)

- L-2: M2 Notes — added scope-hold rule about loop inventory.
- L-4: M3 — added `LESSONS-BACKLOG.md` row schema in `references/issue-filing-discipline.md` content.
- S-1: M3 BDD — added Unicode-normalization adversarial scenario.
- S-3: design overview — added `tm-loops-abuse-7` to "New abuse cases" enumeration.

## Asks for project-owner decision

- **L-1**: cap carry-forward surface to N items? (Lean: 3-5.)
- **L-3**: spike-result review at +6 months — accept defer to runbook close?
- **L-5**: `gh` timeout for carry-forward query (5s default)?
- **S-4**: stronger argv-list runtime fixture test — high-effort?

## Final disposition

**Accept with minor edits** — auto-fix items applied; ask items surfaced for user decision. No critical findings.

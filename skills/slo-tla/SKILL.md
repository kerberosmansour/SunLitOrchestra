---
name: slo-tla
description: >
  Use this skill when /slo-architect has set tla_required=true, or when the user
  asks to "verify the design", "model check", "prove this is correct", "add TLA+
  to this plan", or whenever a design involves concurrent actors, distributed
  state, ordering guarantees, resource ownership, or failure recovery. Produces
  a TLA+ spec, runs TLC, translates counterexamples to plain English, iterates
  with the user on fixes, and writes a verified-design doc with explicit model
  bounds. Skip for simple CRUD systems with no real concurrency risk.
---

# /slo-tla — verify the design with TLA+

You are a formal-methods engineer who has seen a lot of "this looks fine on a whiteboard" designs blow up in production. You translate designs into TLA+, run TLC, and do not let the word "verified" mean anything less than "TLC found no violations at stated bounds."

## Inputs

- A design doc, usually at `docs/design/<slug>-overview.md` from `/slo-architect`, or a hand-written design.
- Optionally, an existing spec at `specs/<name>.tla` to extend.

## Outputs

Four artifacts under `specs/` and `docs/design/`:

1. `specs/<name>.tla` — the TLA+ spec.
2. `specs/<name>.cfg` — TLC config (constants, invariants, temporal properties, bounds).
3. `specs/<name>.trace.md` — counterexamples TLC produced, translated to plain-English sequences.
4. `docs/design/<name>-verified.md` — the validated design writeup.

If a v3 runbook exists for this slug, patch its "High-Level Design for Formal Verification" section.

## Prereq cascade

Do each step in order. Do not skip. Do not "try it anyway."

### 1. JVM check

```bash
which java
```

If missing → print this and exit non-zero:

> Java is required for TLC. Install one of:
> - macOS:  `brew install openjdk`  (then follow the post-install hints for PATH)
> - Debian/Ubuntu:  `sudo apt install default-jre`
> - Other:  https://adoptium.net/
> Then re-run this skill.

Do not attempt to install Java for the user.

### 2. Jar check

Look for `tla2tools.jar` in this order:

1. `$TLA_TOOLS_JAR` environment variable.
2. `~/.sldo/tla/tla2tools.jar` (our managed cache).
3. `/usr/local/share/tla-tools/tla2tools.jar` (common system install).

If present: set `TLA_JAR` and continue to step 4.

### 3. Jar missing → download

Read pinned URL + SHA-256 from `skills/slo-tla/tools.toml` (sibling of this SKILL.md). Then:

```bash
mkdir -p ~/.sldo/tla
cd ~/.sldo/tla
curl -fL "<pinned_url>" -o tla2tools.jar.partial
echo "<pinned_sha256>  tla2tools.jar.partial" | shasum -a 256 -c -
```

If checksum FAILS:

```bash
rm -f tla2tools.jar.partial
```

Then exit non-zero with:
> SHA-256 mismatch on tla2tools.jar download. The upstream artifact may have been replaced or the network tampered with the response. Do not proceed. Report this to the skill maintainer.

If checksum passes:

```bash
mv tla2tools.jar.partial tla2tools.jar
echo "<pinned_version>" > ~/.sldo/tla/VERSION
cat > ~/.sldo/tla/tlc <<'EOF'
#!/usr/bin/env bash
exec java -Xmx4g -jar "$HOME/.sldo/tla/tla2tools.jar" "$@"
EOF
chmod +x ~/.sldo/tla/tlc
```

### 4. Apalache (lazy)

Do not check or install Apalache at skill start. Only check when TLC reports state explosion. In that case, print:
> TLC ran out of heap on this model. Consider Apalache for symbolic model checking.
> Install: https://github.com/apalache-mc/apalache/releases/latest
> Re-run this skill with `--use-apalache` once installed.

## Elicitation — the first real work

The design doc is almost never directly translatable to TLA+ — it over-specifies (timestamps, UUIDs, payloads) and under-specifies (actions as prose, not transitions). You reduce it.

Ask, in order:

### Q1. What property are we trying to prove?

Make the user name ONE safety property as a crisp sentence. If they name more than one, force ranking — start with the most load-bearing one.

### Q2. What's the smallest state that can violate it?

Example: mutual exclusion — two actors in critical section. No need to model timers, payloads, or tokens unless they're the mechanism that prevents the violation.

### Q3. Who are the actors, and how many?

Force a bound. "An unbounded number of workers" → start with 3. If the property holds at 3, think about whether it's a symmetry argument.

### Q4. What are the atomic actions?

List them. Each action is a TLA+ next-state relation. Merge the ones that share preconditions and effects. Usually 4–8 actions is the right range.

### Q5. What fails, and how?

Network drops? Process crashes? Duplicate delivery? For each failure mode, either model it as an action or explicitly exclude it from the bound.

### Q6. Liveness — what must eventually happen?

Only after safety holds. Every liveness property needs a fairness assumption (weak or strong, on which actions). Force this explicitly — liveness without fairness is a source of silent bugs.

## Spec drafting

Draft in stages. Do not try to write the whole spec in one go.

### Stage A — variables and Init

Declare the state variables. Write Init. Run TLC with Next == FALSE (no transitions) just to check Init is reachable. One state.

### Stage B — one action at a time

Add one action. Run TLC. Assert at least one invariant that's obviously true (TypeOK). Grow the state graph.

### Stage C — the invariant

Add the safety property from Q1 as an invariant. Run TLC. If it passes at a tiny bound, increase the bound. If it fails, translate the counterexample (see below) and fix the spec (or the design) before proceeding.

### Stage D — liveness

Only after safety is solid. Add the temporal property and fairness. Run TLC with `-deadlock` and the `PROPERTIES` line in .cfg.

### Stage E — bound and declare

Once TLC is green at your chosen bound, record the bound explicitly in the verified-design doc. N actors, M requests, K failures — whatever parameters the spec uses, state them.

## Counterexample translation — this is the product

Raw TLC output is a sequence of states with state variables. That is not a design finding. Translate:

1. Read the trace step by step. Name each actor's action ("A sends REQUEST", "B crashes", "A retries").
2. Identify the fork: the state in which the invariant first fails.
3. Write it as a short narrative: "Actor A sends REQUEST. Before B acknowledges, B crashes. A retries. B comes back up and processes twice."
4. Name the design assumption that broke: "We assumed at-most-once delivery, but the retry introduces at-least-once."
5. Propose the fix in the DESIGN, not the spec: "Add an idempotency key", "Require B to persist state before ack", etc.
6. Re-verify after the design fix lands in the spec.

The trace markdown file is shaped like:

```markdown
# Trace — <property name> violation

## Property
<crisp sentence>

## Counterexample
1. <Actor A: action>
2. <Actor B: action>
...
N. <the step at which the property fails>

## Fork point
<state N-1 → N: what changed, why it matters>

## Broken design assumption
<one sentence>

## Proposed fix
<one paragraph, design-level, not spec-level>

## Status
- [ ] design fix applied to spec
- [ ] TLC re-run, green at bound (N=…, M=…, K=…)
```

## Verified-design doc shape

```markdown
---
name: <slug>
verified_at: <YYYY-MM-DD>
tlc_bound: "N=3, M=5, K=2"
tool: "TLC 1.8.0"
---

# Verified Design — <title>

## System goal
<one paragraph>

## Abstract state
<variable list>

## Actions
<list>

## Safety properties checked
- <property> — PASS at <bound>

## Liveness properties checked (with fairness)
- <property> — PASS at <bound>, fairness: <weak|strong> on <actions>

## Simplifications from the real design
- <what was abstracted, why it still covers the real bug>

## Open questions
- <thing we did not model, and why it's acceptable>
```

## Gates — refuse to mark as verified when

- Bound is not stated.
- Fairness is not declared for any liveness property.
- Any invariant was weakened silently (e.g., "no two in CS" → "no two in CS in the same step" — that's a different, weaker property).
- A counterexample was suppressed rather than addressed.

## Anti-patterns

- Running TLC once, finding no violations, declaring victory — always iterate: add an action, re-run, grow the model.
- Translating counterexamples mechanically instead of narratively — "state 5: pc[A]=inCS, pc[B]=inCS" is not a finding; "A and B both got in because the lock check and acquire are not atomic" is.
- Using Apalache when TLC would work — Apalache is for state explosion, not default.
- Dropping the design simplifications paragraph — that's where future readers learn what the spec does NOT cover.

## Handoff

After `-verified.md` is written and TLC is green at the declared bound, suggest `/slo-plan` (if a runbook does not yet exist) or `/slo-critique` (if it does) so the plan reviewers can read the verification.

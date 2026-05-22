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

# /slo-tla — Verify The Design With TLA+

You are a formal-methods engineer. Translate designs into TLA+, run TLC, and do
not let "verified" mean anything less than "TLC found no violations at stated
bounds."

## Shared Discipline References

- Tool/version claims follow [`../../references/templates/citation-discipline.md`](../../references/templates/citation-discipline.md).
- JVM, jar, TLC, and Apalache subprocess checks follow [`../../references/templates/tool-safety-section.md`](../../references/templates/tool-safety-section.md).
- Downloaded verifier artifacts and checksums follow [`../../references/templates/version-pinning-discipline.md`](../../references/templates/version-pinning-discipline.md).

## Inputs

- A design doc, usually `docs/slo/design/<slug>-overview.md` from `/slo-architect`.
- Optionally, an existing spec at `specs/<name>.tla` to extend.

## Outputs

Write `specs/<name>.tla`, `specs/<name>.cfg`, `specs/<name>.trace.md`, and
`docs/slo/design/<name>-verified.md`. If a v3/v4 runbook exists for the slug,
patch its "High-Level Design for Formal Verification" section.

## Prereq Cascade

Do each step in order. Do not skip. Do not "try it anyway."

### 1. JVM Check

Run `which java`. If missing, print the Java requirement and exit non-zero with
install hints: macOS `brew install openjdk`, Debian/Ubuntu
`sudo apt install default-jre`, or https://adoptium.net/. Do not install Java for
the user.

### 2. Jar Check

Look for `tla2tools.jar` in `$TLA_TOOLS_JAR`, then
`~/.sldo/tla/tla2tools.jar`, then
`/usr/local/share/tla-tools/tla2tools.jar`. If present, set `TLA_JAR` and
continue to the Apalache-lazy step.

### 3. Jar Missing → Download

Read the pinned URL and SHA-256 from `skills/slo-tla/tools.toml`, then run:

```bash
mkdir -p ~/.sldo/tla
cd ~/.sldo/tla
curl -fL "<pinned_url>" -o tla2tools.jar.partial
echo "<pinned_sha256>  tla2tools.jar.partial" | shasum -a 256 -c -
```

If checksum FAILS, remove the partial file and exit with:

> SHA-256 mismatch on tla2tools.jar download. The upstream artifact may have
> been replaced or the network tampered with the response. Do not proceed.
> Report this to the skill maintainer.

If checksum passes, move the partial into `tla2tools.jar`, write the pinned
version to `~/.sldo/tla/VERSION`, and create `~/.sldo/tla/tlc` as a wrapper for
`java -Xmx4g -jar "$HOME/.sldo/tla/tla2tools.jar" "$@"`.

### 4. Apalache Lazy Check

Do not check or install Apalache at skill start. Only check when TLC reports
state explosion. Then recommend the pinned `tools.toml` Apalache entry and say:
`TLC ran out of heap on this model. Consider Apalache for symbolic model checking.`

### 5. Gitignore TLC Artifacts

Before the first TLC run, ensure `.gitignore` covers TLC scratch output without
removing tracked `.tla` or `.cfg` sources:

```gitignore
# TLA+ / TLC generated artifacts
**/*_TTrace_*.tla
**/*_TTrace_*.bin
**/states/
**/*-run.log
**/MC.out
**/MC.cfg
**/*.dot
**/*.toolbox/
```

Append only. Scope paths tightly when `states/` is meaningful elsewhere.

## Suitability gate — Is TLA+ The Right Tool?

Apply this before elicitation. TLA+ is a good fit only when at least two actors
or one actor plus a stateful environment can observably race, the race creates a
wrong outcome, correctness depends on interleavings, and the adversarial
scenario can be named in one sentence.

TLA+ is a poor fit when the property is single-function correctness, local
mutex behavior, CRUD/request-response validation, API-design semantics, UI
rendering, or probabilistic/ML accuracy. Recommend property-based tests,
ThreadSanitizer/loom/Jepsen, typed contracts, schema review, visual tests, or
empirical methods as appropriate.

If TLA+ is a poor fit, explain why, set `tla_required: false` in the design
overview, suggest `/slo-plan` next, and do not produce an empty spec for
ceremony. "TLA+ is not the right tool here" is a legitimate skill output.

## Method Dispatch

Load only the method file needed for the current phase:

| Phase | Read |
|---|---|
| Elicitation and spec drafting | [`references/methodology-elicitation.md`](references/methodology-elicitation.md) |
| Abstraction balance and state explosion | [`references/methodology-abstraction.md`](references/methodology-abstraction.md) |
| Counterexample translation | [`references/methodology-counterexample.md`](references/methodology-counterexample.md) |
| Verified-design writeup and gates | [`references/methodology-verified-design.md`](references/methodology-verified-design.md) |

## Common Gates And Anti-Patterns

Refuse to mark a design verified when: Bound is not stated (`N=3, M=5, K=2`);
fairness is missing for any liveness property; an invariant was weakened
silently; a counterexample was suppressed; the Naive / pre-fix variant passes
silently; TLC takes over about two minutes at the minimum-bug-exhibiting bound;
or the simplifications from the real design are hand-wavy.

Always run the Naive / pre-fix variant first and confirm it fails. Use Apalache
only for state explosion, not by default. Do not add `-workers auto` before
cutting the model down. Do not hand a raw trace back without a design-level fix
proposal.

## Handoff

After `-verified.md` is written and TLC is green at the declared bound, suggest
`/slo-plan` (or `/slo-critique` if a runbook exists); for a Rust target with
bounded kernels also suggest `/slo-kani` (refinement: TLA+ action → Rust fn →
Kani harness; TLA+ owns interleavings, Kani is out of scope for concurrency). If the suitability gate short-circuited, suggest `/slo-plan` with the alternative approach as a milestone.

---

**Loops**: Sprint loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).

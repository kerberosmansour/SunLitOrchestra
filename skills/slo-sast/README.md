# /slo-sast — find bugs by *reading the code*, driven by your threat model

> **Status:** active · **License:** [Apache-2.0 OR MIT](../../LICENSE) · **Host:** any agent that loads `/slo-*` skills · **Engine:** Semgrep (public registry + your custom pack)

**This page is for humans.** The machine-facing contract is [`SKILL.md`](SKILL.md); the
deep how-to is in [`references/`](references/). Start here.

## What is this, in one breath?

**SAST** = *Static Application Security Testing*: a tool **reads your source code** and
flags dangerous patterns — *without running the app*. Think spell-check, but for security
bugs.

`/slo-sast` is the skill that wires SAST in **the smart way**: it reads your project's
threat model, works out which bug classes actually matter for *this* app, and points
Semgrep (the scanner) at exactly those — plus a custom rule pack that catches the things
the off-the-shelf rules miss.

## Why does this exist? (the problem, plainly)

Out-of-the-box code-scanning rules are mediocre in a *predictable* way. On a deliberately-
vulnerable practice app, the stock rules caught only about **a third** of the real bugs and
made noise on harmless files. Two boring reasons:

1. **Rules look for the danger and the user input *on the same line*** — e.g.
   `eval(req.body.x)`. Real code never looks like that. It pulls the input out at the top
   of a function and uses it ten lines (or another file) later. The rule walks right past it.
2. **Rules don't know what your files are.** They flag example links in tutorial HTML and
   passwords in test fixtures as if they were live bugs. Noise drowns the signal.

`/slo-sast` fixes both: it drives a **threat-model-aware** rule selection, and the custom
pack is written in a shape that *follows the data* and *ignores docs/fixtures*. On the same
practice app this roughly **doubled** the real-bug catch rate (≈33% → ≈63%) while *adding
one* false alarm — and the same custom pack, unchanged, then found 8 exact known bugs in a
completely different app it was never written for. Generic, not bespoke.

## What it actually does

1. Reads `docs/slo/design/<slug>-threat-model.md` and pulls out the bug classes (CWEs) that
   matter for this app.
2. Detects the stack (language/framework) from the manifest files.
3. Selects the matching tuned Semgrep rules, and (where the registry is thin) applies a
   **generic custom rule shape**: *follow the tainted data, including through the
   `const {x} = req.body` unpacking every app does* — that one trick is what flips the
   misses into hits.
4. Emits a safe CI workflow + a baselined config + an audit-defense manifest, and
   re-derives the ruleset when the threat model changes.

The rule-writing recipe (taint + per-language input propagator, split SQL vs NoSQL so the
CWE label is right, ignore docs/fixtures, honest "no SCA / intra-file only" caveats) is in
[`references/custom-rule-shape.md`](references/custom-rule-shape.md).

## Quick start

```text
/slo-sast <slug>
```

It needs a threat model to exist (`docs/slo/design/<slug>-threat-model.md`) — that's the
input that makes the scan *targeted* instead of generic. No threat model → it tells you to
create one rather than guessing.

## Honest limits (we'd rather say it)

- **No dependency scanning.** Vulnerable third-party packages (a whole bug class) need a
  separate SCA tool — Semgrep can't see them. The skill says so instead of implying coverage.
- **Same-file data flow.** The open-source taint engine follows data within a file. A bug
  whose input and sink are in different files can still slip through; the skill flags this
  as a known gap, not a clean bill of health.

## Pairs with `/slo-dast-tuner`

SAST finds *what and where in the code*. Its sibling [`/slo-dast-tuner`](../slo-dast-tuner/README.md)
takes that output, reads the code to work out *which URL* it maps to, logs in, and **proves
it at runtime**. Together they're far stronger than either alone — that handoff is the
**SAST→DAST bridge**.

## Where to go next

| You want… | Read |
|---|---|
| The exact agent contract | [`SKILL.md`](SKILL.md) |
| The custom-rule recipe (all languages) | [`references/custom-rule-shape.md`](references/custom-rule-shape.md) |
| How rule selection works | [`references/methodology-m2-stack-detect.md`](references/methodology-m2-stack-detect.md) |
| The shared vuln vocabulary | [`../../references/security/vuln-class-taxonomy.md`](../../references/security/vuln-class-taxonomy.md) |
| The runtime sibling | [`../slo-dast-tuner/README.md`](../slo-dast-tuner/README.md) |

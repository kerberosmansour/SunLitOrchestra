# Graphify Provider Evidence Loop

This reference turns Graphify from a "nice graph" into a SLO evidence loop for
large product repos. It is intentionally broader than security: the product
value is faster codebase understanding, faster troubleshooting, and better
security triage from the same graph.

## Product Promise

For an engineer, the graph should answer: "Where should I look first, and why?"
For an AI coding agent, the graph should answer: "What can I rely on before
opening half the repo?"

The useful output is not the graph itself. The useful output is a ranked,
source-grounded map of code paths, findings, likely root causes, and tests to
add.

## Provider Model

| Provider kind | Typical source | Adds | Use |
|---|---|---|---|
| `syntax` | Graphify extraction, AST extractors | Files, imports, calls, routes, modules | Fast navigation and architecture view |
| `semantic` | rust-analyzer, TypeScript language service | symbol definitions, references, command bindings, typed call edges | Cross-file and cross-language confidence |
| `analysis` | OpenGrep or Semgrep-compatible rules | findings, CWE tags, severity, source ranges | Security, reliability, conventions, QA risks |

Every fact must carry provider name, provider kind, confidence, source file,
source range, run id, and timestamp. If the fact came from a stub, the final
decision must say so and must not claim real semantic-provider proof.

## Private-Repo Handling

Raw outputs belong under ignored experiment paths only. Acceptable committed
outputs are:

- counts by provider and finding class;
- anonymized issue buckets;
- generic path shapes such as "frontend route to Rust command";
- severity/disposition totals;
- lessons learned and next-action recommendations.

Do not commit raw file paths, issue titles, code snippets, customer words,
absolute paths, repo names, or generated Graphify HTML/JSON from a private repo.

## GitHub Issues Lens

Read issues before scanning when possible. The graph is most useful when the
queries mirror real work waiting for an engineer.

| Issue signal | Graph query shape | Value |
|---|---|---|
| "Where is this behavior implemented?" | route/component/module to handlers and tests | Knowledge acceleration |
| "This breaks sometimes" | worker/retry/error path to state mutations and logging | Troubleshooting acceleration |
| "Authz or validation gap" | public entry point to missing guard, parser, or persistence edge | Security and QA |
| "Refactor is scary" | dependency fan-in/fan-out and changed-file blast radius | Safer planning |
| "Agent keeps opening the wrong files" | issue keywords to graph neighborhoods | Fewer false starts |

## Security and QA Query Set

Run these as graph queries or equivalent filtered reports:

| Question | Why |
|---|---|
| Which findings cross a trust boundary? | Separates noisy static hits from product risk |
| Which findings are on user-triggered routes or commands? | Finds bugs a user can actually trigger |
| Which findings map to existing GitHub Issues? | Converts scanner output into planned work |
| Which high-risk flows lack regression tests? | Turns security into test selection |
| Which retries, panics, unwraps, or swallowed errors touch user paths? | Surfaces normal QA bugs waiting to happen |
| Which files are central to many issues and findings? | Helps engineers plan small safe changes |

## Custom Rust Rule Packs

SunLit Orchestra ships a Semgrep-compatible Rust rule pack under `.semgrep/rust`.
The rules are intentionally review-oriented, not "every hit is a bug." Useful
classes for Graphify evidence loops include:

- `cwe-755-panic-on-result-fn`: panic or unwrap inside `Result`-returning Rust
  functions, useful for panic-DoS and QA crash risk review.
- `cwe-190-integer-overflow-in-security-context`: capacity and length arithmetic
  in security-sensitive contexts.
- `cwe-20-improper-input-validation`: dynamic regex, path construction, and file
  reads from insufficiently validated input.
- Unsafe/FFI classes such as CWE-416, CWE-125, and CWE-787 when the target has
  low-level Rust code.

When using OpenGrep, treat these as analysis-provider facts and preserve the
rule id, CWE, severity, message, range, and source provider.

## Calibration Signal From An Anonymized Trial

A bounded private-repo trial showed enough value to move to private beta:

| Signal | Result |
|---|---:|
| TypeScript language-service facts | 7,021 |
| rust-analyzer facts | 34,368 |
| Total semantic facts | 41,389 |
| Graph import size | 6,685 nodes / 20,872 edges |
| Analysis findings | 35 warnings, 0 critical/high |
| Issue-derived signals | 109 security / 131 troubleshooting / 123 knowledge |
| Custom rule ids reused | 15 |

The important learning was not "security scanner finds bugs." It was that a
single graph could connect issue themes, semantic code paths, static findings,
and next tests. That helps both a human engineer and an AI coding agent avoid
wandering through unrelated files.

The bounded scan did not prove a critical security issue. It did surface warning
classes worth review: panic/error handling, integer arithmetic in sensitive
contexts, and input validation. Those are also normal QA risks because they can
become crashes, bad retries, bad error handling, or malformed input bugs.

## Readout Contract

A good readout is short and decision-oriented:

| Section | Must include |
|---|---|
| Capability | What Graphify + providers can do now |
| Evidence | Counts, pass/fail gates, provider reality, and limitations |
| Use-case value | Security, knowledge, troubleshooting, QA risk |
| Findings | Critical/high count, warning classes, false-positive posture |
| Next actions | Fix now, file issue, add test, human review, accepted risk |

Do not paste raw scanner output. Do not paste Graphify reports from private
repos. Do not mention private repo names.

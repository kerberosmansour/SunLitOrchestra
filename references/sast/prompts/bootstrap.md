# Bootstrap-mode prompt — `/slo-rulegen` (no `--extend`)

> Read by `/slo-rulegen` when invoked WITHOUT the `--extend` flag.
> Generates the top-10 Rust + Semgrep rule pack from `references/sast/cwe-map-rust.md`.

You are running `/slo-rulegen` in **bootstrap mode**. The user has run `/slo-rulegen` in a Rust workspace (or in this SLO repo) and you are populating `.semgrep/rust/` with the top-10 CWE rule pack.

## Your job

Author 10 rule pairs at `.semgrep/rust/<rule-id>.{yaml,rs}`, one per CWE in `references/sast/cwe-map-rust.md`. Each pair gates through `cargo xtask sast-verify gate <rule.yaml>` before being written to disk.

## The contract — read these before authoring

1. `references/sast/cwe-map-rust.md` — the top-10 ranking, one CWE per row.
2. `references/sast/manifest-schema.md` — the YAML metadata schema. ALL 10 rules MUST conform.
3. `references/sast/AUTHORING.md` — the Trail of Bits AGPL clean-room rule + style guide. Re-author each rule from scratch from the variation template, NEVER copying YAML text from the AGPL precedent.
4. `references/sast/variations/cwe-<NNN>.md` — one per CWE. Each declares `minimum_pattern_either_arms` (the floor `check-coverage` enforces) and `sink_shapes` (the named patterns the rule's `pattern-either` arms MUST cover).
5. `references/sast/semgrep-rust-syntax.md` — which Semgrep primitives work for Rust in 2026. Avoid the disallowed primitives (esp. `metavariable-type` on generics, untested `pattern-inside: unsafe { ... }`).

## Procedure

For each CWE in the map (process in the order listed in `cwe-map-rust.md` — CWE-755 first, CWE-79 last):

1. **Read the variation template** at `references/sast/variations/cwe-<NNN>.md`. Note the `minimum_pattern_either_arms` value and the `sink_shapes` list.
2. **Author a rule YAML** at `.semgrep/rust/cwe-<NNN>-<short-name>.yaml`. Conform to the manifest schema. The `pattern-either` arms cover EVERY `sink_shape` (one arm per shape minimum).
3. **Author a paired fixture** at `.semgrep/rust/cwe-<NNN>-<short-name>.rs`. Include one `// ruleid: <rule-id>` annotation per `pattern-either` arm covering the bad shape, and at least one `// ok: <rule-id>` annotation for a known-clean shape.
4. **Run the gate**: `cargo xtask sast-verify gate .semgrep/rust/cwe-<NNN>-<short-name>.yaml`.
5. **If gate exits 0**, the pair lands. Move to the next CWE.
6. **If gate exits non-zero**, do NOT write anything beyond what's already on disk; tell the user which sub-step failed (validate, test, check-coverage, check-clean) and the structured error. Course-correct: tighten patterns, fix fixture annotations, OR re-author the rule from scratch if the issue is structural.

## Gates — never bypass

- **NEVER** invoke `validate`, `test`, `check-coverage`, or `check-clean` directly. Always use `gate`. Bypassing `gate` is a P1 finding for `/slo-critique` security persona.
- **NEVER** write a rule file unless `gate` exits 0. The gate IS the contract.
- **NEVER** modify `references/sast/` to make a rule pass — if the rule fails `check-coverage` because the variation template's minimum-N is "too high," that's a sign the rule's variation enumeration is incomplete; fix the rule, not the template.
- **NEVER** copy YAML text from `trailofbits/semgrep-rules` (AGPL). Re-author from the variation template.

## Idempotency — existing pack collision

If `.semgrep/rust/<rule-id>.yaml` already exists when you would write it:

- DO NOT overwrite silently.
- Prompt the user: `overwrite | skip | rename-with-suffix`.
- Default on missing input: prompt again.
- Same idempotency contract as `/slo-architect` re-running SECURITY.md.

## Tools you MUST NOT use

This skill's toolflag denial (per [SECURITY.md](../../../SECURITY.md) "SAST rule-gen skill pack — additional rules") forbids `WebFetch` and `WebSearch`. The CWE map and variation templates are pre-baked in `references/sast/`; rule generation does not need network access. The denial is the primary control against prompt-injection-via-bug-summary (threat-model row `tm-sast-rulegen-skill-pack-abuse-1`).

If you find yourself wanting to fetch external content (e.g., look up a Semgrep example rule online), STOP. The information you need is in `references/sast/`. If it's not, file a content gap as a follow-up — do not bypass the denial.

## Outputs

- 10 rule YAMLs at `.semgrep/rust/<rule-id>.yaml`
- 10 paired fixtures at `.semgrep/rust/<rule-id>.rs`
- Each pair `gate`-clean
- A summary printed to stdout listing all 10 rule ids, their CWE, and `gate` exit codes (all 0)

## Out of scope

- `/slo-rulegen --extend` (M2 mode). This bootstrap prompt is for the empty-pack seeding only.
- Editing existing rules in `.semgrep/rust/`. If a rule needs revision, the user invokes `/slo-rulegen` with explicit `--rule-id <id> --action revise` (not implemented in M1).
- Authoring rules for non-Rust languages. Runbook B handles TypeScript.

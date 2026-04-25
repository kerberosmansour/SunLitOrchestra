# Lessons — biz-judgment-runtime M1

## What worked

- **`tests/common/mod.rs` shared module pattern.** Cargo treats `tests/common/` (with `mod.rs`) as an internal module rather than as an integration test, so `mod common;` in each per-milestone test file pulls in shared helpers without compiling the helper file as its own test crate. No new boilerplate per test file beyond the `mod common;` line.
- **Stdlib-only frontmatter parsing.** Fixture frontmatter is flat key:value, so a 30-line parser without `serde_yaml` covered every shape. Same pattern as the M5 PII-scan parser; consistency over abstraction.
- **Path-traversal guard via `canonicalize` + `starts_with`.** Rejects fixture paths outside `references/biz/judgment-fixtures/` before any file read. The matching unit test passes a `/tmp` path and confirms the guard fires — small, clean defense for the BDD abuse-case row.
- **Env-flag gate on top of `#[ignore]`.** Two layers: `#[ignore]` keeps it out of `cargo test`'s default run; `BIZ_JUDGMENT_RUNTIME_LIVE=1` keeps it from running on `cargo test -- --ignored` either, so a developer who runs `--ignored` to debug something else doesn't accidentally bill API calls. Skip message names the env var so the layering is discoverable.

## What I'd do differently

- **The `_timeout: Duration` parameter on `invoke_claude` is currently unused.** I left it in the signature so M2 can add `wait_timeout`-style polling without breaking M1 callers. If M2 ends up not needing it (because `--max-budget-usd` self-bounds runtime), drop the parameter rather than leaving a `_`-prefix carve-out.
- **HOME redirection only sandboxes filesystem state, not credentials.** The harness still inherits the user's Anthropic auth from the parent env (`ANTHROPIC_API_KEY` / OAuth keychain via `--bare`'s `apiKeyHelper` route). That's correct — we *want* the user's billing to apply — but if a future fixture needs to test "what happens with no auth", we'd add an explicit env-clear step.

## Pitfalls / things to remember

- **`--bare` mode** disables CLAUDE.md auto-discovery. The tempdir's symlinked `CLAUDE.md` is still useful (claude reads it via `--add-dir`), but skill-discovery happens through `<add-dir>/.claude/skills/`, NOT through any global skill installation. This is intentional: the tempdir's skill set is the *canonical* set the harness exercises; nothing on the user's `~/.claude/skills/` can leak in.
- **The artifact-discovery contract is single-artifact-per-invocation.** If a skill writes an artifact AND a separate "explanation memo", the harness errors loudly rather than silently picking one. The skills currently shipped don't do that; if a future skill does, the runbook author needs to either change the skill or extend the harness's contract — silent first-wins is the worst outcome.

## Citations to combined critique

- B1+B2+C f5 — IR35-pressure capitulation: the M1 happy-path proves the infrastructure; M2 wires `tax-efficiency-pushback.md` (the adversarial fixture explicitly cited in f5) into the same pipeline.
- Runbook A f6 — LLM judgment residual: the same infrastructure handles all 9 v1 fixtures across the 4 advisor skills.

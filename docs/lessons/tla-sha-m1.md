# Lessons Learned — tla-sha Milestone 1

## What changed
- New crate `crates/sldo-tla-sha` with a binary that reads `skills/slo-tla/tools.toml`, fetches `UNSET` entries, stream-hashes with a size cap, redirects-allowed-hosts check, prints a TOML patch.
- Workspace `Cargo.toml` updated to include the new member.
- Critique findings f1 (redirect_to_foreign_host_aborts) and f2 (oversize_response_aborts) accepted and implemented.
- 10 library unit tests + 7 binary E2E tests.

## Design decisions and why
- **Binary never writes `tools.toml`.** Prints a patch; humans commit. Rationale: self-modifying skill config is weird and risks silently accepting a bad hash.
- **Stream-hash with a 500 MB cap.** Oversize-response defense (f2). The cap is ~50× the current TLC/Apalache release sizes. Exceeding it means upstream changed drastically or the response is tampered; either way it warrants maintainer attention.
- **Final-URL host allow-list, not initial-URL.** Redirects are followed (up to 10), but the FINAL URL's host is checked against the allow-list (f1). Prevents a compromised GitHub-release redirect from silently sending us to a foreign CDN.
- **Allow-list is exact-match, not substring-match.** "`github.com.attacker.tld`" and "`fake-github.com`" are rejected. A substring-based check would be vulnerable to both.
- **`lib.rs` split from `main.rs`.** Library surface (`hash_reader`, `is_host_allowed`, `ToolsToml`, `format_patch`) is unit-testable without network. Network-requiring logic stays in `fetch_and_hash`, which is covered by its callers' integration tests (not unit tests). Matches the runbook's "no network calls in unit tests" constraint.
- **`hash_reader<R: Read>` as the core abstraction.** Takes any `Read`, works against `Cursor<Vec<u8>>` in tests and a `reqwest::Response` in prod. The cap check is in the reader, not the HTTP client, so oversize protection is provider-independent.

## Mistakes made
- First version used `reqwest` blocking default features, which pulled in `native-tls` and bloated the dep tree. Fixed by specifying `default-features = false, features = ["blocking", "rustls-tls"]` to match the rest of the ecosystem convention.
- First cut of `is_host_allowed` used `host.ends_with(allowed)` — vulnerable to lookalike hosts like `github.com.attacker.tld`. Fixed to exact lowercase equality. Caught by my own test (`allowed_hosts_rejects_foreign_hosts`).

## Root causes
- Defaulting to permissive substring matches is a recurring bug pattern. Going forward, any allow-list check should default to exact-match unless there's a specific reason to loosen.

## What was harder than expected
- Keeping the response streaming contract clean. Options: (a) load all bytes into memory, (b) stream in chunks with manual size tracking, (c) hash lazy with reqwest's chunked API. Chose (b) with an 8 KB internal buffer — balances memory, CPU, and simplicity.

## Naming conventions established
- `tools.toml` section name == skill-facing artifact name (`tlc`, `apalache`). The binary iterates sections by name.
- Patch output is a comment-annotated diff, not a machine-applied change. Lines start with `# ` so users can't accidentally eval the output.

## Test patterns that worked well
- Three SHA-256 known-answer tests (empty string, "hello", cap violation) anchor the hasher against the standard. Can't accidentally drift.
- Allow-list rejection tests include the two lookalike attacks (subdomain suffix, prefix) explicitly. That's the discipline fix for the substring-match bug.

## Missing tests that should exist now
- A test that actually fetches from a real GitHub release URL (network-dependent; gated behind `#[ignore]` or env var). Would catch API breakage on our side.
- A test that exercises the 10-redirect limit. Low priority — if this fires, something's already wrong upstream.

## Rules for the next milestone (M2 — --verify)
- Reuse `fetch_and_hash` as-is. M2 adds a comparison path only; no new fetch logic.
- Verify output should be per-section PASS/FAIL, exit 0 only on all-PASS. Already stubbed in `run_verify`; confirm by M2 BDD.
- `--verify` against UNSET must fail early (before network). Already implemented; confirm by M2 test `verify_refuses_when_any_unset` (already green).

## Template improvements suggested
- None; the runbook template worked well for this size of feature.

# Critique Summary — tla-sha-autopop

Run date: 2026-04-24. Runbook reviewed: [RUNBOOK-TLA-SHA-AUTOPOP.md](../RUNBOOK-TLA-SHA-AUTOPOP.md).

## Overall disposition

Plan is small, tight, and executable. No hold-scope or reduce-scope findings. Two `ask` findings and one `auto-fix` landed. Design persona N/A (no UI). Security and eng had most of the findings, as expected for a network-fetching helper.

## Findings

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|----|---------|----------|-----------------|---------|-------------------|----------------|
| f1 | eng | ask | M1 BDD scenarios | Missing scenario: what if the URL redirects to an unrelated host? | Upstream release URL at `github.com/tlaplus/tlaplus/...` 301-redirects to a Netlify mirror that serves a different artifact (hypothetical). Helper follows, computes SHA of wrong file, prints a "correct" patch that is actually wrong. | Add BDD scenario `redirect_to_foreign_host_aborts` — helper must reject redirects that leave the original hostname's allow-list (github.com, objects.githubusercontent.com). |
| f2 | security | ask | M1 Contract Block | No maximum response size — a malicious 10GB response from a compromised CDN would exhaust memory. | Attacker compromises the release-assets CDN (outside normal threat model but worth hardening). Serves 10GB of `/dev/urandom`. Helper reads the whole stream into memory, OOMs the box. | Stream the response through the SHA hasher with a hard size cap (say 500MB for TLC, 1GB for Apalache) and abort on exceed. Add BDD scenario `oversize_response_aborts`. |
| f3 | eng | auto-fix | M1 Files Allowed To Change | `Cargo.toml` listed without specifying which one (workspace root vs. new crate's). | — | Applied: rewrote the row to `Cargo.toml (workspace root, new member)` plus a separate row for `crates/sldo-tla-sha/Cargo.toml`. |
| f4 | CEO | hold-scope | — | Should this be extended to cover the `.sldo/tla/` cache verification too? | — | No — the cache is regenerable; only `tools.toml` is the committed contract. Hold scope. |
| f5 | design | N/A | — | No UI surface. Design persona skipped. | — | — |

## Auto-fixes applied

- f3 — Files Allowed To Change row clarified (see runbook M1 Contract Block).

## Asks awaiting user decision

- f1, f2 — both request adding a BDD scenario to M1. Recommend accepting both; they address real failure modes. If the user accepts, M1's contract widens slightly (allow-list unchanged since scenarios are test-only).

## Hold-scope

- f4 — confirmed hold.

## Next action

Once the user accepts or declines f1 and f2, the runbook is ready for `/slo-execute M1`.

# CWE susceptibility map for Rust (2026)

Top-10 CWE classes that idiomatic safe-Rust + popular ecosystem crates are most susceptible to. Ranked from a 2-hop RustSec → GHSA → OSV `cwe_ids` join (research dossier provenance) plus user-pain-anchored web-class CWEs.

The list is **hybrid-ranked**:

- **Frequency-confirmed** (research dossier sample, n=8 from 2024–2026 RustSec advisories): CWE-416, CWE-697, CWE-125, CWE-787, CWE-190, CWE-295, CWE-672, CWE-20.
- **Threat-class-reasoned** (no major Rust framework advisory but user-pain-anchored): CWE-79, CWE-755.

## Ranking

| Rank | CWE | Title | Provenance | Why Rust is susceptible |
|---|---|---|---|---|
| 1 | CWE-755 | Improper Handling of Exceptional Conditions (panic-DoS class) | threat-class-reasoned; user-pain-anchored. Tagged per Trail of Bits' production rule `panic-in-function-returning-result.yaml`. RustSec/GHSA do NOT natively tag panic-DoS. NOT CWE-248. | Rust's "safe" claim is "no memory corruption / no RCE", not "no crash". `.unwrap()`, `.expect()`, `panic!()`, `?`-without-handler, slice indexing, integer overflow in debug mode, recursive deserialization all panic on attacker input. By default panics unwind; with `panic = 'abort'` they kill the process. For a server, single panic kills request, thread, or process depending on runtime + boundaries. |
| 2 | CWE-416 | Use After Free | frequency-confirmed: 3 hits in research sample (cassandra-rs RUSTSEC-2024-0017, mio RUSTSEC-2024-0019, wasmtime::Linker::clone RUSTSEC-2026-0090) | Surfaces in `unsafe { }` / FFI / raw-pointer code where the borrow checker is bypassed. Common in C-binding crates and low-level concurrency primitives. |
| 3 | CWE-697 | Incorrect Comparison | frequency-confirmed: 2 hits in research sample (idna RUSTSEC-2024-0421, hpke-rs) | Punycode confusion, IDN homograph, constant-time-comparison gaps in cryptography crates, signature verification that compares lengths but not bytes. Often Net-new vs. C-style memory bugs. |
| 4 | CWE-125 | Out-of-bounds Read | frequency-confirmed: 2 hits in research sample (ruzstd RUSTSEC-2024-0400, scaly RUSTSEC-2026-0080) | Decompression / parser code reading attacker-controlled length fields without bounds-checking. Often in `unsafe` slice access. |
| 5 | CWE-787 | Out-of-bounds Write | frequency-confirmed: top in RustXec dataset (9 hits Jan 2021–Apr 2025) | Same surface as CWE-125 but the write side. `unsafe` slice assignment, `ptr::copy_nonoverlapping` with attacker-controlled length, `Vec::set_len` past capacity. |
| 6 | CWE-190 | Integer Overflow or Wraparound | frequency-confirmed: 1 direct hit + many derivative DoS bugs | Release-mode wrapping arithmetic (debug panics; release wraps by default unless overflow-checks enabled). Affects capacity calculations (`Vec::with_capacity(a + b)`), length math (`buf.len() - offset`), authorization counters, expiry timestamps. CWE-191 (integer underflow) often co-located. |
| 7 | CWE-295 | Improper Certificate Validation | frequency-confirmed: present in research sample | TLS configuration mistakes — `danger_accept_invalid_certs(true)`, custom verifiers that accept any cert, missing hostname verification. Existing semgrep-rules covers some of this; we extend with reqwest / rustls-specific patterns. |
| 8 | CWE-672 | Operation on a Resource after Expiration or Release | frequency-confirmed: present in research sample | Closing a file handle then writing; using a session token after expiry; access via a destroyed Drop guard. Rust's lifetimes catch many of these but FFI / `unsafe` / Drop-impl bugs reintroduce them. |
| 9 | CWE-20 | Improper Input Validation | frequency-confirmed: present in research sample (broad class) | Handler accepts input, doesn't validate, downstream code crashes or misbehaves. Very common in axum / actix-web handlers using `Json<T>` without `deny_unknown_fields`. The user pain anchor for "missing input sanitisation". |
| 10 | CWE-79 | Cross-site Scripting (XSS) | threat-class-reasoned; user-pain-anchored. NOT frequency-confirmed in 2024–2025 RustSec corpus (no major framework-level XSS advisory surfaced for axum / actix). | User explicitly named this in idea-doc Q2: "missing input sanitisation or sanitise outbound response (e.g. for an XSS issue)". Rust-rendered HTML (askama, tera, maud) can be misused; embedding `{{ user_data | safe }}` filters or hand-rolled HTML construction without context-aware encoding is the sink shape. Frequency in advisory corpus is low because most Rust web apps either use templating engines that escape by default OR don't render HTML. But user pain is real. |

## Explicitly NOT in the v1 top-10

- **CWE-89** (SQL Injection) — no major Rust framework SQLi advisory surfaced 2024–2025. `sqlx` parameterized queries are the dominant pattern. Users add per-bug via extend mode if they hand-roll SQL.
- **CWE-918** (SSRF) — no major Rust framework SSRF advisory. `reqwest::get(user_input)` without validation is the sink shape; users add per-bug.
- **CWE-352** (CSRF) — framework-mediated; tower-http has middleware. Not a code-pattern that a Semgrep rule catches well.
- **CWE-22** (Path Traversal) — `std::path::Path` does not natively prevent traversal but the Rust ecosystem typically wraps file IO in safer types. Per-bug in extend mode.
- **CWE-862 / CWE-863** (Missing / Incorrect Authorization) — not a syntactic pattern; needs human review or framework-specific authz middleware checks. Not a fit for Semgrep static patterns.

## Provenance two-hop join

Per the research dossier's "CWE susceptibility ranking for Rust in 2026" section:

1. RustSec advisory-db's TOML schema has NO `cwe` field. Categories (`code-execution`, `denial-of-service`, `memory-corruption`, etc.) are coarser than CWE.
2. Each RustSec advisory's `aliases[GHSA-*]` lists a GHSA identifier when one exists.
3. The corresponding GitHub Security Advisory (GHSA) record on OSV.dev DOES populate `database_specific.cwe_ids`.
4. So the join is `RustSec.aliases[GHSA-*]` → `OSV.GHSA.database_specific.cwe_ids`.

Sample (n=8) from 2024–2026 vuln-class advisories returned 8/8 GHSA-side CWE coverage. ~95% join hit-rate expected for `informational: vulnerability` advisories; the residual is GHSA-less informational/unmaintained-crate advisories.

## How variations are derived

For each CWE in this map, `references/sast/variations/cwe-<NNN>.md` declares:

- A `minimum_pattern_either_arms` count (the floor `check-coverage` enforces).
- A `sink_shapes` list (the named patterns each `pattern-either` arm must cover; `cwe_<NNN>_rule_covers_documented_variation_shapes` BDD asserts content coverage).
- Representative bad-snippet + good-snippet pairs that the rule's paired `<rule-id>.rs` fixture should reflect.

When `cargo xtask sast-verify gate` runs, `check-coverage` reads `metadata.cwe` from the rule, looks up the variation file, and asserts the rule's `pattern-either` arm count is within `[minimum_pattern_either_arms, 25]`.

## Updating this map

- Adding a CWE: add a row to the table above + author `references/sast/variations/cwe-<NNN>.md` + cite at least one RustSec / GHSA URL or document why threat-class reasoning was used.
- Removing a CWE: requires re-running `/slo-architect` (changes the M1 milestone scope).
- Re-ranking: routine; update the table.

## Citations

- RustSec advisory-db: https://github.com/rustsec/advisory-db
- OSV.dev: https://osv.dev/
- 2025 CWE Top 25: https://cwe.mitre.org/top25/archive/2025/2025_cwe_top25.html
- RustXec dataset (Virginia Tech 2026): https://people.cs.vt.edu/xinw/publications/RustXec26-B38KjKAe.pdf
- Trail of Bits CWE-755 precedent: https://github.com/trailofbits/semgrep-rules/blob/main/rs/panic-in-function-returning-result.yaml

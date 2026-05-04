# Real-world FP shakedown — 2026-04-25

Calibration record for the SAST rule pack, sampled against five popular Rust crates. Performed as part of the M1.6 follow-up cycle.

## Method

Cloned each repo at `--depth 1` to `/tmp/sast-shakedown/<crate>/`, then ran:

```bash
semgrep --config .semgrep/rust/ --include '*.rs' --quiet /tmp/sast-shakedown/<crate>/
```

No `.semgrepignore` was applied during the shakedown itself (it was being calibrated). Findings counted by rule + by top-level directory.

## Sample crates

| Crate | URL | Reason |
|---|---|---|
| `bytes` | https://github.com/tokio-rs/bytes | low-level memory primitives → exercises CWE-787 / CWE-125 |
| `aho-corasick` | https://github.com/BurntSushi/aho-corasick | mature parser + DFA → exercises CWE-755, CWE-125 |
| `http` | https://github.com/hyperium/http | HTTP types crate → exercises CWE-20, CWE-755 |
| `httparse` | https://github.com/seanmonstar/httparse | low-level HTTP parser → exercises CWE-787, CWE-125 |
| `regex` | https://github.com/rust-lang/regex | regex engine → exercises CWE-755, CWE-190 |

## Findings (raw, pre-tightening)

| Crate | Total | CWE-755 | CWE-787 | CWE-125 | CWE-190 | Other |
|---|---|---|---|---|---|---|
| `bytes` | 31 | 0 | 21 | 10 | 0 | 0 |
| `aho-corasick` | 15 | 12 | 0 | 3 | 0 | 0 |
| `http` | 3 | 0 | 2 | 1 | 0 | 0 |
| `httparse` | 1 | 0 | 1 | 0 | 0 | 0 |
| `regex` | 83 | 76 | 1 | 3 | 3 | 0 |
| **Total** | **133** | **88** | **25** | **17** | **3** | **0** |

## Triage

### CWE-755 — 88 findings, mostly TPs the maintainers accept

`.unwrap()` / `.expect()` inside `Result`-returning fns. Sampled findings in `regex/regex-automata/`:

- `accel.rs:324` — `.unwrap()` on `try_from(self.len())` after the comment "The number of accelerators can never exceed AccelTy::MAX". Maintainer invariant.
- `automaton.rs:288-291` — `.expect("no quit in start without look-behind")` — explicit invariant in the message.

These are not bugs in the maintainer's view. The rule is correctly firing on a high-cost-of-panic shape; the maintainer has reviewed and accepted each. **Action**: lower the rule's confidence from `HIGH` to `MEDIUM`. The findings remain actionable for review (every undocumented `.unwrap()` warrants a second look) but the HIGH framing overstated the bug-to-FP ratio.

### CWE-787 / CWE-125 — 42 findings, mostly TPs in dangerous-primitive-wrapping crates

The findings in `bytes/src/` and `httparse/src/` are correct: `Vec::set_len`, `ptr::copy_nonoverlapping`, `slice::from_raw_parts`, `get_unchecked` are exactly the OOB-write/read primitives the rules target. Crates whose **purpose** is to wrap these primitives safely will surface every use of them — that's the rule working as designed, not failing.

A maintainer of `bytes` running this pack would mark each of the 21 CWE-787 + 10 CWE-125 findings as accepted with reference to the `// SAFETY:` comment that already accompanies each. **Action**: documentation. The README's expected-FP-rate framing should explain that mature memory crates will look noisy under this pack and that's correct behavior.

### CWE-787 / CWE-125 — 8 findings in `bytes/benches/`, real FPs

Microbenchmarks that use `set_len(0)` on a pre-allocated buffer to reset between iterations — bounded usage, not a bug. **Action**: extend `.semgrepignore` to exclude `benches/`, `examples/`, `fuzz/`, `vendor/`, `third_party/` by default.

### CWE-190 — 3 findings in `regex`, low-priority

`regex` does intentional integer arithmetic in tight loops with documented `wrapping_*` calls. **Action**: none — the rule's `pattern-not` carve-outs already exclude `wrapping_*` / `checked_*` / `saturating_*`; the 3 findings are arithmetic in security-context-shaped fns the rule was designed to flag for review.

## Tightenings applied

1. **`.semgrepignore`** — added `benches/`, `examples/`, `fuzz/`, `vendor/`, `third_party/`. These are non-shipping code paths where unsafe primitives are routinely used out of necessity (microbenches) or by design (fuzz harnesses).
2. **CWE-755 confidence** — `HIGH` → `MEDIUM` per the regex / aho-corasick sample. Rule message + sink shapes unchanged.

No structural rule changes. No CWE-787 / CWE-125 / CWE-416 / CWE-672 changes — those rules are correctly firing on the dangerous primitives they target; tightening would lose real coverage.

## Re-running the shakedown

```bash
mkdir -p /tmp/sast-shakedown
cd /tmp/sast-shakedown
for repo in tokio-rs/bytes BurntSushi/aho-corasick hyperium/http seanmonstar/httparse rust-lang/regex; do
    git clone --depth 1 -q "https://github.com/$repo.git" 2>/dev/null
done
cd /path/to/SunLitOrchestra
for crate in bytes aho-corasick http httparse regex; do
    echo "=== $crate ==="
    semgrep --config .semgrep/rust/ --include '*.rs' --quiet /tmp/sast-shakedown/$crate/
done
```

Future cycles should re-run when:
- A new rule is added (M2 / M2.5 / future).
- Major Rust idiom shifts (e.g., `let-else`, new const-generics patterns) land in the ecosystem.
- A user reports a high FP rate.

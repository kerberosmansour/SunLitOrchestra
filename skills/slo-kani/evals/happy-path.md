---
skill: slo-kani
case: happy-path
category: happy-path
expected_behavior: Verify a small bounded Rust kernel and report scope honestly.
risk: high
---

## Input
~~~text
/slo-kani on a crate with an unsafe `read_byte(&[u8], idx)` wrapper.
~~~

## Expected Behavior
Run the prereq cascade, score the unsafe wrapper high, write a `#[cfg(kani)]`
harness, run `cargo kani`, and write a scope report stating the bound and
assumptions. The pre-fix variant fails first.

## Must Not
- Report green without a scope block.
- Claim anything beyond the stated harness/bounds.

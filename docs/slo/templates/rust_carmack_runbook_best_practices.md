# Rust Best Practices for AI Coding Agents — Carmack-Style Reliability Runbook Companion

> **Purpose**: Provide Rust-specific execution rules for AI coding agents implementing a planned runbook.  
> **Use with**: `carmack_ai_first_runbook_template.md` or an existing AI-first runbook.  
> **Core rule**: Rust’s compiler is not the finish line. The agent must combine Rust’s type system, Clippy, tests, assertions, debugger inspection, bounded resources, CI evidence, and dependency auditing.

---

## 1. Rust Project Metadata To Fill In

| Field | Value |
|---|---|
| Workspace root | `[path]` |
| Rust toolchain | `[stable/nightly/version]` |
| Crate names | `[crate list]` |
| Binary targets | `[targets]` |
| Library targets | `[targets]` |
| Feature flags | `[features]` |
| MSRV | `[minimum supported Rust version]` |
| Async runtime | `[tokio/async-std/smol/none]` |
| Serialization format | `[serde/json/bincode/etc.]` |
| Persistence layer | `[db/files/none]` |
| FFI/unsafe present? | `[yes/no]` |
| Debugger setup | `[CodeLLDB/rust-lldb/rust-gdb/CLion/etc.]` |

---

## 2. Default Rust Commands

Use these as the default runbook commands unless the project has more precise equivalents.

```bash
# Formatting
cargo fmt --all --check

# Fast compile/type check
cargo check --workspace --all-targets --all-features

# Static analysis
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Tests
cargo test --workspace --all-features

# Documentation build for public API drift
cargo doc --workspace --all-features --no-deps

# Dependency/security checks, when installed
cargo audit
cargo deny check

# Unsafe/UB-oriented validation, when nightly and Miri-compatible
cargo +nightly miri test
```

For feature-sensitive crates, add matrix checks:

```bash
cargo check --workspace --all-targets --no-default-features
cargo test --workspace --no-default-features
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
```

---

## 3. Agent Pre-Flight Protocol for Rust Milestones

Before editing Rust code, the agent must:

1. Read `Cargo.toml`, workspace configuration, feature flags, and relevant crate manifests.
2. Read `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `.cargo/config.toml`, and CI config if present.
3. Run or record why it could not run the baseline commands.
4. Identify whether the milestone touches:
   - public Rust APIs
   - serialized structs/enums
   - FFI boundaries
   - async/concurrency code
   - persistent state
   - feature flags
   - `unsafe`
   - dependency graph
5. Write BDD/integration tests before production code.
6. Add runtime validation tests where behavior crosses process, IPC, network, file, or persistence boundaries.
7. Re-state allowed files, forbidden changes, compatibility obligations, invariants, and required commands.

---

## 4. Debugger-First Rust Workflow

Do not rely only on `println!`, `dbg!`, or reading the code.

Recommended tools:

- VS Code + `rust-analyzer` + CodeLLDB
- CLion / IntelliJ Rust
- `rust-lldb`
- `rust-gdb`
- `RUST_BACKTRACE=1` for stack traces

Rules:

- `dbg!` is allowed during investigation but must not remain in production code.
- `println!`/`eprintln!` must not be used as permanent observability in libraries or backend services.
- Use `tracing` or the project’s existing logging framework for production diagnostics.
- If a test failure depends on state evolution, step through the test or inspect state with a debugger before broad rewrites.
- If an async/concurrency bug is suspected, inspect task boundaries, lock lifetimes, and cancellation paths.

Milestone Evidence Log additions:

| Step | Command / Tool | Expected | Actual | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Debugger available | `[tool]` | can debug tests or binary | | | |
| Failure inspected | `[test/binary]` | state inspected before fix | | | |

---

## 5. Clippy and Lint Policy

### 5.1 Baseline Policy

Every Rust milestone should pass:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

For projects with a large existing lint backlog, create an explicit baseline exception list and do not add new warnings.

### 5.2 Recommended Workspace Lints

Prefer configuring lints in `Cargo.toml` when supported by the project’s toolchain. Otherwise use crate-level attributes in `lib.rs`/`main.rs`.

```toml
# Cargo.toml
[workspace.lints.rust]
unsafe_code = "forbid"
unused_must_use = "deny"
missing_debug_implementations = "warn"

[workspace.lints.clippy]
correctness = "deny"
suspicious = "deny"
perf = "warn"
complexity = "warn"
pedantic = "warn"
dbg_macro = "deny"
todo = "deny"
unimplemented = "deny"
print_stdout = "warn"
print_stderr = "warn"
unwrap_used = "warn"
expect_used = "warn"
panic = "warn"
await_holding_lock = "deny"
```

Crate-level fallback:

```rust
#![forbid(unsafe_code)]
#![deny(unused_must_use)]
#![deny(clippy::correctness)]
#![deny(clippy::suspicious)]
#![warn(clippy::perf)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![deny(clippy::await_holding_lock)]
```

### 5.3 Waiver Rules

If the agent suppresses a lint:

- use the narrowest possible scope
- explain why in a nearby comment or Evidence Log
- do not globally suppress a lint to pass CI
- prefer fixing the design over silencing the tool

Example:

```rust
// This mirrors the externally specified protocol field name.
#[allow(clippy::struct_field_names)]
struct ProtocolMessage {
    message_id: MessageId,
}
```

---

## 6. Assertions and Invariants in Rust

### 6.1 Which Mechanism To Use

| Mechanism | Use For | Notes |
|---|---|---|
| `assert!` | invariant that must hold in all builds | active in release |
| `debug_assert!` | development-only internal invariant | normally removed from optimized release builds |
| `Result<T, E>` | expected recoverable failure | public boundaries, IO, user input |
| `Option<T>` | legitimate absence | do not use as silent failure |
| `panic!` | unrecoverable programmer error | avoid in libraries except strong invariant violation |
| `unreachable!` | state proven impossible | add tests/proof; prefer type modeling first |
| `unsafe { unreachable_unchecked() }` | almost never | forbidden unless formally reviewed unsafe block |

### 6.2 Examples

```rust
fn average_non_empty(values: &[f64]) -> Result<f64, AverageError> {
    if values.is_empty() {
        return Err(AverageError::EmptyInput);
    }

    debug_assert!(values.iter().all(|v| v.is_finite()));

    let sum: f64 = values.iter().sum();
    let average = sum / values.len() as f64;

    assert!(average.is_finite(), "average must remain finite after validation");
    Ok(average)
}
```

### 6.3 Runbook Requirements

Every milestone touching non-trivial logic must list:

- internal invariants
- public input validation rules
- expected recoverable errors
- assertions added or intentionally not added
- tests proving the invariants or errors

---

## 7. Type-Driven Guardrails

Rust agents should make invalid states unrepresentable before adding procedural checks.

### 7.1 Newtypes for Domain IDs

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(u64);

pub fn load_project(project_id: ProjectId) -> Result<Project, LoadError> {
    // Cannot accidentally pass UserId here.
    todo!()
}
```

### 7.2 Constrained Constructors

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyName(String);

impl NonEmptyName {
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(NameError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### 7.3 Enums for State Machines

```rust
pub enum JobState {
    Queued,
    Running { started_at: Timestamp },
    Succeeded { output: OutputId },
    Failed { error: JobError },
    Cancelled,
}
```

Avoid loose state strings such as `"running"`, `"done"`, or `"error"` unless crossing a serialization boundary. At boundaries, convert into typed enums immediately.

### 7.4 Use Standard Constrained Types

Prefer standard constrained types where applicable:

```rust
use std::num::{NonZeroUsize, NonZeroU64};

pub struct PageSize(NonZeroUsize);
pub struct PositiveRetryLimit(NonZeroU64);
```

---

## 8. Bounded Resources in Rust

### 8.1 Vectors and Collections

Use `Vec` when growth is genuinely data-dependent and acceptable. Otherwise prefer fixed or bounded structures.

Options:

| Need | Preferred Rust Pattern |
|---|---|
| fixed exact size | `[T; N]` |
| grow up to hard limit without heap growth | `arrayvec::ArrayVec<T, N>` if dependency allowed |
| embedded/no-alloc bounded vector | `heapless::Vec<T, N>` if dependency allowed |
| normal dynamic vector with expected size | `Vec::with_capacity(N)` plus invariant tests/assertions |
| bounded queue/channel | bounded channel API, semaphore, or explicit rejection |

Example with no new dependency:

```rust
const MAX_VISIBLE_ITEMS: usize = 256;

fn collect_visible_items(items: impl Iterator<Item = Item>) -> Result<Vec<Item>, CollectError> {
    let mut visible = Vec::with_capacity(MAX_VISIBLE_ITEMS);

    for item in items {
        if visible.len() == MAX_VISIBLE_ITEMS {
            return Err(CollectError::TooManyVisibleItems { max: MAX_VISIBLE_ITEMS });
        }
        visible.push(item);
    }

    debug_assert!(visible.len() <= MAX_VISIBLE_ITEMS);
    Ok(visible)
}
```

### 8.2 Async and Queues

For async Rust:

- prefer bounded `mpsc` channels over unbounded channels
- define behavior when a channel is full
- avoid spawning unbounded tasks
- use semaphores for concurrency caps
- use timeouts for external calls
- propagate cancellation safely

Milestone contract must list:

| Resource | Limit | Enforcement | At-Limit Behavior | Test |
|---|---:|---|---|---|
| `[channel/tasks/cache]` | `[N]` | `[bounded channel/semaphore/etc.]` | `[backpressure/error/drop]` | `[test]` |

---

## 9. Error Handling Rules

### 9.1 Public Boundaries Return Errors

At public API, command, IPC, FFI, network, and file boundaries, use structured error handling.

Good:

```rust
pub fn parse_config(input: &str) -> Result<Config, ConfigError> {
    // ...
    Ok(config)
}
```

Avoid:

```rust
pub fn parse_config(input: &str) -> Config {
    serde_json::from_str(input).unwrap()
}
```

### 9.2 `unwrap` and `expect`

Default rule:

- no `unwrap()` in production code
- no `expect()` in production code unless the message proves why the invariant is guaranteed
- tests may use `unwrap()`/`expect()` when failure would make the test invalid, but assertion messages should remain clear

Acceptable production exception:

```rust
let nonzero = NonZeroUsize::new(1).expect("literal 1 is non-zero");
```

### 9.3 Error Visibility

Every error must be visible through:

- returned error type
- user-visible state
- structured log/tracing event
- metric/event, if the project has telemetry

Do not convert errors to defaults silently.

---

## 10. Testing Strategy for Rust Runbooks

### 10.1 Test Layers

| Layer | Location | Purpose |
|---|---|---|
| Unit tests | `#[cfg(test)] mod tests` near code | local invariants and edge cases |
| Integration/BDD tests | `tests/<prefix>_<feature>.rs` | public crate behavior |
| Scenario tests | `tests/scenarios/...` | multi-step workflows |
| Runtime/E2E tests | project-specific | real binary/API/IPC/persistence behavior |
| Doc tests | rustdoc examples | public API examples remain true |

### 10.2 Required Categories

Each milestone should cover applicable cases:

- happy path
- invalid input
- empty input/state
- too-large input/resource limit
- dependency failure
- partial failure and cleanup
- persistence round-trip
- backward compatibility
- concurrency/race behavior

### 10.3 Test Artifact Cleanup

Use temp directories and RAII cleanup.

```rust
#[test]
fn writes_output_to_tempdir() {
    let temp = tempfile::tempdir().expect("tempdir should be created");
    let output_path = temp.path().join("output.json");

    write_output(&output_path).expect("output should be written");

    assert!(output_path.exists());
} // temp directory is removed here
```

A new dependency such as `tempfile` must still follow the runbook dependency policy unless already present.

---

## 11. Runtime Validation and Smoke Tests

Rust agents must validate behavior beyond compilation.

Examples:

```bash
# CLI smoke test
cargo run --bin my_app -- --help

# Server boot smoke test
cargo run --bin my_server

# Specific integration test
cargo test --test my_feature_bdd
```

Runtime tests should prove:

- binary starts
- config loads
- changed command/API works
- invalid input fails visibly
- state is persisted/restored if applicable
- no panic occurs in expected failure paths

---

## 12. Unsafe Rust Policy

### 12.1 Default Policy

Use:

```rust
#![forbid(unsafe_code)]
```

unless the crate explicitly requires unsafe.

### 12.2 If Unsafe Is Required

When unsafe is unavoidable:

```rust
#![deny(unsafe_op_in_unsafe_fn)]
```

Rules:

- isolate unsafe in the smallest module possible
- expose a safe API around it
- write `SAFETY:` comments for every unsafe block
- document invariants that callers and callees must uphold
- add tests for boundary cases
- run Miri where applicable
- never use unsafe to bypass borrow checker friction without a written proof of correctness

Example:

```rust
pub fn first_byte(bytes: &[u8]) -> Option<u8> {
    if bytes.is_empty() {
        return None;
    }

    // SAFETY: We checked that the slice is not empty, so index 0 is in bounds.
    Some(unsafe { *bytes.get_unchecked(0) })
}
```

Prefer the safe equivalent unless profiling proves the unsafe version matters.

---

## 13. Async and Concurrency Rules

For async/concurrent Rust milestones:

- do not hold a synchronous mutex guard across `.await`
- prefer scoped lock lifetimes
- prefer message passing or ownership transfer where possible
- bound task spawning
- use cancellation-safe APIs
- add tests for cancellation, timeout, and partial failure
- use `loom` for critical concurrent data structures if the dependency is allowed
- model high-risk protocols in the runbook’s formal/state-machine section

Bad pattern:

```rust
async fn bad(state: std::sync::Mutex<State>) {
    let mut guard = state.lock().unwrap();
    external_call().await;
    guard.update();
}
```

Better pattern:

```rust
async fn better(state: &std::sync::Mutex<State>) -> Result<(), Error> {
    let input = {
        let guard = state.lock().map_err(|_| Error::StatePoisoned)?;
        guard.snapshot_for_call()
    };

    let output = external_call(input).await?;

    let mut guard = state.lock().map_err(|_| Error::StatePoisoned)?;
    guard.apply(output);
    Ok(())
}
```

---

## 14. Serialization, Schema, and Persistence

When a milestone touches serialized data:

- do not rename fields casually
- define compatibility with older persisted data
- add round-trip tests
- add fixture-based tests for old versions
- avoid accepting unknown or malformed data silently unless the schema requires forward compatibility
- document defaulting behavior

Serde example for strict config input:

```rust
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub endpoint: String,
    pub retry_limit: u8,
}
```

For forward-compatible public data formats, do not use `deny_unknown_fields` blindly; document the policy.

---

## 15. Dependency and Supply-Chain Policy

### 15.1 Adding a Crate

A new crate is allowed only when the milestone contract lists:

- crate name
- version or version policy
- features enabled/disabled
- why `std` or existing crates are insufficient
- maintenance/security rationale
- license compatibility
- build/runtime cost
- tests covering integration

### 15.2 Required Checks

```bash
cargo tree
cargo tree -d
cargo audit
cargo deny check
```

If a dependency is added, record:

| Check | Result |
|---|---|
| `cargo tree` reviewed | `[yes/no]` |
| duplicate versions checked | `[yes/no]` |
| advisory audit | `[pass/fail/exception]` |
| license/source policy | `[pass/fail/exception]` |

---

## 16. Documentation Rules

Rust milestones must update docs when behavior changes.

Potential docs:

- `README.md`
- `ARCHITECTURE.md`
- crate-level docs in `lib.rs`
- public item docs
- examples
- migration notes
- feature flag docs

For public APIs, prefer examples that compile as doc tests when feasible.

---

## 17. Rust Milestone Contract Additions

Add these fields to every Rust milestone contract:

| Field | Value |
|---|---|
| Crates touched | `[crate names]` |
| Targets touched | `[lib/bin/test/example]` |
| Feature flags affected | `[features or none]` |
| Public Rust APIs changed | `[yes/no + list]` |
| Serialized types changed | `[yes/no + list]` |
| Unsafe touched | `[yes/no]` |
| Async/concurrency touched | `[yes/no]` |
| New resource bounds | `[bounds]` |
| Error types changed | `[yes/no + compatibility]` |
| Clippy/lint expectations | `[commands]` |
| Miri required | `[yes/no + reason]` |
| Dependency audit required | `[yes/no + reason]` |

---

## 18. Rust Evidence Log Additions

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Format | `cargo fmt --all --check` | clean | | | |
| Check | `cargo check --workspace --all-targets --all-features` | clean | | | |
| Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | clean | | | |
| Tests | `cargo test --workspace --all-features` | green | | | |
| No-default-features check | `cargo check --workspace --all-targets --no-default-features` | green or N/A | | | |
| Docs | `cargo doc --workspace --all-features --no-deps` | builds | | | |
| Audit | `cargo audit` | no unapproved advisories | | | |
| Deny | `cargo deny check` | policy pass | | | |
| Miri | `cargo +nightly miri test` | pass or N/A | | | |
| Artifact cleanup | `git status` | no test artifacts | | | |

---

## 19. Rust Self-Review Gate

Before marking a Rust milestone done, answer:

- Did `cargo fmt --all --check` pass?
- Did `cargo clippy --workspace --all-targets --all-features -- -D warnings` pass?
- Did the full relevant test suite pass?
- Did I avoid `unwrap`, `expect`, and `panic` in production paths unless justified?
- Did I avoid adding `unsafe`? If not, is it isolated, documented, tested, and Miri-checked where possible?
- Did I preserve public APIs and serialized formats unless explicitly changed?
- Did I add typed domain models instead of passing raw primitives where constraints matter?
- Did I add assertions for internal invariants?
- Did I bound collections, queues, retries, caches, or spawned tasks introduced by the change?
- Did I avoid holding locks across `.await`?
- Did all tests clean up temp files?
- Did dependency checks pass if dependencies changed?
- Did docs and examples match the implementation?

If any answer is “no,” the milestone is not done.

---

## 20. Carmack Principle to Rust Enforcement Map

| Carmack-Style Principle | Rust Enforcement |
|---|---|
| Do not debug by guessing | debugger, `RUST_BACKTRACE=1`, test debugging, state inspection |
| Static analysis humbles everyone | compiler, Clippy, rustfmt, `cargo check`, `cargo doc` |
| Assertions are executable comments | `assert!`, `debug_assert!`, invariant tests |
| Avoid silent unbounded growth | fixed arrays, bounded channels, `Vec::with_capacity`, explicit max checks |
| Guardrails over intentions | newtypes, enums, constrained constructors, feature gates, lints |
| Evidence over claims | Evidence Log with exact commands and results |
| Avoid ego-driven broad rewrites | allowed-files list, refactor budget, compatibility checklist |
| Runtime behavior matters | integration tests, E2E tests, smoke tests, boot checks |
| Unsafe assumptions must be proven | `#![forbid(unsafe_code)]`, `unsafe_op_in_unsafe_fn`, `SAFETY:` comments, Miri |

---

## 21. Recommended CI Skeleton

```yaml
name: Rust CI

on:
  pull_request:
  push:
    branches: [main]

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Format
        run: cargo fmt --all --check
      - name: Check
        run: cargo check --workspace --all-targets --all-features
      - name: Clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      - name: Test
        run: cargo test --workspace --all-features
      - name: Docs
        run: cargo doc --workspace --all-features --no-deps
```

Add `cargo audit`, `cargo deny`, Miri, database services, browser/runtime E2E, or feature-matrix jobs according to the milestone contract.

---

## 22. Final Agent Prompt for Rust Milestones

Use this before implementation:

> Restate the Rust milestone goal, crates touched, allowed files, forbidden changes, public APIs and serialized types that must remain compatible, feature flags, unsafe policy, async/concurrency risks, required BDD tests, required runtime validations, new resource bounds, assertions/invariants, Clippy/lint gates, dependency checks, and exact Definition of Done. Then propose the smallest Rust implementation that satisfies the contract without broad refactoring.

---

## 23. Source Basis

This document specializes the language-independent Carmack-style AI-first runbook for Rust. It assumes the project follows a milestone contract with scope control, test-first development, runtime validation, evidence logs, artifact cleanup, compatibility checks, dependency/migration policy, lessons files, and completion summaries.

---
cwe: "CWE-20"
title: "Improper Input Validation"
minimum_pattern_either_arms: 4
sink_shapes:
  - serde_json_without_deny_unknown_fields
  - axum_handler_takes_string_no_length_cap
  - regex_compile_with_user_input_dos
  - path_join_user_input_no_canonicalize
provenance: "Idea-doc user pain anchor: missing input sanitization across web app surfaces"
---

# Variation template — CWE-20 Improper Input Validation

## Why this class

Broad class. The user pain anchor for "missing input sanitisation". Rust's type system helps but doesn't eliminate: a `String` parameter doesn't bound length, an `&[u8]` doesn't bound deserialization depth, a `serde_json::Value` is unstructured.

## Sink shapes (4 minimum)

### 1. `serde_json_without_deny_unknown_fields`

```rust
#[derive(serde::Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}
```

Without `#[serde(deny_unknown_fields)]`, an attacker can sneak extra fields into the JSON body. Mass-assignment risk if `name` / `email` aren't the full schema.

### 2. `axum_handler_takes_string_no_length_cap`

```rust
async fn comment_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    body: String,
) -> impl axum::response::IntoResponse {
    save_comment(state, body).await   // BAD: no length cap on body
}
```

Should use `axum::body::Bytes` with a `RequestBodyLimitLayer` OR `axum::Json<T>` where T has bounded fields.

### 3. `regex_compile_with_user_input_dos`

```rust
fn matches(pattern: &str, target: &str) -> bool {
    let re = regex::Regex::new(pattern).unwrap();   // BAD: attacker controls pattern → catastrophic backtracking → DoS
    re.is_match(target)
}
```

`regex` crate is linear-time so doesn't backtrack catastrophically, but other regex engines (rust-regex with PCRE feature) do. The bigger DoS shape is compile cost on giant patterns.

### 4. `path_join_user_input_no_canonicalize`

```rust
fn read(file: &str) -> std::io::Result<Vec<u8>> {
    let p = std::path::PathBuf::from(format!("./uploads/{file}"));
    std::fs::read(p)   // BAD: file = "../../etc/passwd" escapes
}
```

## Good (silent) shapes for the fixture

```rust
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateUserSafe { name: String, email: String }

async fn comment_handler_safe(
    body: axum::body::Bytes,   // bounded by RequestBodyLimitLayer
) -> impl axum::response::IntoResponse { /* ... */ }

fn read_safe(file: &str) -> std::io::Result<Vec<u8>> {
    if file.contains('/') || file.contains("..") {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "bad name"));
    }
    let p = std::path::PathBuf::from("./uploads").join(file);
    let canon = std::fs::canonicalize(&p)?;
    let prefix = std::fs::canonicalize("./uploads")?;
    if !canon.starts_with(prefix) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "escape"));
    }
    std::fs::read(canon)
}
```

## Composition note

This rule is broad. Confidence MEDIUM. Pair with `cargo audit` for known-CVE input-validation crates, and Clippy `clippy::missing_panics_doc` for surfaces that may panic on input.

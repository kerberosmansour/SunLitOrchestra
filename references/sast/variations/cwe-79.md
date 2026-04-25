---
cwe: "CWE-79"
title: "Cross-site Scripting (XSS)"
minimum_pattern_either_arms: 3
sink_shapes:
  - askama_safe_filter_on_user_data
  - format_html_with_curly_user_data
  - response_html_unwrapped_user_data
provenance: "Idea-doc user pain anchor: missing outbound output encoding for XSS issue"
---

# Variation template — CWE-79 Cross-site Scripting (Rust web rendering)

## Why this class

Rust webapps that render HTML need context-aware encoding. Templating engines (askama, tera, maud) escape by default, but the `safe` filter / `Raw` wrapper opts out. Hand-rolled HTML formatting via `format!` / `write!` interpolates user data unescaped.

NOT frequency-confirmed in the 2024–2025 RustSec corpus (no major framework-level XSS advisory surfaced for axum/actix), but the user explicitly named XSS as a recurring pain. Threat-class-reasoned inclusion.

## Sink shapes (3 minimum)

### 1. `askama_safe_filter_on_user_data`

```rust
#[derive(askama::Template)]
#[template(source = "<p>Hi {{ name | safe }}</p>", ext = "html")]   // BAD: `safe` filter on user data bypasses auto-escape
struct Hi { name: String }
```

Equivalent in tera (`{{ name | safe }}`), maud (`(PreEscaped(name))`), or askama's `{{ name | escape("none") }}`.

### 2. `format_html_with_curly_user_data`

```rust
async fn render(name: String) -> axum::response::Html<String> {
    axum::response::Html(format!("<h1>Welcome, {name}!</h1>"))   // BAD: `name` not encoded
}
```

The `format!` shape interpolates unescaped. Even via `write!` to a `Vec<u8>` body, same risk.

### 3. `response_html_unwrapped_user_data`

```rust
async fn show(req: axum::extract::Query<MyQuery>) -> axum::response::Html<String> {
    let q = req.0.search;
    axum::response::Html(format!("<div>You searched for: {q}</div>"))   // BAD: q unencoded
}
```

## Good (silent) shapes for the fixture

```rust
async fn render_safe(name: String) -> axum::response::Html<String> {
    let escaped = html_escape::encode_text(&name);
    axum::response::Html(format!("<h1>Welcome, {escaped}!</h1>"))
}

#[derive(askama::Template)]
#[template(source = "<p>Hi {{ name }}</p>", ext = "html")]   // default escape
struct HiSafe { name: String }
```

## Composition note

`html_escape` and `askama` (without `safe` filter) are the canonical safe paths. Rule confidence MEDIUM — the false-positive rate is non-trivial because format-string-with-user-data is also legitimately used outside HTML rendering. The rule should scope `pattern-inside fn $F(...) -> axum::response::Html<...> { ... }` or similar response-type marker.

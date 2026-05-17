# Rust Boundary Surface

```rust
use secure_boundary::render_untrusted_markdown_literal;

pub fn render_issue_body_for_agent_prompt(issue_body: &str) -> Result<String, secure_boundary::PromptBoundaryError> {
    render_untrusted_markdown_literal(issue_body)
}
```

Surface map:

| Touched surface | Secure-construction default | Expected tests |
|---|---|---|
| Untrusted Markdown entering an agent prompt | `secure_boundary::render_untrusted_markdown_literal` | unit tests for fence breakout, bidi controls, and size limits |
| Variant-analysis evidence | `security_core::variant_analysis` | schema serialization and required-field tests |

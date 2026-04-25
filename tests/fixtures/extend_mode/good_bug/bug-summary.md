# Bug summary — clean fixture for /slo-rulegen --extend

A handler in `src/api/users.rs:42` panicked when the request body was empty:

```
thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Error("EOF while parsing a value", line: 1, column: 0)', src/api/users.rs:42:34
```

The handler was:

```rust
async fn create_user(body: String) -> Result<axum::Json<User>, AppError> {
    let user: User = serde_json::from_str(&body).unwrap();   // BAD
    Ok(axum::Json(user))
}
```

The fix changed `.unwrap()` to `.map_err(|e| AppError::BadRequest(e.to_string()))?`. The class is panic-on-Result-fn (CWE-755).

We want extend-mode to produce 3-5 variation rules covering credible variants:
- `.unwrap()` (this exact case)
- `.expect("...")`
- explicit `panic!()` in the Err arm of a match
- `?` propagation into a context that doesn't handle the Err

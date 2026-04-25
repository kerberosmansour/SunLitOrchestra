// Paired fixture for cwe-755-panic-on-result-fn.
// Per Semgrep upstream convention: same basename as the rule, language ext .rs,
// `// ruleid:` annotates the line that should fire, `// ok:` annotates the line
// that should not.

use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Debug, thiserror::Error)]
enum HandlerError {
    #[error("bad request: {0}")]
    BadRequest(String),
}

// --- BAD shapes (rule MUST fire) ---

fn parse_user_unwrap(body: &str) -> Result<User, HandlerError> {
    // ruleid: cwe-755-panic-on-result-fn
    let user: User = serde_json::from_str(body).unwrap();
    Ok(user)
}

fn parse_user_expect(body: &str) -> Result<User, HandlerError> {
    // ruleid: cwe-755-panic-on-result-fn
    let user: User = serde_json::from_str(body).expect("body must be valid JSON");
    Ok(user)
}

fn parse_user_explicit_panic(body: &str) -> Result<User, HandlerError> {
    let parsed: Result<User, _> = serde_json::from_str(body);
    match parsed {
        Ok(u) => Ok(u),
        Err(_) => {
            // ruleid: cwe-755-panic-on-result-fn
            panic!("invalid JSON");
        }
    }
}

async fn parse_user_async(body: &str) -> Result<User, HandlerError> {
    // ruleid: cwe-755-panic-on-result-fn
    let user: User = serde_json::from_str(body).unwrap();
    Ok(user)
}

// --- OK shapes (rule MUST NOT fire) ---

fn parse_user_safe(body: &str) -> Result<User, HandlerError> {
    // ok: cwe-755-panic-on-result-fn
    let user: User =
        serde_json::from_str(body).map_err(|e| HandlerError::BadRequest(e.to_string()))?;
    Ok(user)
}

fn parse_user_default(body: &str) -> User {
    // unwrap_or_default in a non-Result-returning fn — ok
    serde_json::from_str(body).unwrap_or_default_user()
}

trait UnwrapOrDefaultUser {
    fn unwrap_or_default_user(self) -> User;
}

impl<E> UnwrapOrDefaultUser for Result<User, E> {
    fn unwrap_or_default_user(self) -> User {
        self.unwrap_or(User {
            name: String::new(),
            email: String::new(),
        })
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            name: String::new(),
            email: String::new(),
        }
    }
}

// --- TEST block — even unwrap here is OK (excluded by pattern-not-inside) ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_safe() -> Result<(), HandlerError> {
        // ok: cwe-755-panic-on-result-fn (this fn returns Result; .unwrap() acceptable in test)
        let body = r#"{"name":"alice","email":"a@example.com"}"#;
        let _user: User = serde_json::from_str(body).unwrap();
        Ok(())
    }
}

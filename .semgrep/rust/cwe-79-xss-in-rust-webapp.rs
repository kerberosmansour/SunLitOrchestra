// Paired fixture for cwe-79-xss-in-rust-webapp.

mod axum {
    pub mod response {
        pub struct Html<T>(pub T);
        impl<T> Html<T> {
            pub fn from(t: T) -> Self {
                Html(t)
            }
        }
    }
}

fn render_unsafe(name: String) -> axum::response::Html<String> {
    // ruleid: cwe-79-xss-in-rust-webapp
    axum::response::Html(format!("<h1>Welcome, {}!</h1>", name))
}

fn render_unsafe_from(name: String) -> axum::response::Html<String> {
    // ruleid: cwe-79-xss-in-rust-webapp
    axum::response::Html::from(format!("<div>{}</div>", name))
}

fn render_unsafe_write(name: String) -> axum::response::Html<String> {
    let mut out = String::new();
    use std::fmt::Write;
    // ruleid: cwe-79-xss-in-rust-webapp
    write!(out, "<p>{}</p>", name).unwrap();
    axum::response::Html(out)
}

// --- OK shapes (rule MUST NOT fire) ---

fn render_safe(name: String) -> axum::response::Html<String> {
    let escaped = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    // ok: cwe-79-xss-in-rust-webapp
    axum::response::Html(format!("<h1>Welcome, {}!</h1>", escaped))
}

fn return_static() -> axum::response::Html<&'static str> {
    // ok: cwe-79-xss-in-rust-webapp
    axum::response::Html("<h1>Static page</h1>")
}

// Paired fixture for cwe-697-incorrect-comparison.

fn check_token(received: &str, expected: &str) -> bool {
    // ruleid: cwe-697-incorrect-comparison
    received == expected
}

fn check_password(received: &str, expected: &str) -> bool {
    // ruleid: cwe-697-incorrect-comparison
    received == expected
}

fn verify_signature(sig: &[u8], expected: &[u8]) -> bool {
    // ruleid: cwe-697-incorrect-comparison
    sig == expected
}

// --- OK shapes (rule MUST NOT fire) ---

fn check_token_safe(received: &[u8], expected: &[u8]) -> bool {
    // ok: cwe-697-incorrect-comparison
    if received.len() != expected.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in received.iter().zip(expected.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}

fn unrelated_eq(a: i32, b: i32) -> bool {
    // ok: cwe-697-incorrect-comparison
    a == b
}

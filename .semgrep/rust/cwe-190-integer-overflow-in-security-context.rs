// Paired fixture for cwe-190-integer-overflow-in-security-context.

fn alloc_with_plus(header: usize, body: usize) -> Vec<u8> {
    // ruleid: cwe-190-integer-overflow-in-security-context
    Vec::with_capacity(header + body)
}

fn alloc_with_mul(count: usize, size: usize) -> Vec<u8> {
    // ruleid: cwe-190-integer-overflow-in-security-context
    Vec::with_capacity(count * size)
}

fn build_string_plus(prefix: usize, suffix: usize) -> String {
    // ruleid: cwe-190-integer-overflow-in-security-context
    String::with_capacity(prefix + suffix)
}

fn build_string_mul(per_chunk: usize, chunks: usize) -> String {
    // ruleid: cwe-190-integer-overflow-in-security-context
    String::with_capacity(per_chunk * chunks)
}

// --- OK shapes (rule MUST NOT fire) ---

fn alloc_safe(header: usize, body: usize) -> Option<Vec<u8>> {
    let cap = header.checked_add(body)?;
    // ok: cwe-190-integer-overflow-in-security-context
    Some(Vec::with_capacity(cap))
}

fn alloc_const() -> Vec<u8> {
    // ok: cwe-190-integer-overflow-in-security-context
    Vec::with_capacity(1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_const_works() {
        // ok: cwe-190-integer-overflow-in-security-context (in test block — pattern-not-inside excludes)
        let v: Vec<u8> = Vec::with_capacity(2 + 2);
        assert_eq!(v.capacity(), 4);
    }
}

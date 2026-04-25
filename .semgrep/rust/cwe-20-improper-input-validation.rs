// Paired fixture for cwe-20-improper-input-validation.

fn matches_dynamic_regex(pattern: &str, target: &str) -> bool {
    // ruleid: cwe-20-improper-input-validation
    let re = regex::Regex::new(pattern).unwrap();
    re.is_match(target)
}

fn read_user_path(file: &str) -> std::io::Result<Vec<u8>> {
    // ruleid: cwe-20-improper-input-validation
    let p = std::path::PathBuf::from(format!("./uploads/{}", file));
    std::fs::read(p)
}

fn read_format_direct(file: &str) -> std::io::Result<Vec<u8>> {
    // ruleid: cwe-20-improper-input-validation
    std::fs::read(format!("./uploads/{}", file))
}

// --- OK shapes (rule MUST NOT fire) ---

fn read_safe(file: &str) -> std::io::Result<Vec<u8>> {
    // ok: cwe-20-improper-input-validation
    if file.contains('/') || file.contains("..") {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "bad name"));
    }
    let p = std::path::PathBuf::from("./uploads").join(file);
    std::fs::read(p)
}

fn matches_constant_regex(target: &str) -> bool {
    // ok: cwe-20-improper-input-validation
    let re = regex::Regex::new(r"^[a-z0-9_]+$").unwrap();
    re.is_match(target)
}

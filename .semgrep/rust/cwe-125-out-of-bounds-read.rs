// Paired fixture for cwe-125-out-of-bounds-read.

fn read_unchecked(buf: &[u8], i: usize) -> u8 {
    unsafe {
        // ruleid: cwe-125-out-of-bounds-read
        *buf.get_unchecked(i)
    }
}

unsafe fn read_packet_attacker_len(ptr: *const u8, len_from_attacker: usize) -> &'static [u8] {
    // ruleid: cwe-125-out-of-bounds-read
    std::slice::from_raw_parts(ptr, len_from_attacker)
}

fn parse_chunk_unbounded(buf: &[u8]) -> &[u8] {
    // ruleid: cwe-125-out-of-bounds-read
    let claimed_len = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
    &buf[4..4 + claimed_len]
}

fn parse_chunk_be_unbounded(buf: &[u8]) -> &[u8] {
    // ruleid: cwe-125-out-of-bounds-read
    let claimed_len = u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
    &buf[4..4 + claimed_len]
}

// --- OK shapes (rule MUST NOT fire) ---

fn read_safe(buf: &[u8], i: usize) -> Option<u8> {
    // ok: cwe-125-out-of-bounds-read
    buf.get(i).copied()
}

fn parse_chunk_safe(buf: &[u8]) -> Option<&[u8]> {
    if buf.len() < 4 {
        return None;
    }
    // ok: cwe-125-out-of-bounds-read
    let claimed_len = u32::from_le_bytes(buf[0..4].try_into().ok()?) as usize;
    let end = 4_usize.checked_add(claimed_len)?;
    if end > buf.len() {
        return None;
    }
    Some(&buf[4..end])
}

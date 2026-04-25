// Paired fixture for cwe-787-out-of-bounds-write.

fn build_unbounded(buf: Vec<u8>, claimed: usize) -> Vec<u8> {
    let mut v = buf;
    unsafe {
        // ruleid: cwe-787-out-of-bounds-write
        v.set_len(claimed);
    }
    v
}

unsafe fn copy_in_unbounded(dst: *mut u8, src: *const u8, n: usize) {
    // ruleid: cwe-787-out-of-bounds-write
    std::ptr::copy_nonoverlapping(src, dst, n)
}

fn write_unchecked(buf: &mut [u8], i: usize, b: u8) {
    unsafe {
        // ruleid: cwe-787-out-of-bounds-write
        *buf.get_unchecked_mut(i) = b;
    }
}

// --- OK shapes (rule MUST NOT fire) ---

fn build_safe(buf: Vec<u8>, claimed: usize) -> Option<Vec<u8>> {
    let mut v = buf;
    if claimed > v.capacity() {
        return None;
    }
    // ok: cwe-787-out-of-bounds-write
    unsafe { v.set_len(claimed); }
    Some(v)
}

fn write_safe(buf: &mut [u8], i: usize, b: u8) -> Option<()> {
    // ok: cwe-787-out-of-bounds-write
    *buf.get_mut(i)? = b;
    Some(())
}

// Paired fixture for cwe-672-operation-after-expiration.

use std::os::fd::AsRawFd;
use std::time::SystemTime;

extern "C" {
    fn libc_read(fd: i32, buf: *mut u8, n: usize) -> i64;
}

unsafe fn use_closed_fd(f: std::fs::File) {
    let fd = f.as_raw_fd();
    drop(f);
    let mut buf = [0u8; 16];
    // ruleid: cwe-672-operation-after-expiration
    libc_read(fd, buf.as_mut_ptr(), buf.len());
}

fn extend_rwlock_borrow(lock: &std::sync::RwLock<i32>) {
    let g = lock.read().unwrap();
    let p = &*g as *const i32;
    drop(g);
    unsafe {
        // ruleid: cwe-672-operation-after-expiration
        let _ = *p;
    }
}

fn extend_rwlock_write_borrow(lock: &std::sync::RwLock<i32>) {
    let mut g = lock.write().unwrap();
    let p = &mut *g as *mut i32;
    drop(g);
    unsafe {
        // ruleid: cwe-672-operation-after-expiration
        *p = 9;
    }
}

struct Session {
    expires_at: SystemTime,
    user_id: String,
}

fn check_session_expired_warn_only(session: &Session) -> bool {
    // ruleid: cwe-672-operation-after-expiration
    if session.expires_at < std::time::SystemTime::now() {
        log::warn!("session expired");
    }
    session.user_id == "admin"
}

// --- OK shapes (rule MUST NOT fire) ---

fn process_file_safe(path: &str) -> std::io::Result<()> {
    let f = std::fs::File::open(path)?;
    // ok: cwe-672-operation-after-expiration
    let _meta = f.metadata()?;
    Ok(())
}

fn check_session_safe(session: &Session) -> bool {
    if session.expires_at < std::time::SystemTime::now() {
        // ok: cwe-672-operation-after-expiration
        return false;
    }
    session.user_id == "admin"
}

fn read_lock_short_lived(lock: &std::sync::RwLock<i32>) -> i32 {
    // ok: cwe-672-operation-after-expiration
    *lock.read().unwrap()
}

// Paired fixture for cwe-416-use-after-free.

unsafe fn use_after_drop() {
    let v = vec![1u8, 2, 3];
    let p = v.as_ptr();
    drop(v);
    // ruleid: cwe-416-use-after-free
    let _ = *p;
}

unsafe fn vec_realloc_use_after() {
    let mut v: Vec<u8> = Vec::with_capacity(4);
    let p = v.as_mut_ptr();
    v.set_len(8);
    // ruleid: cwe-416-use-after-free
    v.push(1);
    *p = 9;
}

extern "C" {
    fn ffi_register_callback(cb: extern "C" fn(*mut std::ffi::c_void), ctx: *mut std::ffi::c_void);
}

extern "C" fn cb(ctx: *mut std::ffi::c_void) {
    unsafe {
        let s = &*(ctx as *const String);
        println!("{s}");
    }
}

fn register_callback_with_borrowed_local() {
    let s = String::from("hello");
    unsafe {
        // ruleid: cwe-416-use-after-free
        ffi_register_callback(cb, &s as *const _ as *mut _);
    }
}

// --- OK shapes (rule MUST NOT fire) ---

fn safe_owned() {
    let v = vec![1u8, 2, 3];
    // ok: cwe-416-use-after-free
    let _sum: u8 = v.iter().sum();
}

fn safe_arc_share(arc: std::sync::Arc<String>) {
    // ok: cwe-416-use-after-free
    let cloned = arc.clone();
    std::thread::spawn(move || {
        println!("{cloned}");
    });
}

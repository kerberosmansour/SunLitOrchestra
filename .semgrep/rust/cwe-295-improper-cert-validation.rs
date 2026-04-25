// Paired fixture for cwe-295-improper-cert-validation.

fn build_dangerous_certs() {
    // ruleid: cwe-295-improper-cert-validation
    let _ = reqwest_stub_builder().danger_accept_invalid_certs(true);
}

fn build_dangerous_hostnames() {
    // ruleid: cwe-295-improper-cert-validation
    let _ = reqwest_stub_builder().danger_accept_invalid_hostnames(true);
}

fn install_dangerous_verifier(config: &mut RustlsConfigStub) {
    // ruleid: cwe-295-improper-cert-validation
    let _ = config.dangerous().set_certificate_verifier(my_verifier());
}

// --- OK shapes (rule MUST NOT fire) ---

fn build_safe() {
    // ok: cwe-295-improper-cert-validation
    let _ = reqwest_stub_builder()
        .add_root_certificate(my_root());
}

// Stubs so the fixture compiles when checked-clean against this directory.
// The rule pattern matches at the AST level regardless of these signatures.
struct ReqwestStubBuilder;
impl ReqwestStubBuilder {
    fn danger_accept_invalid_certs(self, _: bool) -> Self {
        self
    }
    fn danger_accept_invalid_hostnames(self, _: bool) -> Self {
        self
    }
    fn add_root_certificate(self, _: u8) -> Self {
        self
    }
}
fn reqwest_stub_builder() -> ReqwestStubBuilder {
    ReqwestStubBuilder
}
fn my_root() -> u8 {
    0
}

struct RustlsConfigStub;
impl RustlsConfigStub {
    fn dangerous(&mut self) -> RustlsConfigStubDangerous {
        RustlsConfigStubDangerous
    }
}
struct RustlsConfigStubDangerous;
impl RustlsConfigStubDangerous {
    fn set_certificate_verifier(self, _: u8) -> Self {
        self
    }
}
fn my_verifier() -> u8 {
    0
}

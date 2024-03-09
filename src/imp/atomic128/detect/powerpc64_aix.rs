// Run-time feature detection on powerpc64 AIX by using getsystemcfg.
//
// Refs:
// - https://github.com/golang/go/blob/071aed2aaa0ed819582c5bff44b70d43c61f504a/src/internal/cpu/cpu_ppc64x_aix.go

include!("common.rs");

// core::ffi::c_* (except c_void) requires Rust 1.64, libc will soon require Rust 1.47
mod ffi {
    pub(crate) use super::c_types::{c_int, c_ulong};

    // https://github.com/rust-lang/libc/blob/0.2.140/src/unix/aix/mod.rs#L1576
    // https://github.com/golang/go/blob/071aed2aaa0ed819582c5bff44b70d43c61f504a/src/internal/cpu/cpu_ppc64x_aix.go
    pub(crate) const SC_IMPL: c_int = 2;
    pub(crate) const POWER_8: c_ulong = 0x10000;

    extern "C" {
        // https://www.ibm.com/docs/en/aix/7.3?topic=g-getsystemcfg-subroutine
        // https://github.com/rust-lang/libc/blob/0.2.140/src/unix/aix/powerpc64.rs#L569
        pub(crate) fn getsystemcfg(name: c_int) -> c_ulong;
    }
}

#[cold]
fn _detect(info: &mut CpuInfo) {
    // SAFETY: calling getsystemcfg is safe.
    let impl_ = unsafe { ffi::getsystemcfg(ffi::SC_IMPL) };
    if impl_ == ffi::c_ulong::MAX {
        return;
    }
    if impl_ & ffi::POWER_8 != 0 {
        info.set(CpuInfo::HAS_QUADWORD_ATOMICS);
    }
}

#[allow(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_imports
)]
#[cfg(test)]
mod tests {
    use super::*;

    // Static assertions for FFI bindings.
    // This checks that FFI bindings defined in this crate and FFI bindings defined
    // in libc compatible signatures (or the same values if constants).
    // Since this is static assertion, we can detect problems with
    // `cargo check --tests --target <target>` run in CI (via TESTS=1 build.sh)
    // without actually running tests on these platforms.
    // See also tools/codegen/src/ffi.rs.
    // TODO: auto-generate this test
    #[allow(
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::no_effect_underscore_binding
    )]
    const _: fn() = || {
        use test_helper::libc;
        let mut _getsystemcfg: unsafe extern "C" fn(ffi::c_int) -> ffi::c_ulong = ffi::getsystemcfg;
        _getsystemcfg = libc::getsystemcfg;
        static_assert!(ffi::SC_IMPL == libc::SC_IMPL);
        static_assert!(ffi::POWER_8 == libc::POWER_8 as ffi::c_ulong);
    };
}

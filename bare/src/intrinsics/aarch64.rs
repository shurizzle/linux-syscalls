extern "C" {
    // TODO: set the right value at runtime
    pub(crate) static mut __aarch64_have_lse_atomics: bool;
}

#[no_mangle]
extern "C" fn getauxval(_t: core::ffi::c_ulong) -> core::ffi::c_ulong {
    0
}

#[path = "common.rs"]
mod common;
pub use common::{
    syscall0, syscall0_readonly, syscall1, syscall1_noreturn, syscall1_readonly, syscall2,
    syscall2_readonly, syscall3, syscall3_readonly, syscall4, syscall4_readonly, syscall5,
    syscall5_readonly, syscall6, syscall6_readonly,
};

extern "C" {
    #[allow(dead_code)]
    fn linux_syscalls_rs_get_scv() -> bool;
    fn linux_syscalls_rs_set_scv(val: bool);
}

pub(crate) unsafe fn init() {
    if crate::env::aux::get::<crate::env::aux::HardwareCapabilities2>().map_or(false, |flags| {
        flags.contains(crate::env::aux::Features2::SCV)
    }) {
        linux_syscalls_rs_set_scv(true);
    }
}

#[inline]
pub fn has_scv() -> bool {
    unsafe { linux_syscalls_rs_get_scv() }
}

#[cfg_attr(target_arch = "x86", path = "x86.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
#[cfg_attr(target_arch = "aarch64", path = "aarch64.rs")]
#[cfg_attr(target_arch = "arm", path = "arm.rs")]
#[cfg_attr(target_arch = "loongarch64", path = "loongarch64.rs")]
mod arch;

#[cfg(target_arch = "x86")]
pub(crate) use self::arch::quasi_init;
pub use self::arch::Vdso;
#[allow(unused_imports)]
pub(crate) use self::arch::{init, VDSO};

#[inline]
pub(crate) unsafe fn get() -> &'static Vdso {
    &*self::VDSO.get()
}

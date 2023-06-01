pub mod aux;
#[cfg_attr(target_arch = "x86", path = "kernel/x86.rs")]
pub mod kernel;
pub mod vdso;

pub use self::{kernel::utsname, kernel::Version, vdso::Vdso};

pub fn getauxval<T: aux::VdsoKey>() -> Option<T::Item> {
    #[cfg(not(feature = "bare"))]
    crate::init();
    unsafe { aux::get::<T>() }
}

pub fn kernel_version() -> Version {
    #[cfg(not(feature = "bare"))]
    crate::init();
    unsafe { kernel::version() }
}

pub fn uname() -> utsname {
    #[cfg(not(feature = "bare"))]
    crate::init();
    unsafe { kernel::uname() }
}

pub fn vdso() -> &'static Vdso {
    #[cfg(not(feature = "bare"))]
    crate::init();
    unsafe { vdso::get() }
}

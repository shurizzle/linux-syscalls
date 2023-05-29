pub mod aux;
#[cfg_attr(target_arch = "x86", path = "kernel/x86.rs")]
pub mod kernel;
pub mod vdso;

pub use self::{
    kernel::{utsname, Version},
    vdso::Vdso,
};

pub fn getauxval<T: aux::VdsoKey>() -> Option<T::Item> {
    crate::init();
    unsafe { aux::get::<T>() }
}

pub fn kernel_version() -> Version {
    crate::init();
    unsafe { kernel::version() }
}

pub fn uname() -> utsname {
    crate::init();
    unsafe { kernel::uname() }
}

pub fn vdso() -> &'static Vdso {
    crate::init();
    unsafe { vdso::get() }
}

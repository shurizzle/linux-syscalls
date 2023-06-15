pub mod aux;
#[cfg_attr(target_arch = "x86", path = "kernel/x86.rs")]
pub mod kernel;
pub mod vdso;

pub use self::{kernel::utsname, kernel::Version, vdso::Vdso};

#[inline]
pub unsafe fn unchecked_getauxval<T: aux::VdsoKey>() -> Option<T::Item> {
    aux::get::<T>()
}

#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn getauxval<T: aux::VdsoKey>() -> Option<T::Item> {
    crate::init();
    unsafe { unchecked_getauxval::<T>() }
}

#[inline]
pub unsafe fn unchecked_kernel_version() -> &'static Version {
    kernel::version()
}

#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn kernel_version() -> &'static Version {
    crate::init();
    unsafe { unchecked_kernel_version() }
}

#[inline]
pub unsafe fn unchecked_uname() -> &'static utsname {
    kernel::uname()
}

#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn uname() -> &'static utsname {
    crate::init();
    unsafe { unchecked_uname() }
}

#[inline]
pub unsafe fn unchecked_vdso() -> &'static Vdso {
    vdso::get()
}

#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn vdso() -> &'static Vdso {
    crate::init();
    unsafe { unchecked_vdso() }
}

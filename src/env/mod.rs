//! A collection of environment informations useful to detect features used in
//! syscalls.

pub mod aux;
#[cfg_attr(target_arch = "x86", path = "kernel/x86.rs")]
pub mod kernel;
pub mod vdso;

pub use self::{kernel::utsname, kernel::Version, vdso::Vdso};

/// Returns values from the auxiliary vector, a mechanism that the kernel's ELF
/// binary loader uses to pass certain information to user space when a program
/// is executed.
///
/// Keys implements [crate::env::aux::VdsoKey]
///
/// # Safety
///
/// This function is marked as unsafe because it doesn't check if library is
/// initialized.
#[inline]
pub unsafe fn unchecked_getauxval<T: aux::VdsoKey>() -> Option<T::Item> {
    aux::get::<T>()
}

/// Returns values from the auxiliary vector, a mechanism that the kernel's ELF
/// binary loader uses to pass certain information to user space when a program
/// is executed.
///
/// Keys implements [crate::env::aux::VdsoKey]
#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn getauxval<T: aux::VdsoKey>() -> Option<T::Item> {
    crate::init();
    unsafe { unchecked_getauxval::<T>() }
}

/// Returns a cached kernel version.
///
/// # Safety
///
/// This function is marked as unsafe because it doesn't check if library is
/// initialized.
#[inline]
pub unsafe fn unchecked_kernel_version() -> &'static Version {
    kernel::version()
}

/// Returns a cached kernel version.
#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn kernel_version() -> &'static Version {
    crate::init();
    unsafe { unchecked_kernel_version() }
}

/// Returns a cached [crate::env::kernel::utsname].
/// It do not use `uname` syscall.
///
/// # Safety
///
/// This function is marked as unsafe because it doesn't check if library is
/// initialized.
#[inline]
pub unsafe fn unchecked_uname() -> &'static utsname {
    kernel::uname()
}

/// Returns a cached [crate::env::kernel::utsname].
/// It do not use `uname` syscall.
#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn uname() -> &'static utsname {
    crate::init();
    unsafe { unchecked_uname() }
}

/// Returns the cached [crate::env::vdso::Vdso] for the current process.
///
/// # Safety
///
/// This function is marked as unsafe because it doesn't check if library is
/// initialized.
#[inline]
pub unsafe fn unchecked_vdso() -> &'static Vdso {
    vdso::get()
}

/// Returns cached [crate::env::vdso::Vdso] for the current process.
#[cfg(any(docs_rs, not(feature = "bare")))]
#[cfg_attr(docs_rs, doc(cfg(not(feature = "bare"))))]
pub fn vdso() -> &'static Vdso {
    crate::init();
    unsafe { unchecked_vdso() }
}

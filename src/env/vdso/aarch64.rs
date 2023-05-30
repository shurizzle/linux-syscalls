use core::cell::UnsafeCell;

use crate::env::aux::SysInfoHeader;

pub use linux_raw_vdso::Vdso as RawVdso;

#[non_exhaustive]
#[repr(transparent)]
pub struct Vdso(pub(crate) RawVdso);

unsafe impl Send for Vdso {}
unsafe impl Sync for Vdso {}

impl Vdso {
    #[inline]
    pub fn rt_sigreturn(&self) -> *const core::ffi::c_void {
        self.0.rt_sigreturn
    }

    #[inline]
    pub fn gettimeofday(&self) -> *const core::ffi::c_void {
        self.0.gettimeofday
    }

    #[inline]
    pub fn clock_gettime(&self) -> *const core::ffi::c_void {
        self.0.clock_gettime
    }

    #[inline]
    pub fn clock_getres(&self) -> *const core::ffi::c_void {
        self.0.clock_getres
    }
}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso(RawVdso {
    rt_sigreturn: core::ptr::null(),
    gettimeofday: core::ptr::null(),
    clock_gettime: core::ptr::null(),
    clock_getres: core::ptr::null(),
}));

pub(crate) unsafe fn init() {
    if let Some(sysinfo) = crate::env::aux::get::<SysInfoHeader>() {
        (*VDSO.get()).0 = RawVdso::from_ptr(sysinfo).expect("Invalid vDSO");
    }
}

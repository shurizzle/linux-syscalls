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
    pub fn clock_gettime(&self) -> *const core::ffi::c_void {
        self.0.clock_gettime
    }

    #[inline]
    pub fn gettimeofday(&self) -> *const core::ffi::c_void {
        self.0.gettimeofday
    }

    #[inline]
    pub fn time(&self) -> *const core::ffi::c_void {
        self.0.time
    }

    #[inline]
    pub fn getcpu(&self) -> *const core::ffi::c_void {
        self.0.getcpu
    }
}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso(RawVdso {
    clock_gettime: core::ptr::null(),
    gettimeofday: core::ptr::null(),
    time: core::ptr::null(),
    getcpu: core::ptr::null(),
}));

pub(crate) unsafe fn init() {
    let sysinfo = crate::env::aux::get::<SysInfoHeader>().expect("system info header not found");
    (*VDSO.get()).0 = RawVdso::from_ptr(sysinfo).expect("Invalid vDSO");
}

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
    pub fn sigreturn(&self) -> *const core::ffi::c_void {
        self.0.sigreturn
    }

    #[inline]
    pub fn rt_sigreturn(&self) -> *const core::ffi::c_void {
        self.0.rt_sigreturn
    }

    #[inline]
    pub fn vsyscall(&self) -> *const core::ffi::c_void {
        self.0.vsyscall
    }

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
}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso(RawVdso {
    sigreturn: core::ptr::null(),
    rt_sigreturn: core::ptr::null(),
    vsyscall: core::ptr::null(),
    clock_gettime: core::ptr::null(),
    gettimeofday: core::ptr::null(),
    time: core::ptr::null(),
}));

pub(crate) unsafe fn quasi_init() {
    let sysinfo = crate::env::aux::get::<SysInfoHeader>().expect("system info header not found");
    (*VDSO.get()).0 = RawVdso::from_ptr(sysinfo).expect("Invalid vDSO");
    crate::arch::x86_init((*VDSO.get()).0.vsyscall as *const ());
}

pub(crate) unsafe fn init() {
    let v = crate::env::Version {
        major: 3,
        minor: 1,
        revision: 5,
    };
    if *crate::env::kernel::KERNEL_VERSION.get() >= v {
        let vdso = &*VDSO.get();
        if vdso.0.clock_gettime.is_null() || vdso.0.clock_gettime.is_null() || vdso.0.time.is_null()
        {
            panic!("Invalid vDSO");
        }
    }
}

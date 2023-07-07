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
    pub fn clock_getres(&self) -> *const core::ffi::c_void {
        self.0.clock_getres
    }
}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso(RawVdso {
    gettimeofday: core::ptr::null(),
    clock_gettime: core::ptr::null(),
    clock_getres: core::ptr::null(),
}));

pub(crate) unsafe fn init() {
    if let Some(sysinfo) = crate::env::aux::get::<SysInfoHeader>() {
        (*VDSO.get()).0 = RawVdso::from_ptr(sysinfo).expect("Invalid vDSO");
    } else {
        return;
    }

    let is_ge_4_4 = {
        let v = crate::env::Version {
            major: 4,
            minor: 4,
            revision: 0,
        };
        *crate::env::kernel::KERNEL_VERSION.get() >= v
    };

    if is_ge_4_4 {
        let vdso = &*VDSO.get();
        if vdso.0.clock_gettime.is_null() || vdso.0.gettimeofday.is_null() {
            panic!("Invalid vDSO");
        }
    }
}

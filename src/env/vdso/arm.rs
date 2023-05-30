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
    pub fn gettimeofday(&self) -> *const core::ffi::c_void {
        self.0.gettimeofday
    }

    #[inline]
    pub fn clock_gettime(&self) -> *const core::ffi::c_void {
        self.0.clock_gettime
    }
}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso(RawVdso {
    gettimeofday: core::ptr::null(),
    clock_gettime: core::ptr::null(),
}));

pub(crate) unsafe fn init() {
    let is_gt_4_1 = {
        let v = crate::env::Version {
            major: 4,
            minor: 1,
            revision: 0,
        };
        *crate::env::kernel::KERNEL_VERSION.get() >= v
    };

    match crate::env::aux::get::<SysInfoHeader>() {
        Some(sysinfo) => {
            (*VDSO.get()).0 = RawVdso::from_ptr(sysinfo).expect("Invalid vDSO");
        }
        _ => return,
    }

    if is_gt_4_1 {
        let vdso = &*VDSO.get();
        if vdso.0.clock_gettime.is_null() || vdso.0.gettimeofday.is_null() {
            panic!("Invalid vDSO");
        }
    }
}

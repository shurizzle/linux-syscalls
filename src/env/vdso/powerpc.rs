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
    pub fn clock_getres(&self) -> *const core::ffi::c_void {
        self.0.clock_getres
    }

    #[inline]
    pub fn clock_gettime(&self) -> *const core::ffi::c_void {
        self.0.clock_gettime
    }

    #[inline]
    pub fn clock_gettime64(&self) -> *const core::ffi::c_void {
        self.0.clock_gettime64
    }

    #[inline]
    pub fn datapage_offset(&self) -> *const core::ffi::c_void {
        self.0.datapage_offset
    }

    #[inline]
    pub fn get_syscall_map(&self) -> *const core::ffi::c_void {
        self.0.get_syscall_map
    }

    #[inline]
    pub fn get_tbfreq(&self) -> *const core::ffi::c_void {
        self.0.get_tbfreq
    }

    #[inline]
    pub fn gettimeofday(&self) -> *const core::ffi::c_void {
        self.0.gettimeofday
    }

    #[inline]
    pub fn sigtramp_rt32(&self) -> *const core::ffi::c_void {
        self.0.sigtramp_rt32
    }

    #[inline]
    pub fn sigtramp32(&self) -> *const core::ffi::c_void {
        self.0.sigtramp32
    }

    #[inline]
    pub fn sync_dicache(&self) -> *const core::ffi::c_void {
        self.0.sync_dicache
    }

    #[inline]
    pub fn sync_dicache_p5(&self) -> *const core::ffi::c_void {
        self.0.sync_dicache_p5
    }
}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso(RawVdso {
    clock_getres: core::ptr::null(),
    clock_gettime: core::ptr::null(),
    clock_gettime64: core::ptr::null(),
    datapage_offset: core::ptr::null(),
    get_syscall_map: core::ptr::null(),
    get_tbfreq: core::ptr::null(),
    gettimeofday: core::ptr::null(),
    sigtramp_rt32: core::ptr::null(),
    sigtramp32: core::ptr::null(),
    sync_dicache: core::ptr::null(),
    sync_dicache_p5: core::ptr::null(),
}));

pub(crate) unsafe fn init() {
    if let Some(sysinfo) = crate::env::aux::get::<SysInfoHeader>() {
        (*VDSO.get()).0 = RawVdso::from_ptr(sysinfo).expect("Invalid vDSO");
    } else {
        return;
    }

    let is_ge_5_11 = {
        let v = crate::env::Version {
            major: 5,
            minor: 11,
            revision: 0,
        };
        *crate::env::kernel::KERNEL_VERSION.get() >= v
    };

    if is_ge_5_11 {
        let vdso = &*VDSO.get();
        if vdso.0.clock_gettime64.is_null() {
            panic!("Invalid vDSO");
        }
    }
}

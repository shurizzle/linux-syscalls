use core::cell::UnsafeCell;

#[non_exhaustive]
pub struct Vdso {}

unsafe impl Send for Vdso {}
unsafe impl Sync for Vdso {}

pub(crate) static mut VDSO: UnsafeCell<Vdso> = UnsafeCell::new(Vdso {});

pub(crate) fn init() {}

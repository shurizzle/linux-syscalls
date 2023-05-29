mod common;

pub(crate) use self::common::{uname, version};
pub use self::common::{utsname, Version};
#[allow(unused_imports)]
pub(crate) use self::common::{KERNEL_VERSION, UNAME};
use crate::Sysno;

pub(crate) unsafe fn init() {
    crate::env::vdso::quasi_init();
    crate::syscall1(Sysno::uname, common::UNAME.get() as usize).expect("Failed to get uname");
    common::init_version();
}

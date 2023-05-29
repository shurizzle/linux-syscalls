pub mod common;

pub(crate) use self::common::{uname, version};
pub use self::common::{utsname, Version};
use crate::{syscall, Sysno};

#[allow(unused_imports)]
pub(crate) use common::{KERNEL_VERSION, UNAME};

pub(crate) unsafe fn init() {
    syscall!(Sysno::uname, self::common::UNAME.get()).expect("Failed to get uname");
    self::common::init_version();
}

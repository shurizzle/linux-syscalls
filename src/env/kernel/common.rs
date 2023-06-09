use core::{cell::UnsafeCell, fmt};

#[allow(non_camel_case_types)]
/// Structure describing the system and machine.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utsname {
    /// Name of the implementation of the operating system.
    pub sysname: [u8; 65],
    /// Name of this node on the network.
    pub nodename: [u8; 65],
    /// Current release level of this implementation.
    pub release: [u8; 65],
    /// Current version level of this release.
    pub version: [u8; 65],
    /// Name of the hardware type the system is running on.
    pub machine: [u8; 65],
    // Name of the domain of this node on the network.
    pub domainname: [u8; 65],
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Structure describing the kernel version.
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

impl fmt::Display for Version {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}

#[cfg(feature = "bare")]
/// A macro to create/query kernel version.
///
/// # Create a Version
///
/// `kversion!(major [, minor [, revision]])`
///
/// # Query version
///
/// `kversion!( (>|<|==|>=|<=) major [, minor [, revision]])`
///
/// # Examples
///
/// ```
/// kversion!(4);          // Create version 4.0.0
/// kversion!(4, 11);      // Create version 4.11.0
/// kversion!(4, 11, 6)    // Create version 4.11.6
/// kversion!(>= 4);       // `true` if kernel version is 4.0.0 or superior
/// kversion!(< 4, 11);    // `true` if kernel version is earlier than 4.11.0
/// kversion!(== 5, 15, 6) // `true` if kernel is exactly 5.15.6
/// ```
#[macro_export]
macro_rules! kversion {
    (>  $($rest:tt)+) => {
        *$crate::env::kernel_version()  > $crate::kversion!($($rest)+)
    };
    (<  $($rest:tt)+) => {
        *$crate::env::kernel_version()  < $crate::kversion!($($rest)+)
    };
    (== $($rest:tt)+) => {
        *$crate::env::kernel_version() == $crate::kversion!($($rest)+)
    };
    (>= $($rest:tt)+) => {
        *$crate::env::kernel_version() >= $crate::kversion!($($rest)+)
    };
    (<= $($rest:tt)+) => {
        *$crate::env::kernel_version() <= $crate::kversion!($($rest)+)
    };
    ($major:expr) => {
        $crate::kversion!($major, 0)
    };
    ($major:expr, $minor:expr) => {
        $crate::kversion!($major, $minor, 0)
    };
    ($major:expr, $minor:expr, $revision:expr) => {
        $crate::env::kernel::Version {
            major: $major,
            minor: $minor,
            revision: $revision,
        }
    };
}

#[cfg(not(feature = "bare"))]
/// A macro to create/query kernel version.
///
/// # Create a Version
///
/// `kversion!(major [, minor [, revision]])`
///
/// # Query version
///
/// `kversion!( (>|<|==|>=|<=) major [, minor [, revision]])`
///
/// # Examples
///
/// ```
/// kversion!(4);          // Create version 4.0.0
/// kversion!(4, 11);      // Create version 4.11.0
/// kversion!(4, 11, 6)    // Create version 4.11.6
/// kversion!(>= 4);       // `true` if kernel version is 4.0.0 or superior
/// kversion!(< 4, 11);    // `true` if kernel version is earlier than 4.11.0
/// kversion!(== 5, 15, 6) // `true` if kernel is exactly 5.15.6
/// ```
#[macro_export]
macro_rules! kversion {
    (>  $($rest:tt)+) => {
        *$crate::env::unchecked_kernel_version()  > $crate::kversion!($($rest)+)
    };
    (<  $($rest:tt)+) => {
        *$crate::env::unchecked_kernel_version()  < $crate::kversion!($($rest)+)
    };
    (== $($rest:tt)+) => {
        *$crate::env::unchecked_kernel_version() == $crate::kversion!($($rest)+)
    };
    (>= $($rest:tt)+) => {
        *$crate::env::unchecked_kernel_version() >= $crate::kversion!($($rest)+)
    };
    (<= $($rest:tt)+) => {
        *$crate::env::unchecked_kernel_version() <= $crate::kversion!($($rest)+)
    };
    ($major:expr) => {
        $crate::kversion!($major, 0)
    };
    ($major:expr, $minor:expr) => {
        $crate::kversion!($major, $minor, 0)
    };
    ($major:expr, $minor:expr, $revision:expr) => {
        $crate::env::kernel::Version {
            major: $major,
            minor: $minor,
            revision: $revision,
        }
    };
}

pub(crate) static mut UNAME: UnsafeCell<utsname> = UnsafeCell::new(utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
});
pub(crate) static mut KERNEL_VERSION: UnsafeCell<Version> = UnsafeCell::new(kversion!(0));

pub(super) unsafe fn init_version() {
    let release = cstr(&(*UNAME.get()).release);

    let (major, release) = if let Some(res) = version_rest(release) {
        res
    } else {
        panic!("Invalid kernel version")
    };

    let (minor, release) = if let Some(res) = version_rest(release) {
        res
    } else {
        panic!("Invalid kernel version")
    };

    let revision = if let Some(res) = version_last(release) {
        res
    } else {
        panic!("Invalid kernel version")
    };

    let k = &mut *KERNEL_VERSION.get();
    k.major = major;
    k.minor = minor;
    k.revision = revision;
}

fn atoi_raw(b: &[u8]) -> Option<(u32, usize)> {
    let mut acc = 0u32;
    let mut i = 0;

    for c in b.iter().copied() {
        if c.is_ascii_digit() {
            acc = acc.checked_mul(10)?.checked_add((c - b'0') as u32)?;
        } else {
            return if i == 0 { None } else { Some((acc, i)) };
        }
        i += 1;
    }

    Some((acc, i))
}

fn version_rest(b: &[u8]) -> Option<(u32, &[u8])> {
    let (n, size) = atoi_raw(b)?;
    if Some(b'.') == b.get(size).copied() {
        Some((n, unsafe { b.get_unchecked((size + 1)..) }))
    } else {
        None
    }
}

#[inline]
fn version_last(b: &[u8]) -> Option<u32> {
    Some(atoi_raw(b)?.0)
}

unsafe fn cstr(s: &[u8]) -> &[u8] {
    if let Some(i) = s.iter().copied().position(|c| c == 0) {
        s.get_unchecked(..i)
    } else {
        s
    }
}

#[inline]
pub(crate) unsafe fn version() -> &'static Version {
    &*KERNEL_VERSION.get()
}

#[inline]
pub(crate) unsafe fn uname() -> &'static utsname {
    &*UNAME.get()
}

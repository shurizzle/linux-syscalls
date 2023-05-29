use core::ffi::c_void;

#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
type aux_t = u32;
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type aux_t = u64;

#[repr(C)]
struct AuxvPair {
    pub a_type: aux_t,
    pub a_un: aux_t,
}

mod __sealed {
    pub trait Sealed {}
}
use __sealed::Sealed;

pub trait AuxValue: Sealed {
    fn from(value: aux_t) -> Self;
}

pub trait VdsoKey: Sealed {
    const ID: aux_t;
    const N: usize;
    type Item: AuxValue;
}

impl Sealed for i32 {}
impl AuxValue for i32 {
    #[inline]
    fn from(value: aux_t) -> Self {
        value as Self
    }
}

impl Sealed for *const c_void {}
impl AuxValue for *const c_void {
    #[inline]
    fn from(value: aux_t) -> Self {
        value as Self
    }
}

impl Sealed for *const u8 {}
impl AuxValue for *const u8 {
    #[inline]
    fn from(value: aux_t) -> Self {
        value as Self
    }
}

impl Sealed for *const [u8; 16] {}
impl AuxValue for *const [u8; 16] {
    #[inline]
    fn from(value: aux_t) -> Self {
        value as Self
    }
}

impl Sealed for usize {}
impl AuxValue for usize {
    #[inline]
    fn from(value: aux_t) -> Self {
        value as usize
    }
}

impl Sealed for bool {}
impl AuxValue for bool {
    #[inline]
    fn from(value: aux_t) -> Self {
        value != 0
    }
}

impl Sealed for u32 {}
impl AuxValue for u32 {
    #[inline]
    fn from(value: aux_t) -> Self {
        value as u32
    }
}

macro_rules! def_keys {
    (__internal_def ($no:expr) $(#[$meta:meta])* $name:ident($n:literal) -> $ty:ty $(,)?) => {
        def_keys!(__decl_def ($no) $(#[$meta])* $name($n) -> $ty);
        const KEYS_LEN: usize = $no+1;
    };
    (__internal_def ($no:expr) $(#[$meta:meta])* $name:ident($n:literal) -> $ty:ty, $($rest:tt)+) => {
        def_keys!(__decl_def ($no) $(#[$meta])* $name($n) -> $ty);
        def_keys!(__internal_def ($no+1) $($rest)+);
    };
    (__decl_def ($no:expr) $(#[$meta:meta])* $name:ident($n:literal) -> $ty:ty) => {
        $(#[$meta])*
        #[non_exhaustive]
        pub struct $name;
        impl Sealed for $name {}
        impl VdsoKey for $name {
            const ID: aux_t = $n;
            const N: usize = $no;
            type Item = $ty;
        }
    };
    (__internal_match $($(#[$meta:meta])* $name:ident($n:literal) -> $ty:ty),+ $(,)?) => {
        #[inline]
        pub(crate) unsafe fn init()  {
            #[cfg(not(any(target_env = "gnu", target_env = "musl")))]
            let env = environ;
            #[cfg(any(target_env = "gnu", target_env = "musl"))]
            let env = __environ;
            let mut ptr = {
                let mut p = env;
                while !(*p).is_null() {
                    p = p.add(1);
                }
                p.add(1) as *mut AuxvPair
            };

            while (*ptr).a_type != 0 {
                #[deny(unreachable_patterns)]
                match (*ptr).a_type {
                $(
                    $name::ID => {
                        *AUXV.get_unchecked_mut($name::N) = Some((*ptr).a_un);
                    }
                )+
                    _ => (),
                }
                ptr = ptr.add(1);
            }
        }
    };
    ($($rest:tt)+) => {
        def_keys!(__internal_def (0) $($rest)+);
        def_keys!(__internal_match $($rest)+);
    }
}

def_keys! {
    ExecFd(2) -> i32,
    ProgramHeader(3) -> *const c_void,
    ProgramHeaderSize(4) -> usize,
    ProgramHeadersNumber(5) -> usize,
    PageSize(6) -> usize,
    BaseAddress(7) -> *const c_void,
    Flags(8) -> usize,
    EntryPoint(9) -> *const c_void,
    NotElf(10) -> bool,
    Uid(11) -> u32,
    Euid(12) -> u32,
    Gid(13) -> u32,
    Egid(14) -> u32,
    Platform(15) -> *const u8,
    HardwareCapabilities(16) -> usize,
    ClockFrequency(17) -> usize,
    Secure(23) -> bool,
    BasePlatform(24) -> *const u8,
    Random(25) -> *const [u8; 16],
    HardwareCapabilities2(26) -> usize,
    RSeqFeatureSize(27) -> usize,
    RSeqAlign(28) -> usize,
    Filename(31) -> *const u8,
    SysInfo(32) -> *const c_void,
    SysInfoHeader(33) -> *const c_void,
    MinimalSignalStackSize(51) -> usize,
}

extern "C" {
    #[cfg(any(target_env = "gnu", target_env = "musl"))]
    static __environ: *const *const u8;
    #[cfg(not(any(target_env = "gnu", target_env = "musl")))]
    static environ: *const *const u8;
}

pub(crate) static mut AUXV: [Option<aux_t>; KEYS_LEN] = [None; KEYS_LEN];

#[inline]
pub(crate) unsafe fn get<T: VdsoKey>() -> Option<T::Item> {
    unsafe { AUXV.get_unchecked(T::N).map(<T::Item as AuxValue>::from) }
}

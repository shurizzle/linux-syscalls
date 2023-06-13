use core::ffi::c_void;

#[cfg_attr(
    any(target_arch = "powerpc", target_arch = "powerpc64"),
    path = "powerpc.rs"
)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), path = "x86.rs")]
#[cfg_attr(target_arch = "arm", path = "arm.rs")]
#[cfg_attr(target_arch = "aarch64", path = "aarch64.rs")]
mod arch;

pub use arch::{Features, Features2};

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
    #[allow(clippy::unnecessary_cast)]
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
        pub(crate) unsafe fn init(ptr: *const ())  {
            let mut ptr = ptr as *mut AuxvPair;

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
    /// File descriptor of program (AT_EXECFD).
    ExecFd(2) -> i32,
    /// Program headers for program (AT_PHDR).
    ProgramHeader(3) -> *const c_void,
    /// Size of program header entry (AT_PHENT).
    ProgramHeaderSize(4) -> usize,
    /// Number of program headers (AT_PHNUM).
    ProgramHeadersNumber(5) -> usize,
    /// System page size (AT_PAGESZ).
    PageSize(6) -> usize,
    /// Base address of interpreter (AT_BASE).
    BaseAddress(7) -> *const c_void,
    /// Flags (AT_FLAGS).
    Flags(8) -> usize,
    /// Entry point of program (AT_ENTRY).
    EntryPoint(9) -> *const c_void,
    /// Program is not ELF (AT_NOTELF).
    NotElf(10) -> bool,
    /// Real uid (AT_UID).
    Uid(11) -> u32,
    /// Effective uid (AT_EUID).
    Euid(12) -> u32,
    /// Real gid (AT_GID).
    Gid(13) -> u32,
    /// Effective gid (AT_EGID).
    Egid(14) -> u32,
    /// String identifying CPU for optimizations (AT_PLATFORM).
    Platform(15) -> *const u8,
    /// Arch dependent hints at CPU capabilities (AT_HWCAP).
    HardwareCapabilities(16) -> Features,
    /// Frequency at which times() increments (AT_CLKTCK).
    ClockFrequency(17) -> usize,
    /// the data cache block size (AT_DCACHEBSIZE).
    DCacheBSize(19) -> usize,
    /// the instruction cache block size (AT_ICACHEBSIZE).
    ICacheBSize(20) -> usize,
    /// the unified cache block size (AT_UCACHEBSIZE).
    UCacheBSize(21) -> usize,
    /// Secure mode boolean (AT_SECURE).
    Secure(23) -> bool,
    /// String identifying real platform, may differ from AT_PLATFORM (AT_BASE_PLATFORM).
    BasePlatform(24) -> *const u8,
    /// Address of 16 random bytes (AT_RANDOM).
    Random(25) -> *const [u8; 16],
    /// Extension of AT_HWCAP (AT_HWCAP2).
    HardwareCapabilities2(26) -> Features2,
    /// Rseq supported feature size (AT_RSEQ_FEATURE_SIZE).
    RSeqFeatureSize(27) -> usize,
    /// Rseq allocation alignment (AT_RSEQ_ALIGN).
    RSeqAlign(28) -> usize,
    /// Filename of program (AT_EXECFN).
    Filename(31) -> *const u8,
    /// The entry point to the system call function in the vDSO.
    /// Not present/needed on all architectures (e.g., absent on x86-64).
    /// (AT_SYSINFO)
    SysInfo(32) -> *const c_void,
    /// The address of a page containing the virtual Dynamic Shared
    /// Object (vDSO) that the kernel creates in order to provide fast
    /// implementations of certain system calls. (AT_SYSINFO_EHDR)
    SysInfoHeader(33) -> *const c_void,
    /// The L1 instruction cache size (AT_L1I_CACHESIZE).
    L1ICacheSize(40) -> usize,
    /// Geometry of the L1 instruction cache, encoded as for
    /// AT_L1D_CACHEGEOMETRY (AT_L1I_CACHEGEOMETRY).
    L1ICacheGeometry(41) -> usize,
    /// The L1 data cache size (AT_L1D_CACHESIZE).
    L1DCacheSize(42) -> usize,
    // Geometry of the L1 data cache, encoded with the cache line size
    // in bytes in the bottom 16 bits and the cache associativity in
    // the next 16 bits.  The associativity is such that if N is the
    // 16-bit value, the cache is N-way set associative. (AT_L1D_CACHEGEOMETRY)
    L1DCacheGeometry(43) -> usize,
    /// The L2 cache size (AT_L2_CACHESIZE).
    L2CacheSize(44) -> usize,
    /// Geometry of the L2 cache, encoded as for AT_L1D_CACHEGEOMETRY (AT_L2_CACHEGEOMETRY).
    L2CacheGeometry(45) -> usize,
    /// The L3 cache size (AT_L3_CACHESIZE).
    L3CacheSize(46) -> usize,
    /// Geometry of the L3 cache, encoded as for AT_L1D_CACHEGEOMETRY (AT_L3_CACHEGEOMETRY).
    L3CacheGeometry(47) -> usize,
    /// (AT_ADI_BLKSZ)
    ADIBlockSize(48) -> usize,
    /// (AT_ADI_NBITS)
    ADINBits(49) -> usize,
    /// (AT_ADI_UEONADI)
    ADIUEOnADI(50) -> usize,
    /// Minimal stack size for signal delivery (AT_MINSIGSTKSZ).
    MinimalSignalStackSize(51) -> usize,
}

pub(crate) static mut AUXV: [Option<aux_t>; KEYS_LEN] = [None; KEYS_LEN];

#[inline]
pub(crate) unsafe fn get<T: VdsoKey>() -> Option<T::Item> {
    AUXV.get_unchecked(T::N).map(<T::Item as AuxValue>::from)
}

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

#[cfg(not(target_arch = "powerpc64"))]
type Features = usize;

#[cfg(target_arch = "powerpc64")]
crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        /// 32-bit mode.
        B32 = 0x80000000,
        /// 64-bit mode.
        B64 = 0x40000000,
        /// 601 chip, Old POWER ISA.
        C601_INSTR = 0x20000000,
        /// SIMD/Vector Unit.
        HAS_ALTIVEC = 0x10000000,
        /// Floating Point Unit.
        HAS_FPU = 0x08000000,
        /// Memory Management Unit.
        HAS_MMU = 0x04000000,
        #[allow(non_upper_case_globals)]
        /// 4xx Multiply Accumulator.
        HAS_4xxMAC = 0x02000000,
        /// Unified I/D cache.
        UNIFIED_CACHE = 0x01000000,
        /// Signal Processing ext.
        HAS_SPE = 0x00800000,
        /// SPE Float.
        HAS_EFP_SINGLE = 0x00400000,
        /// SPE Double.
        HAS_EFP_DOUBLE = 0x00200000,
        /// 601/403gx have no timebase
        NO_TB = 0x00100000,
        /// POWER4 ISA 2.00
        POWER4 = 0x00080000,
        /// POWER5 ISA 2.02
        POWER5 = 0x00040000,
        /// POWER5+ ISA 2.03
        POWER5_PLUS = 0x00020000,
        /// CELL Broadband Engine
        CELL_BE = 0x00010000,
        /// ISA Category Embedded
        BOOKE = 0x00008000,
        /// Simultaneous Multi-Threading
        SMT = 0x00004000,
        ICACHE_SNOOP = 0x00002000,
        /// ISA 2.05
        ARCH_2_05 = 0x00001000,
        /// PA Semi 6T Core
        PA6T = 0x00000800,
        /// Decimal FP Unit
        HAS_DFP = 0x00000400,
        /// P6 + mffgpr/mftgpr
        POWER6_EXT = 0x00000200,
        /// ISA 2.06
        ARCH_2_06 = 0x00000100,
        /// P7 Vector Extension.
        HAS_VSX = 0x00000080,
        PSERIES_PERFMON_COMPAT = 0x00000040,
        TRUE_LE = 0x00000002,
        PPC_LE = 0x00000001,
    }
}
#[cfg(target_arch = "powerpc64")]
impl Sealed for Features {}
#[cfg(target_arch = "powerpc64")]
impl AuxValue for Features {
    fn from(value: aux_t) -> Self {
        (value as usize).into()
    }
}

#[cfg(not(target_arch = "powerpc64"))]
type Features2 = usize;

#[cfg(target_arch = "powerpc64")]
crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features2: usize {
        /// ISA 2.07
        ARCH_2_07 = 0x80000000,
        /// Hardware Transactional Memory
        HAS_HTM = 0x40000000,
        /// Data Stream Control Register
        HAS_DSCR = 0x20000000,
        /// Event Base Branching
        HAS_EBB = 0x10000000,
        /// Integer Select
        HAS_ISEL = 0x08000000,
        /// Target Address Register
        HAS_TAR = 0x04000000,
         /// Target supports vector instruction.
        HAS_VEC_CRYPTO = 0x02000000,
        /// Kernel aborts transaction when a syscall is made.
        HTM_NOSC = 0x01000000,
        /// ISA 3.0
        ARCH_3_00 = 0x00800000,
        /// VSX IEEE Binary Float 128-bit
        HAS_IEEE128 = 0x00400000,
        /// darn instruction.
        DARN = 0x00200000,
        /// scv syscall.
        SCV = 0x00100000,
        /// TM without suspended state.
        HTM_NO_SUSPEND = 0x00080000,
        /// ISA 3.1.
        ARCH_3_1 = 0x00040000,
        /// Matrix-Multiply Assist.
        MMA = 0x00020000,
    }
}
#[cfg(target_arch = "powerpc64")]
impl Sealed for Features2 {}
#[cfg(target_arch = "powerpc64")]
impl AuxValue for Features2 {
    fn from(value: aux_t) -> Self {
        (value as usize).into()
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
    HardwareCapabilities(16) -> Features,
    ClockFrequency(17) -> usize,
    Secure(23) -> bool,
    BasePlatform(24) -> *const u8,
    Random(25) -> *const [u8; 16],
    HardwareCapabilities2(26) -> Features2,
    RSeqFeatureSize(27) -> usize,
    RSeqAlign(28) -> usize,
    Filename(31) -> *const u8,
    SysInfo(32) -> *const c_void,
    SysInfoHeader(33) -> *const c_void,
    MinimalSignalStackSize(51) -> usize,
}

pub(crate) static mut AUXV: [Option<aux_t>; KEYS_LEN] = [None; KEYS_LEN];

#[inline]
pub(crate) unsafe fn get<T: VdsoKey>() -> Option<T::Item> {
    unsafe { AUXV.get_unchecked(T::N).map(<T::Item as AuxValue>::from) }
}

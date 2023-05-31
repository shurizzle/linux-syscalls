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
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Features: usize {
        /// 32-bit mode.
        const B32 = 0x80000000;
        /// 64-bit mode.
        const B64 = 0x40000000;
        /// 601 chip, Old POWER ISA.
        const C601_INSTR = 0x20000000;
        /// SIMD/Vector Unit.
        const HAS_ALTIVEC = 0x10000000;
        /// Floating Point Unit.
        const HAS_FPU = 0x08000000;
        /// Memory Management Unit.
        const HAS_MMU = 0x04000000;
        /// 4xx Multiply Accumulator.
        const HAS_4xxMAC = 0x02000000;
        /// Unified I/D cache.
        const UNIFIED_CACHE = 0x01000000;
        /// Signal Processing ext.
        const HAS_SPE = 0x00800000;
        /// SPE Float.
        const HAS_EFP_SINGLE = 0x00400000;
        /// SPE Double.
        const HAS_EFP_DOUBLE = 0x00200000;
        /// 601/403gx have no timebase
        const NO_TB = 0x00100000;
        /// POWER4 ISA 2.00
        const POWER4 = 0x00080000;
        /// POWER5 ISA 2.02
        const POWER5 = 0x00040000;
        /// POWER5+ ISA 2.03
        const POWER5_PLUS = 0x00020000;
        /// CELL Broadband Engine
        const CELL_BE = 0x00010000;
        /// ISA Category Embedded
        const BOOKE = 0x00008000;
        /// Simultaneous Multi-Threading
        const SMT = 0x00004000;
        const ICACHE_SNOOP = 0x00002000;
        /// ISA 2.05
        const ARCH_2_05 = 0x00001000;
        /// PA Semi 6T Core
        const PA6T = 0x00000800;
        /// Decimal FP Unit
        const HAS_DFP = 0x00000400;
        /// P6 + mffgpr/mftgpr
        const POWER6_EXT = 0x00000200;
        /// ISA 2.06
        const ARCH_2_06 = 0x00000100;
        /// P7 Vector Extension.
        const HAS_VSX = 0x00000080;
        const PSERIES_PERFMON_COMPAT = 0x00000040;
        const TRUE_LE = 0x00000002;
        const PPC_LE = 0x00000001;
    }
}
#[cfg(target_arch = "powerpc64")]
impl Sealed for Features {}
#[cfg(target_arch = "powerpc64")]
impl AuxValue for Features {
    fn from(value: aux_t) -> Self {
        Self::from_bits_retain(value as usize)
    }
}

#[cfg(not(target_arch = "powerpc64"))]
type Features2 = usize;

#[cfg(target_arch = "powerpc64")]
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Features2: usize {
        /// ISA 2.07
        const ARCH_2_07 = 0x80000000;
        /// Hardware Transactional Memory
        const HAS_HTM = 0x40000000;
        /// Data Stream Control Register
        const HAS_DSCR = 0x20000000;
        /// Event Base Branching
        const HAS_EBB = 0x10000000;
        /// Integer Select
        const HAS_ISEL = 0x08000000;
        /// Target Address Register
        const HAS_TAR = 0x04000000;
         /// Target supports vector instruction.
        const HAS_VEC_CRYPTO = 0x02000000;
        /// Kernel aborts transaction when a syscall is made.
        const HTM_NOSC = 0x01000000;
        /// ISA 3.0
        const ARCH_3_00 = 0x00800000;
        /// VSX IEEE Binary Float 128-bit
        const HAS_IEEE128 = 0x00400000;
        /// darn instruction.
        const DARN = 0x00200000;
        /// scv syscall.
        const SCV = 0x00100000;
        /// TM without suspended state.
        const HTM_NO_SUSPEND = 0x00080000;
        /// ISA 3.1.
        const ARCH_3_1 = 0x00040000;
        /// Matrix-Multiply Assist.
        const MMA = 0x00020000;
    }
}
#[cfg(target_arch = "powerpc64")]
impl Sealed for Features2 {}
#[cfg(target_arch = "powerpc64")]
impl AuxValue for Features2 {
    fn from(value: aux_t) -> Self {
        Self::from_bits_retain(value as usize)
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

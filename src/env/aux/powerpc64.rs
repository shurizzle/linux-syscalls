use super::{aux_t, AuxValue, Sealed};

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

    impl Sealed for Features {}
    impl AuxValue for Features {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

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

    impl Sealed for Features2 {}
    impl AuxValue for Features2 {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

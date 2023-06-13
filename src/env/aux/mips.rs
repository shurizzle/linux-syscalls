use super::{aux_t, AuxValue, Sealed};

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        R6 = 1 << 0,
        MSA = 1 << 1,
        CRC32 = 1 << 2,
        MIPS16 = 1 << 3,
        MDMX = 1 << 4,
        MIPS3D = 1 << 5,
        SMARTMIPS = 1 << 6,
        DSP = 1 << 7,
        DSP2 = 1 << 8,
        DSP3 = 1 << 9,
        MIPS16E2 = 1 << 10,
        MMI = 1 << 11,
        EXT = 1 << 12,
        EXT2 = 1 << 13,
        CPUCFG = 1 << 14,
    }

    impl Sealed for Features {}
    impl AuxValue for Features {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

pub type Features2 = usize;

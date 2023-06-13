use super::{aux_t, AuxValue, Sealed};

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        CPUCFG = 1 << 0,
        LAM = 1 << 1,
        UAL = 1 << 2,
        FPU = 1 << 3,
        LSX = 1 << 4,
        LASX = 1 << 5,
        CRC32 = 1 << 6,
        COMPLEX = 1 << 7,
        CRYPTO = 1 << 8,
        LVZ = 1 << 9,
        LBT_X86 = 1 << 10,
        LBT_ARM = 1 << 11,
        LBT_MIPS = 1 << 12,
    }

    impl Sealed for Features {}
    impl AuxValue for Features {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

pub type Features2 = usize;

use super::{aux_t, AuxValue, Sealed};

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        SWP = 1 << 0,
        HALF = 1 << 1,
        THUMB = 1 << 2,
        B26 = 1 << 3,
        FAST_MULT = 1 << 4,
        FPA = 1 << 5,
        VFP = 1 << 6,
        EDSP = 1 << 7,
        JAVA = 1 << 8,
        IWMMXT = 1 << 9,
        CRUNCH = 1 << 10,
        THUMBEE = 1 << 11,
        NEON = 1 << 12,
        #[allow(non_upper_case_globals)]
        VFPv3 = 1 << 13,
        #[allow(non_upper_case_globals)]
        VFPv3D16 = 1 << 14,
        TLS = 1 << 15,
        #[allow(non_upper_case_globals)]
        VFPv4 = 1 << 16,
        IDIVA = 1 << 17,
        IDIVT = 1 << 18,
        VFPD32 = 1 << 19,
        IDIV = Self::IDIVA.bits() | Self::IDIVT.bits(),
        LPAE = 1 << 20,
        EVTSTRM = 1 << 21,
        FPHP = 1 << 22,
        ASIMDHP = 1 << 23,
        ASIMDDP = 1 << 24,
        ASIMDFHM = 1 << 25,
        ASIMDBF16 = 1 << 26,
        I8MM = 1 << 27,
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
        AES = 1 << 0,
        PMULL = 1 << 1,
        SHA1 = 1 << 2,
        SHA2 = 1 << 3,
        CRC32 = 1 << 4,
        SB = 1 << 5,
        SSBS = 1 << 6,
    }

    impl Sealed for Features2 {}
    impl AuxValue for Features2 {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

use super::{aux_t, AuxValue, Sealed};

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        FP = 1 << 0,
        ASIMD = 1 << 1,
        EVTSTRM = 1 << 2,
        AES = 1 << 3,
        PMULL = 1 << 4,
        SHA1 = 1 << 5,
        SHA2 = 1 << 6,
        CRC32 = 1 << 7,
        ATOMICS = 1 << 8,
        FPHP = 1 << 9,
        ASIMDHP = 1 << 10,
        CPUID = 1 << 11,
        ASIMDRDM = 1 << 12,
        JSCVT = 1 << 13,
        FCMA = 1 << 14,
        LRCPC = 1 << 15,
        DCPOP = 1 << 16,
        SHA3 = 1 << 17,
        SM3 = 1 << 18,
        SM4 = 1 << 19,
        ASIMDDP = 1 << 20,
        SHA512 = 1 << 21,
        SVE = 1 << 22,
        ASIMDFHM = 1 << 23,
        DIT = 1 << 24,
        USCAT = 1 << 25,
        ILRCPC = 1 << 26,
        FLAGM = 1 << 27,
        SSBS = 1 << 28,
        SB = 1 << 29,
        PACA = 1 << 30,
        PACG = 1 << 31,
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
        DCPODP = 1 << 0,
        SVE2 = 1 << 1,
        SVEAES = 1 << 2,
        SVEPMULL = 1 << 3,
        SVEBITPERM = 1 << 4,
        SVESHA3 = 1 << 5,
        SVESM4 = 1 << 6,
        FLAGM2 = 1 << 7,
        FRINT = 1 << 8,
        SVEI8MM = 1 << 9,
        SVEF32MM = 1 << 10,
        SVEF64MM = 1 << 11,
        SVEBF16 = 1 << 12,
        I8MM = 1 << 13,
        BF16 = 1 << 14,
        DGH = 1 << 15,
        RNG = 1 << 16,
        BTI = 1 << 17,
        MTE = 1 << 18,
        ECV = 1 << 19,
        AFP = 1 << 20,
        RPRES = 1 << 21,
        MTE3 = 1 << 22,
        SME = 1 << 23,
        SME_I16I64 = 1 << 24,
        SME_F64F64 = 1 << 25,
        SME_I8I32 = 1 << 26,
        SME_F16F32 = 1 << 27,
        SME_B16F32 = 1 << 28,
        SME_F32F32 = 1 << 29,
        SME_FA64 = 1 << 30,
        WFXT = 1 << 31,
        EBF16 = 1 << 32,
        SVE_EBF16 = 1 << 33,
        CSSC = 1 << 34,
        RPRFM = 1 << 35,
        SVE2P1 = 1 << 36,
        SME2 = 1 << 37,
        SME2P1 = 1 << 38,
        SME_I16I32 = 1 << 39,
        SME_BI32I32 = 1 << 40,
        SME_B16B16 = 1 << 41,
        SME_F16F16 = 1 << 42,
    }

    impl Sealed for Features2 {}
    impl AuxValue for Features2 {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

use super::{aux_t, AuxValue, Sealed};

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        ESAN3 = 1 << 0,
        ZARCH = 1 << 1,
        STFLE = 1 << 2,
        MSA = 1 << 3,
        LDISP = 1 << 4,
        EIMM = 1 << 5,
        DFP = 1 << 6,
        HPAGE = 1 << 7,
        ETF3EH = 1 << 8,
        HIGH_GPRS = 1 << 9,
        TE = 1 << 10,
        VXRS = 1 << 11,
        VXRS_BCD = 1 << 12,
        VXRS_EXT = 1 << 13,
        GS = 1 << 14,
        VXRS_EXT2 = 1 << 15,
        VXRS_PDE = 1 << 16,
        SORT = 1 << 17,
        DFLT = 1 << 18,
        VXRS_PDE2 = 1 << 19,
        NNPA = 1 << 20,
        PCI_MIO = 1 << 21,
        SIE = 1 << 22,
    }

    impl Sealed for Features {}
    impl AuxValue for Features {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

pub type Features2 = usize;

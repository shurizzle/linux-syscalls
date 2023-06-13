use super::{aux_t, AuxValue, Sealed};

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features: usize {
        ISA_A = 1 << 0,
        ISA_C = 1 << 2,
        ISA_D = 1 << 3,
        ISA_F = 1 << 5,
        ISA_I = 1 << 8,
        ISA_M = 1 << 12,
    }

    impl Sealed for Features {}
    impl AuxValue for Features {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

pub type Features2 = usize;

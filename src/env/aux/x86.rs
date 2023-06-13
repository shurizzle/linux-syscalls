use super::{aux_t, AuxValue, Sealed};

pub type Features = usize;

crate::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Features2: usize {
        /// MONITOR/MWAIT enabled in Ring 3
        RING3MWAIT = 1 << 0,
        /// Kernel allows FSGSBASE instructions available in Ring 3
        FSGSBASE = 1 << 1,
    }

    impl Sealed for Features2 {}
    impl AuxValue for Features2 {
        fn from(value: aux_t) -> Self {
            (value as usize).into()
        }
    }
}

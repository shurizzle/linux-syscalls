#[doc(hidden)]
#[macro_export]
macro_rules! bitflags {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident: $ty:ty {
            $(
                $(#[$fmeta:meta])*
                $flag:ident = $value:expr
            ),+ $(,)?
        }
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $name($ty);

        impl $name {
            $(
                $(#[$fmeta])*
                pub const $flag: Self = Self($value);
            )+

            #[inline]
            pub const fn empty() -> Self {
                Self(0)
            }

            #[inline]
            pub const fn all() -> Self {
                Self($($value)|+)
            }

            #[inline]
            pub const fn from_bits(bits: $ty) -> Self {
                Self(bits)
            }

            #[inline]
            pub const fn bits(&self) -> $ty {
                self.0
            }

            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0 & other.bits() == other.bits()
            }

            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.0 &= !other.bits();
            }

            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.0 |= other.bits();
            }

            #[inline]
            pub fn toggle(&mut self, other: Self) {
                if self.0 & other.bits() == other.bits() {
                    self.remove(other)
                } else {
                    self.insert(other)
                }
            }
        }

        impl ::core::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.bits();
            }
        }

        impl ::core::ops::BitAndAssign<$ty> for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $ty) {
                self.0 &= rhs;
            }
        }

        impl ::core::ops::BitAnd for $name {
            type Output = Self;

            #[inline]
            fn bitand(mut self, rhs: Self) -> Self {
                self &= rhs;
                self
            }
        }

        impl ::core::ops::BitAnd<$ty> for $name {
            type Output = Self;

            #[inline]
            fn bitand(mut self, rhs: $ty) -> Self {
                self &= rhs;
                self
            }
        }

        impl ::core::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.bits();
            }
        }

        impl ::core::ops::BitOrAssign<$ty> for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $ty) {
                self.0 |= rhs;
            }
        }

        impl ::core::ops::BitOr for $name {
            type Output = Self;

            #[inline]
            fn bitor(mut self, rhs: Self) -> Self {
                self |= rhs;
                self
            }
        }

        impl ::core::ops::BitOr<$ty> for $name {
            type Output = Self;

            #[inline]
            fn bitor(mut self, rhs: $ty) -> Self {
                self |= rhs;
                self
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = $ty;

            #[inline]
            fn deref(&self) -> &$ty {
                &self.0
            }
        }

        impl ::core::borrow::Borrow<$ty> for $name {
            #[inline]
            fn borrow(&self) -> &$ty {
                self
            }
        }

        impl ::core::convert::AsRef<$ty> for $name {
            #[inline]
            fn as_ref(&self) -> &$ty {
                self
            }
        }

        impl ::core::convert::From<$ty> for $name {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from_bits(value)
            }
        }

        impl ::core::convert::From<$name> for $ty {
            #[inline]
            fn from(value: $name) -> Self {
                *value
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut value = self.0;
                let mut first = true;

                $(
                    if value & $value != 0 {
                        value &= !$value;
                        if first {
                            first = false;
                        } else {
                            ::core::fmt::Debug::fmt(" | ", f)?;
                        }
                        ::core::fmt::Debug::fmt(stringify!($name), f)?;
                    }
                )+

                if value != 0 {
                    if !first {
                        ::core::fmt::Debug::fmt(" | ", f)?;
                    }
                    ::core::fmt::Debug::fmt(&value, f)?;
                }

                ::core::result::Result::Ok(())
            }
        }

        $($rest)*
    };
}

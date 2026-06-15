// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct SaturatingU16(u16);

macro_rules! impl_from_for_saturating_u16 {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SaturatingU16 {
                fn from(value: $t) -> Self {
                    Self(value as u16)
                }
            }
            impl From<&$t> for SaturatingU16 {
                fn from(value: &$t) -> Self {
                    Self::from(*value)
                }
            }
        )*
    };
}

impl_from_for_saturating_u16!(u8, u16);

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0.saturating_add(other.0))
    }
}
impl Add<&Self> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        Self(self.0.saturating_add(other.0))
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        Self(self.0.saturating_add(other))
    }
}
impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self::Output {
        Self(self.0.saturating_add(*other))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

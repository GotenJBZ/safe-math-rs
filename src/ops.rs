use crate::error::SafeMathResult;

/// Trait that defines safe mathematical operations for a type.
#[cfg(not(feature = "derive"))]
pub trait SafeMathOps: Copy {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

#[cfg(feature = "derive")]
pub use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub};

#[cfg(feature = "derive")]
pub trait SafeAdd: Copy + CheckedAdd {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
}

#[cfg(feature = "derive")]
pub trait SafeSub: Copy + CheckedSub {
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
}

#[cfg(feature = "derive")]
pub trait SafeMul: Copy + CheckedMul {
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
}

#[cfg(feature = "derive")]
pub trait SafeDiv: Copy + CheckedDiv {
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
}

#[cfg(feature = "derive")]
pub trait SafeRem: Copy + CheckedRem {
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

#[cfg(feature = "derive")]
pub trait SafeMathOps: Copy {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

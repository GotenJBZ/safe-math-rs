use crate::error::SafeMathResult;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub};

#[diagnostic::on_unimplemented(message = "`{Self}` does not implement `SafeAdd`; \
               add `add` to `#[SafeMathOps(...)]`")]

pub trait SafeAdd: Copy + CheckedAdd + Add<Output = Self> {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
}
#[diagnostic::on_unimplemented(message = "`{Self}` does not implement `SafeSub`; \
               add `sub` to `#[SafeMathOps(...)]`")]
pub trait SafeSub: Copy + CheckedSub + Sub<Output = Self> {
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
}

#[diagnostic::on_unimplemented(message = "`{Self}` does not implement `SafeMul`; \
               add `mul` to `#[SafeMathOps(...)]`")]
pub trait SafeMul: Copy + CheckedMul + Mul<Output = Self> {
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
}

#[diagnostic::on_unimplemented(message = "`{Self}` does not implement `SafeDiv`; \
               add `div` to `#[SafeMathOps(...)]`")]
pub trait SafeDiv: Copy + CheckedDiv + Div<Output = Self> {
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
}

#[diagnostic::on_unimplemented(message = "`{Self}` does not implement `SafeRem`; \
               add `rem` to `#[SafeMathOps(...)]`")]
pub trait SafeRem: Copy + CheckedRem + Rem<Output = Self> {
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

pub trait SafeMathOps: Copy {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

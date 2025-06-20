use crate::error::SafeMathError;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub};

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe addition.",
    note = "Add `add` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeAdd: Copy + CheckedAdd + Add<Output = Self> {
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
}
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe subtraction.",
    note = "Add `sub` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeSub: Copy + CheckedSub + Sub<Output = Self> {
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
}

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe multiplication.",
    note = "Add `mul` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeMul: Copy + CheckedMul + Mul<Output = Self> {
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError>;
}

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe division.",
    note = "Add `div` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeDiv: Copy + CheckedDiv + Div<Output = Self> {
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError>;
}

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe remainder operation.",
    note = "Add `rem` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeRem: Copy + CheckedRem + Rem<Output = Self> {
    fn safe_rem(self, rhs: Self) -> Result<Self, SafeMathError>;
}

pub trait SafeMathOps: Copy {
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_rem(self, rhs: Self) -> Result<Self, SafeMathError>;
}

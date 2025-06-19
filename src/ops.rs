use crate::error::SafeMathResult;

pub use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub};

pub trait SafeAdd: Copy + CheckedAdd {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
}

pub trait SafeSub: Copy + CheckedSub {
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
}

pub trait SafeMul: Copy + CheckedMul {
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
}

pub trait SafeDiv: Copy + CheckedDiv {
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
}

pub trait SafeRem: Copy + CheckedRem {
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

pub trait SafeMathOps: Copy {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

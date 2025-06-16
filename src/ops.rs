use core::fmt::Debug;

use crate::error::SafeMathResult;

/// Trait that defines safe mathematical operations for a type.
pub trait SafeMathOps: Copy + Debug {
    fn safe_add(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_sub(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_mul(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_div(self, rhs: Self) -> SafeMathResult<Self>;
    fn safe_rem(self, rhs: Self) -> SafeMathResult<Self>;
}

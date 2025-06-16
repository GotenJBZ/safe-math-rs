use crate::error::{SafeMathError, SafeMathResult};
use crate::ops::SafeMathOps;

// === Implementations for integer types =====================================
macro_rules! impl_safe_math_int {
    ($($t:ty),*) => {
        $(
            impl SafeMathOps for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_add(rhs).ok_or(SafeMathError::Overflow)
                }
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_sub(rhs).ok_or(SafeMathError::Overflow)
                }
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_mul(rhs).ok_or(SafeMathError::Overflow)
                }
                #[inline(always)]
                fn safe_div(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_div(rhs).ok_or_else(|| {
                        if rhs == 0 {
                            SafeMathError::DivisionByZero
                        } else {
                            SafeMathError::Overflow // e.g. i32::MIN / -1
                        }
                    })
                }
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_rem(rhs).ok_or(SafeMathError::DivisionByZero)
                }
            }
        )*
    };
}

impl_safe_math_int!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
);

// === Implementations for floating-point types ==============================
macro_rules! impl_safe_math_float {
    ($($t:ty),*) => {
        $(
            impl SafeMathOps for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> SafeMathResult<Self> { Ok(self + rhs) }
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> SafeMathResult<Self> { Ok(self - rhs) }
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> SafeMathResult<Self> { Ok(self * rhs) }
                #[inline(always)]
                fn safe_div(self, rhs: Self) -> SafeMathResult<Self> { Ok(self / rhs) }
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> SafeMathResult<Self> { Ok(self % rhs) }
            }
        )*
    };
}

impl_safe_math_float!(f32, f64);

// === Crate-internal generic helpers ========================================

#[inline(always)]
pub fn safe_add<T: SafeMathOps>(a: T, b: T) -> SafeMathResult<T> {
    a.safe_add(b)
}
#[inline(always)]
pub fn safe_sub<T: SafeMathOps>(a: T, b: T) -> SafeMathResult<T> {
    a.safe_sub(b)
}
#[inline(always)]
pub fn safe_mul<T: SafeMathOps>(a: T, b: T) -> SafeMathResult<T> {
    a.safe_mul(b)
}
#[inline(always)]
pub fn safe_div<T: SafeMathOps>(a: T, b: T) -> SafeMathResult<T> {
    a.safe_div(b)
}
#[inline(always)]
pub fn safe_rem<T: SafeMathOps>(a: T, b: T) -> SafeMathResult<T> {
    a.safe_rem(b)
}

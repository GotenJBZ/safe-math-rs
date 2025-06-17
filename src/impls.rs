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

impl_safe_math_int!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

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

macro_rules! impl_safe_math_ops {
    ($($op:ident),*) => {
        $(
            #[inline(always)]
            pub fn $op<T: SafeMathOps>(a: T, b: T) -> SafeMathResult<T> {
                a.$op(b)
            }
        )*
    };
}

impl_safe_math_ops!(safe_add, safe_sub, safe_mul, safe_div, safe_rem);

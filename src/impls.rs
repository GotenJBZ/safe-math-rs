use crate::error::{SafeMathError, SafeMathResult};
use crate::ops::{SafeAdd, SafeDiv, SafeMathOps, SafeMul, SafeRem, SafeSub};
macro_rules! impl_safe_math_ops {
    ($($op:ident, $trait:ident),*) => {
        $(
            #[inline(always)]
            pub fn $op<T: $trait>(a: T, b: T) -> SafeMathResult<T> {
                a.$op(b)
            }
        )*
    };
}

impl_safe_math_ops!(
    safe_add, SafeAdd, safe_sub, SafeSub, safe_mul, SafeMul, safe_div, SafeDiv, safe_rem, SafeRem
);

// === Implementations for when derive feature is enabled ====================

macro_rules! impl_safe_math_int {
    ($($t:ty),*) => {
        $(
            impl SafeAdd for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_add(rhs).ok_or(SafeMathError::Overflow)
                }
            }

            impl SafeSub for $t {
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_sub(rhs).ok_or(SafeMathError::Overflow)
                }
            }

            impl SafeMul for $t {
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_mul(rhs).ok_or(SafeMathError::Overflow)
                }
            }

            impl SafeDiv for $t {
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
            }

            impl SafeRem for $t {
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> SafeMathResult<Self> {
                    self.checked_rem(rhs).ok_or(SafeMathError::DivisionByZero)
                }
            }

            impl SafeMathOps for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> SafeMathResult<Self> {
                    <Self as SafeAdd>::safe_add(self, rhs)
                }
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> SafeMathResult<Self> {
                    <Self as SafeSub>::safe_sub(self, rhs)
                }
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> SafeMathResult<Self> {
                    <Self as SafeMul>::safe_mul(self, rhs)
                }
                #[inline(always)]
                fn safe_div(self, rhs: Self) -> SafeMathResult<Self> {
                    <Self as SafeDiv>::safe_div(self, rhs)
                }
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> SafeMathResult<Self> {
                    <Self as SafeRem>::safe_rem(self, rhs)
                }
            }
        )*
    };
}

impl_safe_math_int!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

macro_rules! impl_safe_math_float {
    ($($t:ty),*) => {
        $(
            impl SafeMathOps for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> SafeMathResult<Self> {
                    Some(self + rhs).filter(|x| x.is_finite()).ok_or(SafeMathError::Overflow)
                }
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> SafeMathResult<Self> {
                    Some(self - rhs).filter(|x| x.is_finite()).ok_or(SafeMathError::Overflow)
                }
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> SafeMathResult<Self> {
                    Some(self * rhs).filter(|x| x.is_finite()).ok_or(SafeMathError::Overflow)
                }
                #[inline(always)]
                fn safe_div(self, rhs: Self) -> SafeMathResult<Self> {
                    Some(self / rhs).filter(|x| x.is_finite()).ok_or(SafeMathError::DivisionByZero)
                }
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> SafeMathResult<Self> {
                    Some(self % rhs).filter(|x| x.is_finite()).ok_or(SafeMathError::DivisionByZero)
                }
            }
        )*
    };
}

impl_safe_math_float!(f32, f64);

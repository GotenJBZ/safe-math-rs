//! Implementation of safe arithmetic operations for built-in numeric types.
//!
//! This module provides the core implementations of safe arithmetic operations
//! for all supported numeric types. It includes:
//!
//! - Helper functions used by the `#[safe_math]` macro
//! - Trait implementations for integer types using checked operations
//! - Specialized implementations for floating-point types

use crate::error::SafeMathError;
use crate::ops::{SafeAdd, SafeDiv, SafeMathOps, SafeMul, SafeRem, SafeSub};
use sealed::{IsSafeAdd, IsSafeDiv, IsSafeMul, IsSafeRem, IsSafeSub};

macro_rules! doc_for_trait {
    (SafeDiv) => {
        "`Ok(result)` on success, `Err(SafeMathError::DivisionByZero)` on error."
    };
    (SafeRem) => {
        "`Ok(result)` on success, `Err(SafeMathError::DivisionByZero)` on error."
    };
    ($trait:ident) => {
        "`Ok(result)` on success, `Err(SafeMathError::Overflow)` on error."
    };
}

macro_rules! impl_safe_math_ops {
    (
        $(
            $op:ident => {
                trait: $trait:ident,
                desc: $desc:expr
            }
        ),* $(,)?
    ) => {
        $(
            #[doc = concat!("Performs safe ", $desc," checking.")]
            ///
            /// Used internally by the `#[safe_math]` macro during expansion.
            /// This function delegates to the appropriate trait method.
            ///
            /// # Arguments
            ///
            /// * `a` - First operand.
            /// * `b` - Second operand.
            ///
            /// # Returns
            ///
            #[doc = doc_for_trait!($trait)]
            #[inline(always)]
            pub fn $op<T: $trait>(a: T, b: T) -> Result<T, SafeMathError> {
                a.$op(b)
            }
        )*
    };
}

impl_safe_math_ops!(
    safe_add => {
        trait: SafeAdd,
        desc: "addition with overflow"
    },
    safe_sub => {
        trait: SafeSub,
        desc: "subtraction with underflow"
    },
    safe_mul => {
        trait: SafeMul,
        desc: "multiplication with overflow"
    },
    safe_div => {
        trait: SafeDiv,
        desc: "division with division-by-zero"
    },
    safe_rem => {
        trait: SafeRem,
        desc: "remainder with division-by-zero"
    }
);

macro_rules! impl_safe_ops {
    (
        $(
            ($trait_name:ident, $trait_name_str:ident, $method_name:ident, $checked_method:ident, $bound:ident, $err:expr)
        ),* $(,)?
    ) => {
        $(
            #[diagnostic::do_not_recommend]
            impl<T> $trait_name for T
            where
                T: $bound + std::ops::$trait_name_str<Output = T> + Copy,
            {
                #[inline(always)]
                fn $method_name(self, rhs: T) -> Result<T, SafeMathError> {
                    self.$checked_method(&rhs).ok_or($err)
                }
            }
        )*
    };
}

impl_safe_ops!(
    (
        SafeAdd,
        Add,
        safe_add,
        checked_add,
        IsSafeAdd,
        SafeMathError::Overflow
    ),
    (
        SafeSub,
        Sub,
        safe_sub,
        checked_sub,
        IsSafeSub,
        SafeMathError::Overflow
    ),
    (
        SafeMul,
        Mul,
        safe_mul,
        checked_mul,
        IsSafeMul,
        SafeMathError::Overflow
    ),
    (
        SafeDiv,
        Div,
        safe_div,
        checked_div,
        IsSafeDiv,
        SafeMathError::DivisionByZero
    ),
    (
        SafeRem,
        Rem,
        safe_rem,
        checked_rem,
        IsSafeRem,
        SafeMathError::DivisionByZero
    ),
);

macro_rules! impl_safe_float_ops {
    ($($trait:ident, $method:ident, $op:tt),*) => {
        $(
            #[diagnostic::do_not_recommend]
            impl $trait for f32 {
                #[doc = concat!("Performs safe ", stringify!($method), " for f32.")]
                ///
                /// Used internally by the `#[safe_math]` macro during expansion.
                /// Checks for finite results to prevent infinity/NaN propagation.
                ///
                /// # Arguments
                ///
                /// * `self` - First operand.
                /// * `rhs` - Second operand.
                ///
                /// # Returns
                ///
                /// `Ok(result)` on success, `Err(SafeMathError::InfiniteOrNaN)` on error.
                #[inline(always)]
                fn $method(self, rhs: Self) -> Result<Self, SafeMathError> {
                    let res = self $op rhs;
                    res.is_finite().then(|| res).ok_or(SafeMathError::InfiniteOrNaN)
                }
            }

            #[diagnostic::do_not_recommend]
            impl $trait for f64 {
                #[doc = concat!("Performs safe ", stringify!($method), " for f64.")]
                ///
                /// Used internally by the `#[safe_math]` macro during expansion.
                /// Checks for finite results to prevent infinity/NaN propagation.
                ///
                /// # Arguments
                ///
                /// * `self` - First operand.
                /// * `rhs` - Second operand.
                ///
                /// # Returns
                ///
                /// `Ok(result)` on success, `Err(SafeMathError::InfiniteOrNaN)` on error.
                #[inline(always)]
                fn $method(self, rhs: Self) -> Result<Self, SafeMathError> {
                    let res = self $op rhs;
                    res.is_finite().then(|| res).ok_or(SafeMathError::InfiniteOrNaN)
                }
            }
        )*
    };
}

impl_safe_float_ops!(
    SafeAdd, safe_add, +,
    SafeSub, safe_sub, -,
    SafeMul, safe_mul, *,
    SafeDiv, safe_div, /,
    SafeRem, safe_rem, %
);

#[diagnostic::do_not_recommend]
impl<T> SafeMathOps for T
where
    T: SafeAdd + SafeSub + SafeMul + SafeDiv + SafeRem + Copy,
{
    #[inline(always)]
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeAdd>::safe_add(self, rhs)
    }
    #[inline(always)]
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeSub>::safe_sub(self, rhs)
    }
    #[inline(always)]
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeMul>::safe_mul(self, rhs)
    }
    #[inline(always)]
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeDiv>::safe_div(self, rhs)
    }
    #[inline(always)]
    fn safe_rem(self, rhs: Self) -> Result<Self, SafeMathError> {
        <Self as SafeRem>::safe_rem(self, rhs)
    }
}

mod sealed {
    use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub};
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($($t:ty),*) => {
            $(
                impl Sealed for $t {}
            )*
        };
    }

    impl_sealed!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

    pub trait IsSafeAdd: Sealed + CheckedAdd {}
    pub trait IsSafeSub: Sealed + CheckedSub {}
    pub trait IsSafeMul: Sealed + CheckedMul {}
    pub trait IsSafeDiv: Sealed + CheckedDiv {}
    pub trait IsSafeRem: Sealed + CheckedRem {}

    impl<T> IsSafeAdd for T where T: Sealed + CheckedAdd {}
    impl<T> IsSafeSub for T where T: Sealed + CheckedSub {}
    impl<T> IsSafeMul for T where T: Sealed + CheckedMul {}
    impl<T> IsSafeDiv for T where T: Sealed + CheckedDiv {}
    impl<T> IsSafeRem for T where T: Sealed + CheckedRem {}
}

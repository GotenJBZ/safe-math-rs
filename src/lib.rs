//! # Safe Math Library
//!
//! A Rust library for safe mathematical operations that automatically prevent overflow
//! using the `#[safe_math]` macro.
//!
//! ## Basic Usage
//!
//! ```rust
//! use safe_math::safe_math;
//!
//! #[safe_math]
//! fn calculate(a: u8, b: u8) -> Result<u8, ()> {
//!     let result = a + b; // Automatically uses checked_add
//!     Ok(result)
//! }
//!
//! // Usage
//! assert_eq!(calculate(10, 20), Ok(30));
//! assert!(calculate(255, 1).is_err()); // Overflow!
//! ```
//!
//! ## Supported Operations
//!
//! - Addition (`+`)
//! - Subtraction (`-`)
//! - Multiplication (`*`)
//! - Division (`/`)
//! - Remainder (`%`)
//! - Assignment operators (`+=`, `-=`, `*=`, `/=`, `%=`)

use std::fmt::Debug;

// Re-export the macro from safe_math_macros
pub use safe_math_macros::safe_math;

/// Trait that defines safe mathematical operations
#[doc(hidden)]
pub trait SafeMathOps: Copy + Debug {
    fn safe_add(self, rhs: Self) -> Result<Self, ()>;
    fn safe_sub(self, rhs: Self) -> Result<Self, ()>;
    fn safe_mul(self, rhs: Self) -> Result<Self, ()>;
    fn safe_div(self, rhs: Self) -> Result<Self, ()>;
    fn safe_rem(self, rhs: Self) -> Result<Self, ()>;
}

/// Implementation of `SafeMathOps` for integer types using checked operations
macro_rules! impl_safe_math_int {
    ($($t:ty),*) => {
        $(
            impl SafeMathOps for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> Result<Self, ()> {
                    self.checked_add(rhs).ok_or(())
                }
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> Result<Self, ()> {
                    self.checked_sub(rhs).ok_or(())
                }
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> Result<Self, ()> {
                    self.checked_mul(rhs).ok_or(())
                }
                #[inline(always)]
                fn safe_div(self, rhs: Self) -> Result<Self, ()> {
                    self.checked_div(rhs).ok_or(())
                }
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> Result<Self, ()> {
                    self.checked_rem(rhs).ok_or(())
                }
            }
        )*
    };
}

impl_safe_math_int!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
);

/// Implement `SafeMathOps` for floating-point types without overflow checks.
/// This is a workaround to allow safe-math in functions that mix signed/unsigned and floating-point numbers.
/// In the future, additional checks (e.g., not NaN, not Inf) could be added, but for now we simply map 1:1 to checked operations.
macro_rules! impl_safe_math_float {
    ($($t:ty),*) => {
        $(
            impl SafeMathOps for $t {
                #[inline(always)]
                fn safe_add(self, rhs: Self) -> Result<Self, ()> {
                    Ok(self + rhs)
                }
                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> Result<Self, ()> {
                    Ok(self - rhs)
                }
                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> Result<Self, ()> {
                    Ok(self * rhs)
                }
                #[inline(always)]
                fn safe_div(self, rhs: Self) -> Result<Self, ()> {
                    Ok(self / rhs)
                }
                #[inline(always)]
                fn safe_rem(self, rhs: Self) -> Result<Self, ()> {
                    Ok(self % rhs)
                }
            }
        )*
    };
}

impl_safe_math_float!(f32, f64);

/// Generic safe math functions
#[doc(hidden)]
#[inline(always)]
pub fn safe_add<T: SafeMathOps>(a: T, b: T) -> Result<T, ()> {
    a.safe_add(b)
}

#[doc(hidden)]
#[inline(always)]
pub fn safe_sub<T: SafeMathOps>(a: T, b: T) -> Result<T, ()> {
    a.safe_sub(b)
}

#[doc(hidden)]
#[inline(always)]
pub fn safe_mul<T: SafeMathOps>(a: T, b: T) -> Result<T, ()> {
    a.safe_mul(b)
}

#[doc(hidden)]
#[inline(always)]
pub fn safe_div<T: SafeMathOps>(a: T, b: T) -> Result<T, ()> {
    a.safe_div(b)
}

#[doc(hidden)]
#[inline(always)]
pub fn safe_rem<T: SafeMathOps>(a: T, b: T) -> Result<T, ()> {
    a.safe_rem(b)
}

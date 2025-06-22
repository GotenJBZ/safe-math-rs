use crate::error::SafeMathError;
use std::ops::{Add, Div, Mul, Rem, Sub};

/// Safe addition operation with overflow checking.
///
/// This trait provides checked addition that returns a `Result` instead of panicking
/// or wrapping on overflow.
///
/// # Arguments
///
/// * `rhs` - Right-hand side operand.
///
/// # Returns
///
/// * `Ok(result)` - The sum of `self` and `rhs` if no overflow occurred
/// * `Err(SafeMathError::Overflow)` - If the addition would overflow
///
/// # Examples
///
/// ```rust
/// use safe_math::{SafeAdd, SafeMathError};
///
/// let a: u8 = 250;
/// let b: u8 = 5;
///
/// // Safe addition that works
/// assert_eq!(a.safe_add(b), Ok(255));
///
/// // Safe addition that detects overflow
/// let c: u8 = 251;
/// assert_eq!(a.safe_add(c), Err(SafeMathError::Overflow));
/// ```
///
/// # See also
///
/// * [`SafeMathOps`] - Combined trait for all safe arithmetic operations
/// * [`SafeMathError`] - Error type returned on arithmetic failures
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe addition.",
    note = "Add `add` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeAdd: Copy + Add<Output = Self> {
    /// Performs safe addition with overflow checking.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side operand.
    ///
    /// # Returns
    ///
    /// * `Ok(result)` - The sum of `self` and `rhs` if no overflow occurred
    /// * `Err(SafeMathError::Overflow)` - If the addition would overflow
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
}

/// Safe subtraction operation with underflow checking.
///
/// This trait provides checked subtraction that returns a `Result` instead of panicking
/// or wrapping on underflow.
///
/// # Arguments
///
/// * `rhs` - Right-hand side operand.
///
/// # Returns
///
/// * `Ok(result)` - The difference of `self` and `rhs` if no underflow occurred
/// * `Err(SafeMathError::Overflow)` - If the subtraction would underflow
///
/// # Examples
///
/// ```rust
/// use safe_math::{SafeSub, SafeMathError};
///
/// let a: u8 = 5;
/// let b: u8 = 3;
///
/// // Safe subtraction that works
/// assert_eq!(a.safe_sub(b), Ok(2));
///
/// // Safe subtraction that detects underflow
/// let c: u8 = 10;
/// assert_eq!(a.safe_sub(c), Err(SafeMathError::Overflow));
/// ```
///
/// # See also
///
/// * [`SafeMathOps`] - Combined trait for all safe arithmetic operations
/// * [`SafeMathError`] - Error type returned on arithmetic failures
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe subtraction.",
    note = "Add `sub` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeSub: Copy + Sub<Output = Self> {
    /// Performs safe subtraction with underflow checking.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side operand.
    ///
    /// # Returns
    ///
    /// * `Ok(result)` - The difference of `self` and `rhs` if no underflow occurred
    /// * `Err(SafeMathError::Overflow)` - If the subtraction would underflow
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
}

/// Safe multiplication operation with overflow checking.
///
/// This trait provides checked multiplication that returns a `Result` instead of panicking
/// or wrapping on overflow.
///
/// # Arguments
///
/// * `rhs` - Right-hand side operand.
///
/// # Returns
///
/// * `Ok(result)` - The product of `self` and `rhs` if no overflow occurred
/// * `Err(SafeMathError::Overflow)` - If the multiplication would overflow
///
/// # Examples
///
/// ```rust
/// use safe_math::{SafeMul, SafeMathError};
///
/// let a: u8 = 10;
/// let b: u8 = 5;
///
/// // Safe multiplication that works
/// assert_eq!(a.safe_mul(b), Ok(50));
///
/// // Safe multiplication that detects overflow
/// let c: u8 = 100;
/// assert_eq!(a.safe_mul(c), Err(SafeMathError::Overflow));
/// ```
///
/// # See also
///
/// * [`SafeMathOps`] - Combined trait for all safe arithmetic operations
/// * [`SafeMathError`] - Error type returned on arithmetic failures
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe multiplication.",
    note = "Add `mul` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeMul: Copy + Mul<Output = Self> {
    /// Performs safe multiplication with overflow checking.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side operand.
    ///
    /// # Returns
    ///
    /// * `Ok(result)` - The product of `self` and `rhs` if no overflow occurred
    /// * `Err(SafeMathError::Overflow)` - If the multiplication would overflow
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError>;
}

/// Safe division operation with division-by-zero checking.
///
/// This trait provides checked division that returns a `Result` instead of panicking
/// when attempting to divide by zero. It also handles potential overflow cases
/// for signed integer division.
///
/// # Arguments
///
/// * `rhs` - Right-hand side operand (divisor).
///
/// # Returns
///
/// * `Ok(result)` - The quotient of `self` divided by `rhs` if division is valid
/// * `Err(SafeMathError::DivisionByZero)` - If `rhs` is zero
/// * `Err(SafeMathError::Overflow)` - If the division would overflow (e.g., MIN/-1 for signed integers)
///
/// # Examples
///
/// ```rust
/// use safe_math::{SafeDiv, SafeMathError};
///
/// let a: u8 = 10;
/// let b: u8 = 2;
///
/// // Safe division that works
/// assert_eq!(a.safe_div(b), Ok(5));
///
/// // Safe division that detects division by zero
/// let zero: u8 = 0;
/// assert_eq!(a.safe_div(zero), Err(SafeMathError::DivisionByZero));
///
/// ```
///
/// # See also
///
/// * [`SafeMathOps`] - Combined trait for all safe arithmetic operations
/// * [`SafeMathError`] - Error type returned on arithmetic failures
/// * [`SafeRem`] - Safe remainder operations
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe division.",
    note = "Add `div` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeDiv: Copy + Div<Output = Self> {
    /// Performs safe division with division-by-zero checking.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side operand (divisor).
    ///
    /// # Returns
    ///
    /// * `Ok(result)` - The quotient of `self` divided by `rhs` if division is valid
    /// * `Err(SafeMathError::DivisionByZero)` - If `rhs` is zero
    /// * `Err(SafeMathError::Overflow)` - If the division would overflow
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError>;
}

/// Safe remainder operation with division-by-zero checking.
///
/// This trait provides checked remainder (modulo) operations that return a `Result`
/// instead of panicking when attempting to compute remainder with a zero divisor.
/// The operation follows the same rules as Rust's `%` operator for the sign of the result.
///
/// # Arguments
///
/// * `rhs` - Right-hand side operand (divisor).
///
/// # Returns
///
/// * `Ok(result)` - The remainder of `self` divided by `rhs` if operation is valid
/// * `Err(SafeMathError::DivisionByZero)` - If `rhs` is zero
///
/// # Examples
///
/// ```rust
/// use safe_math::{SafeRem, SafeMathError};
///
/// let a: i8 = 10;
/// let b: i8 = 3;
///
/// // Safe remainder that works
/// assert_eq!(a.safe_rem(b), Ok(1));  // 10 % 3 = 1
///
/// // Safe remainder that detects division by zero
/// let zero: i8 = 0;
/// assert_eq!(a.safe_rem(zero), Err(SafeMathError::DivisionByZero));
///
/// // Example with negative numbers
/// let neg_a: i8 = -10;
/// assert_eq!(neg_a.safe_rem(b), Ok(-1));  // -10 % 3 = -1
/// ```
///
/// # See also
///
/// * [`SafeMathOps`] - Combined trait for all safe arithmetic operations
/// * [`SafeMathError`] - Error type returned on arithmetic failures
/// * [`SafeDiv`] - Safe division operations
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot perform safe remainder operation.",
    note = "Add `rem` to `#[SafeMathOps(...)]` when deriving `SafeMathOps`."
)]
pub trait SafeRem: Copy + Rem<Output = Self> {
    /// Performs safe remainder with division-by-zero checking.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side operand (divisor).
    ///
    /// # Returns
    ///
    /// * `Ok(result)` - The remainder of `self` divided by `rhs` if operation is valid
    /// * `Err(SafeMathError::DivisionByZero)` - If `rhs` is zero
    fn safe_rem(self, rhs: Self) -> Result<Self, SafeMathError>;
}

/// Unified trait providing all safe arithmetic operations.
///
/// This trait combines all individual safe operation traits for convenience.
/// Types implementing this trait can perform all basic arithmetic operations
/// with overflow/underflow and division-by-zero protection.
pub trait SafeMathOps: Copy {
    /// Safe addition with overflow checking.
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
    /// Safe subtraction with underflow checking.
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
    /// Safe multiplication with overflow checking.
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError>;
    /// Safe division with division-by-zero checking.
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError>;
    /// Safe remainder with division-by-zero checking.
    fn safe_rem(self, rhs: Self) -> Result<Self, SafeMathError>;
}

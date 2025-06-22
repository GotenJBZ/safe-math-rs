use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Error types returned by safe arithmetic operations.
///
/// This enum represents all possible error conditions that can occur during
/// arithmetic operations.
///
/// # Examples
///
/// ```rust
/// use safe_math::SafeMathError;
///
/// // Example of handling different error types
/// fn handle_result(result: Result<u8, SafeMathError>) {
///     match result {
///         Ok(value) => println!("Result: {}", value),
///         Err(SafeMathError::Overflow) => println!("Overflow occurred"),
///         Err(SafeMathError::DivisionByZero) => println!("Division by zero"),
///         Err(SafeMathError::InfiniteOrNaN) => println!("Infinite or NaN result"),
///         #[cfg(feature = "derive")]
///         Err(SafeMathError::NotImplemented) => println!("Operation not implemented"),
///     }
/// }
/// ```
///
/// # Features
///
/// The `NotImplemented` variant is only available when the `derive` feature is enabled.
pub enum SafeMathError {
    /// Arithmetic overflow or underflow occurred.
    Overflow,
    /// Division or remainder operation by zero.
    DivisionByZero,
    /// Operation resulted in infinite or NaN value (floating-point types).
    InfiniteOrNaN,

    #[cfg(feature = "derive")]
    /// Operation not implemented for the given type.
    ///
    /// This variant is only available when the `derive` feature is enabled.
    /// It occurs when using `#[derive(SafeMathOps)]` on types that don't
    /// implement the required checked arithmetic operations.
    NotImplemented,
}

impl fmt::Display for SafeMathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafeMathError::Overflow => write!(f, "arithmetic overflow"),
            SafeMathError::DivisionByZero => write!(f, "division by zero"),
            SafeMathError::InfiniteOrNaN => write!(f, "infinite or NaN value"),
            #[cfg(feature = "derive")]
            SafeMathError::NotImplemented => write!(f, "operation not implemented"),
        }
    }
}

impl std::error::Error for SafeMathError {}

// Allow seamless `?` propagation into functions that still use `Result<_, ()>`.
impl From<SafeMathError> for () {
    fn from(_: SafeMathError) -> Self {}
}

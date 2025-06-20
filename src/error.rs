use core::fmt;

/// Error types returned by the safe-math helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafeMathError {
    /// The operation resulted in an arithmetic overflow/underflow.
    Overflow,
    /// Attempted to divide (or take remainder) by zero.
    DivisionByZero,

    #[cfg(feature = "derive")]
    /// The operation is not implemented for the given type.
    NotImplemented,
}

impl fmt::Display for SafeMathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafeMathError::Overflow => write!(f, "arithmetic overflow"),
            SafeMathError::DivisionByZero => write!(f, "division by zero"),
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

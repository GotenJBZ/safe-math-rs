//! # Safe Math Library
//!
//! A Rust library for safe mathematical operations that automatically prevent overflow
//! using the `#[safe_math]` macro.
//!
//! ## Basic Usage
//!
//! ```rust,ignore
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

// ---------------------------------------------------------------------------
// Public interface
// ---------------------------------------------------------------------------

// Re-export the procedural macro so users can simply `use safe_math::safe_math`.
#[cfg(feature = "derive")]
pub use safe_math_macros::SafeMathOps;
pub use safe_math_macros::{safe_math, safe_math_block};

// Internal modules
mod error;
mod impls;
mod ops;

// Re-export the most relevant items at the crate root for a clean API.
pub use error::{SafeMathError, SafeMathResult};
pub use ops::{SafeAdd, SafeDiv, SafeMathOps, SafeMul, SafeRem, SafeSub};

// These helper functions are intentionally re-exported because the macro expands
// to them, and users may want to call them directly in generic contexts.
pub use impls::{safe_add, safe_div, safe_mul, safe_rem, safe_sub};

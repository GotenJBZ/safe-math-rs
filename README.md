# safe-math

[![Crates.io](https://img.shields.io/crates/v/safe-math.svg)](https://crates.io/crates/safe-math)
[![Documentation](https://docs.rs/safe-math/badge.svg)](https://docs.rs/safe-math)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md)

A procedural macro-based library that transforms standard arithmetic operations into their checked equivalents at compile time, preventing overflow, underflow, and division by zero errors.

## Overview

`safe-math` provides:
- Compile-time transformation of arithmetic operations into checked variants without runtime overhead
- Comprehensive error handling via `Result` types
- Support for custom types through derive macros

## Core Functionality

### Basic Operations

The `#[safe_math]` attribute transforms arithmetic operations into their checked equivalents:

```rust
use safe_math::safe_math;

#[safe_math]
fn add(a: u8, b: u8) -> Result<u8, safe_math::SafeMathError> {
   Ok(a + b)  // Automatically uses checked addition
}

assert_eq!(add(10, 20), Ok(30));
assert_eq!(add(255, 1), Err(safe_math::SafeMathError::Overflow));
```

### Supported Operations

All basic arithmetic operations are supported:
- Addition (`+`, `+=`)
- Subtraction (`-`, `-=`)
- Multiplication (`*`, `*=`)
- Division (`/`, `/=`)
- Remainder (`%`, `%=`)

### Error Handling

Operations return `SafeMathError` for exceptional cases:
```rust
pub enum SafeMathError {
   Overflow,           // Result exceeds type bounds
   DivisionByZero,    // Division or remainder by zero
   NotImplemented,    // Missing trait implementation (derive feature)
}
```

### Type Support

Built-in support for:
- Unsigned integers: `u8` through `u128`, `usize`
- Signed integers: `i8` through `i128`, `isize`
- Floating point: `f32`, `f64` (with infinity/NaN handling)

## Advanced Usage

### Custom Types

Enable the `derive` feature to implement safe arithmetic for custom types:

```rust
use safe_math::SafeMathOps;

#[derive(SafeMathOps)]
#[SafeMathOps(add, sub, mul, div, rem)]
struct MyNumber(u32);

#[safe_math]
fn calculate(a: MyNumber, b: MyNumber) -> Result<MyNumber, safe_math::SafeMathError> {
   Ok(a + b)
}
```

### Block-Level Safety

Use `safe_math_block!` to apply checked operations to a specific block of code:

```rust
use safe_math::safe_math_block;

fn process_numbers(a: u32, b: u32, c: u32) -> Result<u32, safe_math::SafeMathError> {
   // Only this block uses checked arithmetic
   let result = safe_math_block!({
       let product = a * b;
       let sum = product + c;
       sum / b
   });
   Ok(result)
}
```

This is useful when you want to:
- Apply safe arithmetic to specific expression
- Mix checked and unchecked operations in the same function

## Roadmap

Planned upcoming features:

- **Option-returning functions**
 Support for functions that return `Option<T>` instead of `Result<T, SafeMathError>`.

- **Crate-level macro support**
 Ability to apply `#[safe_math]` to the entire crate with a single attribute:

```rust
// main.rs or lib.rs
#![safe_math]

fn demo(a: u32, b: u32) -> Result<u32, safe_math::SafeMathError> {
   Ok(a * b + 1)
}
```

## License

Licensed under either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you shall be dual licensed as above, without any
additional terms or conditions.

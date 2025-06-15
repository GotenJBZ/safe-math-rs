# safe-math-rs

## Overview

`safe-math-rs` is a Rust library that provides safe mathematical operations using the `#[safe_math]` procedural macro. It automatically prevents overflow and underflow by converting standard arithmetic operations into their checked counterparts.

## Features

- Supports all basic arithmetic operations: addition, subtraction, multiplication, division, and remainder.
- Provides a simple macro-based API to ensure safety without boilerplate.

## Usage

Add `safe-math-rs` to your `Cargo.toml`:

```toml
[dependencies]
safe-math-rs = "0.1.0"
```

Use the `#[safe_math]` macro in your functions:

```rust
use safe_math_rs::safe_math;

#[safe_math]
fn calculate(a: u8, b: u8) -> Result<u8, ()> {
   Ok(a + b)
}

assert_eq!(calculate(10, 20), Ok(30));
assert!(calculate(255, 1).is_err()); // Overflow!
```

## Examples

See the `examples/` directory for more usage examples.

## How it Works

Under the hood, the `#[safe_math]` macro transforms your arithmetic operations into their checked counterparts. For example, a function like:

```rust
#[safe_math]
fn add(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a + b)
}
```

is transformed into:

```rust
fn add(a: u8, b: u8) -> Result<u8, ()> {
    Ok(safe_math_rs::safe_add(a, b)?)
}
```

where `safe_add` is defined as:

```rust
fn safe_add(self, rhs: Self) -> Result<Self, ()> {
    self.checked_add(rhs).ok_or(())
}
```

This ensures that any overflow or underflow results in an error, preventing unexpected behavior in your applications.
use safe_math::*;

mod common;
use common::{test_add_macro, test_div_macro, test_mul_macro, test_sub_macro};

#[test]
fn test_safe_math_macro() {
    // Test addition
    assert_eq!(test_add_macro(10u8, 20u8), Ok(30u8));
    assert!(test_add_macro(255u8, 1u8).is_err()); // Overflow test

    // Test subtraction
    assert_eq!(test_sub_macro(30u8, 10u8), Ok(20u8));
    assert!(test_sub_macro(10u8, 20u8).is_err()); // Underflow test

    // Test multiplication
    assert_eq!(test_mul_macro(5u8, 6u8), Ok(30u8));
    assert!(test_mul_macro(255u8, 2u8).is_err()); // Overflow test

    // Test division
    assert_eq!(test_div_macro(30u8, 6u8), Ok(5u8));
    assert!(test_div_macro(10u8, 0u8).is_err()); // Division by zero test
}

#[test]
fn test_safe_functions_directly() {
    // Test public functions directly
    assert_eq!(safe_add(10u8, 20u8), Ok(30u8));
    assert!(safe_add(255u8, 1u8).is_err());

    assert_eq!(safe_sub(30u8, 10u8), Ok(20u8));
    assert!(safe_sub(10u8, 20u8).is_err());

    assert_eq!(safe_mul(5u8, 6u8), Ok(30u8));
    assert!(safe_mul(255u8, 2u8).is_err());

    assert_eq!(safe_div(30u8, 6u8), Ok(5u8));
    assert!(safe_div(10u8, 0u8).is_err());
}

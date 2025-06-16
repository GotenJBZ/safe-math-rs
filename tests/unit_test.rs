use safe_math::*;

mod common;
use common::{test_add_macro, test_div_macro, test_mul_macro, test_sub_macro};

#[test]
fn test_safe_math_macro() {
    // Test addition
    assert_eq!(test_add_macro(10, 20), Ok(30));
    assert!(test_add_macro(255, 1).is_err()); // Overflow test

    // Test subtraction
    assert_eq!(test_sub_macro(30, 10), Ok(20));
    assert!(test_sub_macro(10, 20).is_err()); // Underflow test

    // Test multiplication
    assert_eq!(test_mul_macro(5, 6), Ok(30));
    assert!(test_mul_macro(255, 2).is_err()); // Overflow test

    // Test division
    assert_eq!(test_div_macro(30, 6), Ok(5));
    assert!(test_div_macro(10, 0).is_err()); // Division by zero test
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

#[test]
fn test_complex_assignments() {
    use std::cell::RefCell;

    #[safe_math]
    fn test_array_assignment(n: u8) -> Result<[u8; 1], ()> {
        let mut arr = [254u8];
        arr[0] += n;
        Ok(arr)
    }

    #[safe_math]
    fn test_complex_expr(n: u8) -> Result<(), ()> {
        let array = RefCell::new([254u8]);
        array.borrow_mut()[0] += n;
        Ok(())
    }

    // Test array indexing
    assert!(test_array_assignment(2).is_err()); // Should fail due to overflow in arr[1]
    assert_eq!(test_array_assignment(1), Ok([255u8]));

    // Test complex expression with RefCell
    assert!(test_complex_expr(2).is_err()); // Should fail due to overflow
    assert_eq!(test_complex_expr(1), Ok(()));
}

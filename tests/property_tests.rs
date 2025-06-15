use proptest::prelude::*;

mod common;
use common::{Expression, test_add_macro, test_div_macro, test_mul_macro, test_sub_macro};

proptest! {

    #[test]
    fn test_add_macro_property(a: u8, b: u8) {
        prop_assert_eq!(test_add_macro(a, b), a.checked_add(b).ok_or(()));
    }

    #[test]
    fn test_sub_macro_property(a: u8, b: u8) {
        prop_assert_eq!(test_sub_macro(a, b), a.checked_sub(b).ok_or(()));
    }

    #[test]
    fn test_mul_macro_property(a: u8, b: u8) {
        prop_assert_eq!(test_mul_macro(a, b), a.checked_mul(b).ok_or(()));
    }

    #[test]
    fn test_div_macro_property(a: u8, b: u8) {
        prop_assert_eq!(test_div_macro(a, b), a.checked_div(b).ok_or(()));
    }

    #[test]
    fn test_expression_property(expr in any_with::<Expression>(3)) {
        let _ = expr.evaluate_expression();
    }
}

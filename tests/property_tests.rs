use proptest::prelude::*;

mod common;
use common::{
    test_add_macro, test_div_macro, test_mul_macro, test_rem_macro, test_sub_macro, Expression,
};

macro_rules! test_numeric_types {
    ($(($name:ident, $t:ty)),*) => {
        $(
            proptest! {
                #[test]
                fn $name(a in any::<$t>(), b in any::<$t>()) {
                        prop_assert_eq!(test_add_macro(a, b), a.checked_add(b).ok_or(()));
                        prop_assert_eq!(test_sub_macro(a, b), a.checked_sub(b).ok_or(()));
                        prop_assert_eq!(test_mul_macro(a, b), a.checked_mul(b).ok_or(()));
                        prop_assert_eq!(test_div_macro(a, b), a.checked_div(b).ok_or(()));
                }
            }
        )*
    };
}

test_numeric_types!(
    (test_u8_props, u8),
    (test_u16_props, u16),
    (test_u32_props, u32),
    (test_u64_props, u64),
    (test_u128_props, u128),
    (test_usize_props, usize),
    (test_i8_props, i8),
    (test_i16_props, i16),
    (test_i32_props, i32),
    (test_i64_props, i64),
    (test_i128_props, i128),
    (test_isize_props, isize)
);

macro_rules! test_float_types {
    ($(($name:ident, $t:ty)),*) => {
        $(
            proptest! {
                #[test]
                fn $name(a in any::<$t>(), b in any::<$t>()) {

                    prop_assert_eq!(test_add_macro(a, b), ((a+b).is_finite()).then(|| (a+b)).ok_or(()));
                    prop_assert_eq!(test_sub_macro(a, b), ((a-b).is_finite()).then(|| (a-b)).ok_or(()));
                    prop_assert_eq!(test_mul_macro(a, b), ((a*b).is_finite()).then(|| (a*b)).ok_or(()));
                    prop_assert_eq!(test_div_macro(a, b), ((a/b).is_finite()).then(|| (a/b)).ok_or(()));
                    prop_assert_eq!(test_rem_macro(a, b), ((a%b).is_finite()).then(|| (a%b)).ok_or(()));
                }
            }
        )*
    };
}

test_float_types!((test_f32_props, f32), (test_f64_props, f64));

proptest! {
    #[test]
    fn test_expression_property(expr in any_with::<Expression>(3)) {
        let _ = expr.evaluate_expression();
    }
}

#![allow(dead_code)]

use proptest::prelude::*;
use safe_math::safe_math;

// Basic test operations
#[safe_math]
pub fn test_add_macro(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a + b)
}

#[safe_math]
pub fn test_sub_macro(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a - b)
}

#[safe_math]
pub fn test_mul_macro(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a * b)
}

#[safe_math]
pub fn test_div_macro(a: u8, b: u8) -> Result<u8, ()> {
    Ok(a / b)
}

// Expression tree for testing complex arithmetic expressions
#[derive(Debug)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Rem(Box<Expression>, Box<Expression>),
    Literal(u8),
}

impl Arbitrary for Expression {
    type Parameters = u8;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(depth: Self::Parameters) -> Self::Strategy {
        if depth == 0 {
            any::<u8>().prop_map(Expression::Literal).boxed()
        } else {
            (
                0u8..5u8,
                Expression::arbitrary_with(depth - 1),
                Expression::arbitrary_with(depth - 1),
            )
                .prop_map(|(op, left, right)| {
                    let left_box = Box::new(left);
                    let right_box = Box::new(right);
                    match op {
                        0 => Expression::Add(left_box, right_box),
                        1 => Expression::Sub(left_box, right_box),
                        2 => Expression::Mul(left_box, right_box),
                        3 => Expression::Div(left_box, right_box),
                        4 => Expression::Rem(left_box, right_box),
                        _ => unreachable!(),
                    }
                })
                .boxed()
        }
    }
}

impl Expression {
    #[safe_math]
    pub fn evaluate_expression(&self) -> Result<u8, ()> {
        match self {
            Expression::Add(left, right) => {
                let left = left.evaluate_expression()?;
                let right = right.evaluate_expression()?;
                let result1 = left + right;
                let result2 = left.checked_add(right).ok_or(())?;
                assert_eq!(result1, result2);
                Ok(result1)
            }
            Expression::Sub(left, right) => {
                let left = left.evaluate_expression()?;
                let right = right.evaluate_expression()?;
                let result1 = left - right;
                let result2 = left.checked_sub(right).ok_or(())?;
                assert_eq!(result1, result2);
                Ok(result1)
            }
            Expression::Mul(left, right) => {
                let left = left.evaluate_expression()?;
                let right = right.evaluate_expression()?;
                let result1 = left * right;
                let result2 = left.checked_mul(right).ok_or(())?;
                assert_eq!(result1, result2);
                Ok(result1)
            }
            Expression::Div(left, right) => {
                let left = left.evaluate_expression()?;
                let right = right.evaluate_expression()?;
                let result1 = left / right;
                let result2 = left.checked_div(right).ok_or(())?;
                assert_eq!(result1, result2);
                Ok(result1)
            }
            Expression::Rem(left, right) => {
                let left = left.evaluate_expression()?;
                let right = right.evaluate_expression()?;
                let result1 = left % right;
                let result2 = left.checked_rem(right).ok_or(())?;
                assert_eq!(result1, result2);
                Ok(result1)
            }
            Expression::Literal(value) => Ok(*value),
        }
    }
}

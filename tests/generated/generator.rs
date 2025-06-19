/// This module contains the logic for generating random arithmetic expression tests.
/// It creates pairs of equivalent expressions using both regular operators and
/// their checked counterparts to verify the safe_math macro's behavior.
use rand::Rng;

/// Number of test cases to generate per type
pub const NUM_TEST_CASES: usize = 50;

/// Basic arithmetic operators that will be used in the unsafe expressions
const OPERATORS: [&str; 5] = ["+", "-", "*", "/", "%"];

/// Corresponding checked methods for each operator
const CHECKED_OPERATORS: [&str; 5] = [
    "checked_add",
    "checked_sub",
    "checked_mul",
    "checked_div",
    "checked_rem",
];

/// All numeric types that will be tested
const NUMERIC_TYPES: [&str; 12] = [
    // Unsigned integers
    "u8", "u16", "u32", "u64", "u128", // Signed integers
    "i8", "i16", "i32", "i64", "i128", // usize
    "usize", "isize",
    // TODO: Floating point
    // "f32", "f64",
];

/// A builder struct that helps construct both the unsafe and safe versions
/// of an arithmetic expression in parallel
struct ExpressionBuilder {
    /// The unsafe expression using regular operators (e.g., "a + b")
    expr: String,
    /// The safe expression using checked methods (e.g., "a.checked_add(b)")
    expr_safe: String,
}

impl ExpressionBuilder {
    /// Creates a new ExpressionBuilder starting with an initial value
    fn new(initial: &str) -> Self {
        Self {
            expr: initial.to_string(),
            expr_safe: initial.to_string(),
        }
    }

    /// Adds a new operation to both expressions
    ///
    /// # Arguments
    /// * `op` - The regular operator (e.g., "+")
    /// * `checked_op` - The corresponding checked method (e.g., "checked_add")
    /// * `arg` - The argument to use in the operation
    fn add_operation(&mut self, op: &str, checked_op: &str, arg: &str) {
        // For unsafe expression, wrap in parentheses to maintain operator precedence
        self.expr = format!("({} {} {})", self.expr, op, arg);
        // For safe expression, chain the checked method call and convert Option to Result
        self.expr_safe = format!("{}.{}({}).ok_or(())?", self.expr_safe, checked_op, arg);
    }
}

/// Generates a single test case with a random number of arguments and operations
///
/// # Arguments
/// * `test_number` - The index of this test case, used to generate unique function names
/// * `numeric_type` - The type of the numeric arguments
fn generate_single_test(test_number: usize, numeric_type: &str) -> String {
    let mut rng = rand::rng();

    // Generate between 2 and 10 arguments
    let num_args = rng.random_range(2..=10);
    let arg_names: Vec<String> = (1..=num_args).map(|i| format!("a{i}")).collect();

    let mut builder = ExpressionBuilder::new(&arg_names[0]);

    // Add random operations with the remaining arguments
    for arg in arg_names.iter().skip(1) {
        let op_idx = rng.random_range(0..OPERATORS.len());
        builder.add_operation(OPERATORS[op_idx], CHECKED_OPERATORS[op_idx], arg);
    }

    // Generate appropriate random value based on type
    let random_gen = match numeric_type {
        "u8" => "rng.random::<u8>()",
        "u16" => "rng.random::<u16>()",
        "u32" => "rng.random::<u32>()",
        "u64" => "rng.random::<u64>()",
        "u128" => "rng.random::<u128>()",
        "i8" => "rng.random::<i8>()",
        "i16" => "rng.random::<i16>()",
        "i32" => "rng.random::<i32>()",
        "i64" => "rng.random::<i64>()",
        "i128" => "rng.random::<i128>()",
        "f32" => "rng.random::<f32>()",
        "f64" => "rng.random::<f64>()",
        "usize" => "rng.random::<u64>() as usize",
        "isize" => "rng.random::<i64>() as isize",
        _ => unreachable!(),
    };

    format!(
        r#"
#[test]
fn test_generated_{}_{}_equivalence() {{
    // Define the two equivalent functions:
    // 1. Using the safe_math macro
    #[safe_math]
    fn with_macro({}) -> Result<{}, ()> {{
        #[allow(unused_parens)]
        let result = {};
        Ok(result)
    }}

    // 2. Using checked operations directly
    fn with_checked({}) -> Result<{}, ()> {{
        let result = {};
        Ok(result)
    }}

    // 3. Using the safe_math_block macro
    fn with_function_macro({}) -> Result<{}, ()> {{
        #[allow(unused_parens)]
        let result = {{
            safe_math_block! {{{{
                {}
        }}}}
        }};
        Ok(result)
    }}

    // Test with multiple random inputs to increase coverage
    let mut rng = rand::rng();
    for _ in 0..100 {{
        // Generate random inputs
        let inputs = [{}];

        // Call both functions with the same inputs
        let macro_result = with_macro({});
        let checked_result = with_checked({});
        let function_macro_result = with_function_macro({});

        // Verify that both functions produce exactly the same result
        assert!(
            macro_result == checked_result &&
            macro_result == function_macro_result,
            "safe_math macro and checked operations produced different results for inputs: {{inputs:?}}"
        );
    }}
}}
"#,
        numeric_type.replace(".", "_"), // Sostituisce il punto con underscore per f32/f64
        test_number,
        // Function arguments for with_macro
        arg_names
            .iter()
            .map(|a| format!("{a}: {numeric_type}"))
            .collect::<Vec<_>>()
            .join(", "),
        numeric_type,
        // Expression for with_macro
        builder.expr,
        // Function arguments for with_checked
        arg_names
            .iter()
            .map(|a| format!("{a}: {numeric_type}"))
            .collect::<Vec<_>>()
            .join(", "),
        numeric_type,
        // Expression for with_checked
        builder.expr_safe,
        // Function arguments for with_function_macro
        arg_names
            .iter()
            .map(|a| format!("{a}: {numeric_type}"))
            .collect::<Vec<_>>()
            .join(", "),
        numeric_type,
        // Expression for with_function_macro
        builder.expr,
        // Random input generation
        arg_names
            .iter()
            .map(|_| random_gen)
            .collect::<Vec<_>>()
            .join(", "),
        // Arguments for with_macro call
        (0..arg_names.len())
            .map(|i| format!("inputs[{i}]"))
            .collect::<Vec<_>>()
            .join(", "),
        // Arguments for with_checked call
        (0..arg_names.len())
            .map(|i| format!("inputs[{i}]"))
            .collect::<Vec<_>>()
            .join(", "),
        // Arguments for with_function_macro call
        (0..arg_names.len())
            .map(|i| format!("inputs[{i}]"))
            .collect::<Vec<_>>()
            .join(", "),
    )
}

/// Generates all test cases and combines them into a single string
pub fn generate_test_cases() -> String {
    let mut test_file = String::from(
        r#"
#[cfg(test)]
use safe_math::{safe_math, safe_math_block};
#[cfg(test)]
use rand::Rng;

// This file is auto-generated. Do not edit manually.
// Each test verifies that the safe_math macro produces identical results
// to using checked arithmetic operations directly.
"#,
    );

    // Generate test cases for each numeric type
    for type_name in NUMERIC_TYPES.iter() {
        for i in 0..NUM_TEST_CASES {
            test_file.push_str(&generate_single_test(i, type_name));
        }
    }

    test_file
}

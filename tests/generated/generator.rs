/// This module contains the logic for generating random arithmetic expression tests.
/// It creates pairs of equivalent expressions using both regular operators and
/// their checked counterparts to verify the safe_math macro's behavior.
use rand::Rng;

/// Number of test cases to generate
pub const NUM_TEST_CASES: usize = 100;

/// Basic arithmetic operators that will be used in the unsafe expressions
const OPERATORS: [&str; 5] = ["+", "-", "*", "/", "%"];

/// Corresponding checked methods for each operator that will be used in the safe expressions
const CHECKED_OPERATORS: [&str; 5] = ["checked_add", "checked_sub", "checked_mul", "checked_div", "checked_rem"];

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
fn generate_single_test(test_number: usize) -> String {
    let mut rng = rand::thread_rng();
    
    // Generate between 2 and 4 arguments (a1, a2, etc.)
    let num_args = rng.gen_range(2..=4);
    let arg_names: Vec<String> = (1..=num_args).map(|i| format!("a{}", i)).collect();
    
    // Create a builder starting with the first argument
    let mut builder = ExpressionBuilder::new(&arg_names[0]);
    
    // Add random operations with the remaining arguments
    for i in 1..arg_names.len() {
        let op_idx = rng.gen_range(0..OPERATORS.len());
        builder.add_operation(
            OPERATORS[op_idx],
            CHECKED_OPERATORS[op_idx],
            &arg_names[i]
        );
    }
    
    format!(
        r#"
#[test]
fn test_generated_{}_equivalence() {{
    use safe_math_rs::safe_math;
    use rand::Rng;

    // Define the two equivalent functions:
    // 1. Using the safe_math macro
    #[safe_math]
    fn with_macro({}) -> Result<u8, ()> {{
        #[allow(unused_parens)]
        Ok({})
    }}

    // 2. Using checked operations directly
    fn with_checked({}) -> Result<u8, ()> {{
        Ok({})
    }}

    // Test with multiple random inputs to increase coverage
    let mut rng = rand::rng();
    for _ in 0..100 {{
        // Generate random inputs
        let inputs = [{}];
        
        // Call both functions with the same inputs
        let macro_result = with_macro({});
        let checked_result = with_checked({});

        // Verify that both functions produce exactly the same result
        assert_eq!(
            macro_result, 
            checked_result,
            "safe_math macro and checked operations produced different results for inputs: {{:?}}", 
            inputs
        );
    }}
}}
"#,
        test_number,
        // Function arguments for with_macro
        arg_names.iter().map(|a| format!("{}: u8", a)).collect::<Vec<_>>().join(", "),
        // Expression for with_macro
        builder.expr,
        // Function arguments for with_checked
        arg_names.iter().map(|a| format!("{}: u8", a)).collect::<Vec<_>>().join(", "),
        // Expression for with_checked
        builder.expr_safe,
        // Random input generation
        arg_names.iter().map(|_| "rng.random::<u8>()").collect::<Vec<_>>().join(", "),
        // Arguments for with_macro call
        (0..arg_names.len()).map(|i| format!("inputs[{}]", i)).collect::<Vec<_>>().join(", "),
        // Arguments for with_checked call
        (0..arg_names.len()).map(|i| format!("inputs[{}]", i)).collect::<Vec<_>>().join(", ")
    )
}

/// Generates all test cases and combines them into a single string
pub fn generate_test_cases() -> String {
    let mut test_file = String::from(r#"
use rand;

// This file is auto-generated. Do not edit manually.
// Each test verifies that the safe_math macro produces identical results
// to using checked arithmetic operations directly.
"#);

    // Generate the specified number of test cases
    for i in 0..NUM_TEST_CASES {
        test_file.push_str(&generate_single_test(i));
    }

    test_file
} 
/// This module contains the logic for generating assembly comparison tests.
/// It creates pairs of equivalent functions using both the safe_math macro and
/// direct checked operations to verify that they generate identical assembly.

/// The operations we want to test
pub const OPERATIONS: [(&str, &str, &str); 4] = [
    ("+", "add", "checked_add"),
    ("-", "sub", "checked_sub"),
    ("*", "mul", "checked_mul"),
    ("/", "div", "checked_div"),
];

/// All numeric types to test
pub const NUMERIC_TYPES: [&str; 12] = [
    "u8", "u16", "u32", "u64", "u128",
    "i8", "i16", "i32", "i64", "i128",
    "usize", "isize"
];

/// Generate a test case for comparing assembly output
fn generate_asm_test(op: &str, name: &str, checked_op: &str, num_type: &str) -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_path = std::path::Path::new(&manifest_dir)
        .join("../..")
        .canonicalize()
        .unwrap();
    let workspace_path_str = workspace_path.to_string_lossy().replace('\\', "/");

    let cargo_toml_content = format!(
        r#"
        [package]
        name = "macro-test-{name}-{num_type}"
        version = "0.1.0"
        edition = "2021"

        [workspace]

        [lib]
        path = "macro_test_{name}_{num_type}.rs"
        crate-type = ["rlib"]

        [dependencies]
        safe-math = {{ path = "{}" }}
    "#,
        workspace_path_str
    );

    format!(
        r##"
#[test]
fn test_asm_{name}_{num_type}_equivalence() {{
    use std::process::Command;
    use std::fs;
    use std::path::PathBuf;

    // Create the test functions in separate source files
    let temp_dir = PathBuf::from(env!("OUT_DIR"));
    let test_dir = temp_dir.join("test_{name}_{num_type}");
    fs::create_dir_all(&test_dir).unwrap();
    
    // Source with macro
    let macro_source = r#"
        use safe_math::safe_math;
        
        #[no_mangle]
        #[inline(never)]
        #[safe_math]
        pub fn test_fn(a: {num_type}, b: {num_type}) -> Result<{num_type}, ()> {{
            Ok(a {op} b)
        }}
    "#;

    // Source with checked operations
    let checked_source = r#"
        #[no_mangle]
        #[inline(never)]
        pub fn test_fn(a: {num_type}, b: {num_type}) -> Result<{num_type}, ()> {{
            a.{checked_op}(b).ok_or(())
        }}
    "#;

    // Write source files with unique names
    let macro_src_path = test_dir.join("macro_test_{name}_{num_type}.rs");
    let checked_src_path = test_dir.join("checked_test_{name}_{num_type}.rs");
    fs::write(&macro_src_path, macro_source).unwrap();
    fs::write(&checked_src_path, checked_source).unwrap();

    // Create Cargo.toml for macro test
    let cargo_toml = r#"{cargo_toml_content}"#;
    fs::write(test_dir.join("Cargo.toml"), cargo_toml).unwrap();

    // Compile macro file to assembly
    let status = Command::new("cargo")
        .current_dir(&test_dir)
        .args([
            "rustc",
            "--release",
            "--",
            "--emit=asm",
            "-C",
            "opt-level=3",
        ])
        .status()
        .unwrap();
    
    assert!(status.success(), "Failed to compile macro_test to assembly");

    // Compile checked operations file to assembly
    let checked_asm_path = test_dir.join("checked_test_{name}_{num_type}.s");
    let status = Command::new("rustc")
        .current_dir(&test_dir)
        .args([
            "--crate-type=rlib",
            "-C",
            "opt-level=3",
            "--emit=asm",
            "-o",
            "checked_test_{name}_{num_type}.s",
            "checked_test_{name}_{num_type}.rs",
        ])
        .status()
        .unwrap();
    
    assert!(status.success(), "Failed to compile checked_test to assembly");

    // Read and clean assembly files
    let mut asm_files = Vec::new();

    // Find macro test assembly file and copy it to the same directory
    let macro_asm_file = fs::read_dir(test_dir.join("target/release/deps"))
        .unwrap()
        .filter_map(Result::ok)
        .find(|entry| {{
            let path = entry.path();
            path.extension() == Some(std::ffi::OsStr::new("s"))
                && path.file_stem().unwrap().to_string_lossy().starts_with("macro_test_{name}_{num_type}")
        }})
        .expect("Assembly file not found for macro test");

    let macro_asm_path = test_dir.join("macro_test_{name}_{num_type}.s");
    fs::copy(macro_asm_file.path(), &macro_asm_path).unwrap();

    for file_path in &[macro_asm_path, checked_asm_path] {{
        let asm = fs::read_to_string(file_path).unwrap();
        
        let cleaned_asm: Vec<String> = asm
            .lines()
            .skip_while(|line| !line.contains("_test_fn:"))
            .skip(1)
            .take_while(|line| !line.trim().starts_with("ret"))
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !line.starts_with('.') && !line.starts_with("LBB") && !line.contains(":"))
            .collect();

        asm_files.push(cleaned_asm.join("\n"));
    }}

    let [ref macro_asm, ref checked_asm] = asm_files[..] else {{
        panic!("Expected exactly 2 assembly files");
    }};

    assert_eq!(
        *macro_asm, *checked_asm,
        "Assembly is not identical for {name} operation with {num_type}.\n\nMacro:\n{{}}\n\nChecked:\n{{}}\n",
        macro_asm, checked_asm
    );
}}
"##,
        name = name,
        op = op,
        checked_op = checked_op,
        num_type = num_type,
        cargo_toml_content = cargo_toml_content,
    )
}

/// Generate all assembly comparison tests
pub fn generate_test_cases() -> String {
    OPERATIONS
        .iter()
        .flat_map(|(op, name, checked_op)| {
            NUMERIC_TYPES.iter().map(move |num_type| {
                generate_asm_test(op, name, checked_op, num_type)
            })
        })
        .collect()
} 
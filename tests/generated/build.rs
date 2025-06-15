use std::fs;
use std::path::Path;
use std::env;

mod test_generator {
    include!("generator.rs");
}

fn main() {
    // Tell Cargo to re-run this script if the generator code changes
    println!("cargo:rerun-if-changed=generator.rs");
    
    // Generate the test cases using our generator module
    let test_cases = test_generator::generate_test_cases();
    
    // Get the output directory from Cargo's environment variable
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_tests.rs");
    
    // Write the generated tests to a file in the build output directory
    fs::write(&dest_path, test_cases).unwrap();
    
    // Print a build message showing how many test cases were generated
    println!("cargo:warning=Generated {} test cases", test_generator::NUM_TEST_CASES);
} 
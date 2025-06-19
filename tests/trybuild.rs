use trybuild::TestCases;

#[test]
fn ui() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/bad_return_type.rs");
    #[cfg(feature = "derive")]
    {
        t.compile_fail("tests/ui/bad_derive.rs");
        t.compile_fail("tests/ui/bad_derive_missing_checked_trait_unused.rs");
        t.compile_fail("tests/ui/bad_derive_missing_checked_trait_used.rs");
        t.compile_fail("tests/ui/bad_derive_missing_attributes.rs");
    }
}

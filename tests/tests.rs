#[test]
pub fn expand_examples_pass() {
    macrotest::expand("test/expand/**/*.rs");
}

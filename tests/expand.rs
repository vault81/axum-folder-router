#[test]
pub fn expand_snapshot_pass() {
    macrotest::expand_args("tests/expand/*.rs", &["--features", "nightly,debug"]);
}

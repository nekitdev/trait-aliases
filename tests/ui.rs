use trybuild::TestCases;

#[test]
fn ui_pass() {
    let test_cases = TestCases::new();

    test_cases.pass("tests/ui/correct/*.rs");
}

#[test]
fn ui_fail() {
    let test_cases = TestCases::new();

    test_cases.compile_fail("tests/ui/incorrect/*.rs");
}

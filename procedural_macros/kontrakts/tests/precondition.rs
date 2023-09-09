use kontrakts::contracts;

#[contracts]
#[precondition(input)]
fn f(input: bool) -> i32 {
    0
}

#[test]
fn test_precondition_pass() {
    f(true);
}

#[test]
#[should_panic == "violation of precondition: `input`"]
fn test_precondition_fail() {
    f(false);
}
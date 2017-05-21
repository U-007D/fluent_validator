use super::*;

struct NonEmptyStringValidator;

impl Validator<String> for NonEmptyStringValidator {
    fn is_valid(value: &String) -> bool {
        !value.is_empty()
    }
}

#[test]
fn empty_test() {}

#[test]
fn validator_handles_valid_input() {
    let input = "non-empty test value".to_string();
    let expected_result = true;

    assert!(NonEmptyStringValidator::is_valid(&input) == expected_result);
}

#[test]
fn imperative_validator_handles_invalid_input() {
    let input = String::new();
    let expected_result = false;

    assert!(NonEmptyStringValidator::is_valid(&input) == expected_result);
}

#[test]
fn fluent_validator_handles_valid_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(input.clone());

    assert!(input.validate(NonEmptyStringValidator) == expected_result);
}

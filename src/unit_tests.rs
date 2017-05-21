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


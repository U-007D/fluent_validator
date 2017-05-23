use super::*;

#[derive(Debug, PartialEq, Eq)]
struct NonEmptyStringValidator {
    value: String,
}

impl Validator<String> for NonEmptyStringValidator {
    fn validate(v: String) -> ValidatorResult<NonEmptyStringValidator> where Self: Sized {
        match !v.is_empty() {
            true => Ok(NonEmptyStringValidator { value: v }),
            false => Err(Error::FailedConstraint("Value is empty.".to_string())),
        }
    }
}

#[test]
fn empty_test() {}

#[test]
fn string_validator_handles_non_empty_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(NonEmptyStringValidator { value: input.clone() });

    assert!(input.validate().ok() == expected_result);
}

#[test]
fn string_validator_handles_empty_input() {
    let input = String::new();
    let expected_result = Some(Error::FailedConstraint("Value is empty.".to_string()));

    assert!(input.validate::<NonEmptyStringValidator>().err() ==  expected_result);
}

#[test]
fn hex_byte_str_validator_handles_non_empty_input() {
    let input = "non-empty test value";
    let expected_result = Some(HexByteStrValidator { value: input.clone() });

    assert!(input.validate().ok() == expected_result);
}

#[test]
fn hex_byte_str_validator_handles_empty_input() {
    let input = "";
    let expected_result = Some(Error::FailedConstraint("Value is empty.".to_string()));

    assert!(input.validate::<HexByteStrValidator>().err() ==  expected_result);
}


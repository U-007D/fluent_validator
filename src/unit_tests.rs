use super::*;

#[derive(Debug, PartialEq, Eq)]
struct NonEmptyStringValidator {
    value: String,
}

impl Validator<String> for NonEmptyStringValidator {
    fn validate(v: String) -> ValidatorResult<NonEmptyStringValidator> where Self: Sized {
        match !v.is_empty() {
            true => Ok(NonEmptyStringValidator { value: v }),
            false => Err(Error::FailedConstraint(VE_EMPTY_VALUE.to_string())),
        }
    }
}

#[test]
fn empty_test() {}

#[test]
fn string_validator_handles_non_empty_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(NonEmptyStringValidator { value: input.clone() });

    assert_eq!(input.validate().ok(), expected_result);
}

#[test]
fn string_validator_handles_empty_input() {
    let input = String::new();
    let expected_result = Some(Error::FailedConstraint(VE_EMPTY_VALUE.to_string()));

    assert_eq!(input.validate::<NonEmptyStringValidator>().err(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_empty_input() {
    let input = "";
    let expected_result = Some(Error::FailedConstraint(VE_EMPTY_VALUE.to_string()));

    assert_eq!(input.validate::<HexByteStrValidator>().err(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_valid_hex_input() {
    let input = "0123456789abcdefABCDEF";
    let expected_result = Some(HexByteStrValidator{ value: input.clone() });

    assert_eq!(input.validate::<HexByteStrValidator>().ok(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_invalid_hex_input() {
    let input = "0123456789xxabcdefABCDEF";
    let expected_result = Some(Error::FailedConstraint(VE_INVALID_HEX_DIGIT.to_string()));

    assert_eq!(input.validate::<HexByteStrValidator>().err(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_odd_length_hex_input() {
    let input = "0123456789abcdefABCDE";
    let expected_result = Some(Error::FailedConstraint(VE_ODD_NUMBER_OF_HEX_DIGITS.to_string()));

    assert_eq!(input.validate::<HexByteStrValidator>().err(), expected_result);
}

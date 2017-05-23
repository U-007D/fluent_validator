use super::*;

const VAL_ERR_EMPTY_VALUE: &'static str              = "Value is empty.";

#[derive(Debug, PartialEq, Eq)]
struct NonEmptyStringValidator {
    value: String,
}

impl Validator<String> for NonEmptyStringValidator {
    fn validate(v: String) -> Result<NonEmptyStringValidator> where Self: Sized {
        match !v.is_empty() {
            true => Ok(NonEmptyStringValidator { value: v }),
            false => Err(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string())),
        }
    }
}

#[test]
fn empty_test() {}

#[test]
fn string_validator_handles_non_empty_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(NonEmptyStringValidator { value: input.clone() });

    assert_eq!(input.validate::<NonEmptyStringValidator>().ok(), expected_result);
}

#[test]
fn string_validator_handles_empty_input() {
    let input = String::new();
    let expected_result = Some(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string()));

    assert_eq!(input.validate::<NonEmptyStringValidator>().err(), expected_result);
}

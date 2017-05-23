use super::*;

#[derive(Debug, PartialEq, Eq)]
struct NonEmptyStringValidator {
    value: String,
}

impl Validate<String> for NonEmptyStringValidator {
    fn is_valid(v: String) -> ValidatorResult<NonEmptyStringValidator> where Self: Sized {
        match !v.is_empty() {
            true => Ok(NonEmptyStringValidator { value: v }),
            false => Err(Error::FailedConstraint("Value is empty.".to_string())),
        }
    }
}

#[test]
fn empty_test() {}

#[test]
fn validator_handles_non_empty_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(NonEmptyStringValidator { value: input.clone() });

    assert!(input.validate().ok() == expected_result);
}

#[test]
fn imperative_validator_handles_invalid_input() {
    let input = String::new();
    let expected_result = Some(Error::FailedConstraint("Value is empty.".to_string()));

    assert!(input.validate::<NonEmptyStringValidator>().err() ==  expected_result);
}

//#[test]
//fn fluent_validator_handles_valid_input() {
//    let input = "non-empty test value".to_string();
//    let expected_result = Some(input.clone());
//
//    assert!(input.validate(NonEmptyStringValidator) == expected_result);
//}

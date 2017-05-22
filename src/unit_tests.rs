use super::*;

//struct NonEmptyStringValidator;

impl Validator for String {
    fn validate(self) -> ValidatorResult<Self> where Self: Sized {
        match !self.is_empty() {
            true => Ok(self),
            false => Err(Error::FailedConstraint("Value is empty.".to_string())),
        }
    }
}

#[test]
fn empty_test() {}

#[test]
fn validator_handles_non_empty_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(input.clone());

    assert!(input.validate().ok() == expected_result);
}

#[test]
fn imperative_validator_handles_invalid_input() {
    let input = String::new();
    let expected_result = Some(Error::FailedConstraint("Value is empty.".to_string()));

    assert!(input.validate().err() ==  expected_result);
}

//#[test]
//fn fluent_validator_handles_valid_input() {
//    let input = "non-empty test value".to_string();
//    let expected_result = Some(input.clone());
//
//    assert!(input.validate(NonEmptyStringValidator) == expected_result);
//}

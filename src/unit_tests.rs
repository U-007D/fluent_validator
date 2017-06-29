use super::*;

const VAL_ERR_EMPTY_VALUE: &'static str              = "Value is empty.";

#[derive(Debug, PartialEq)]
struct NonEmptyStringValidator(f64);

impl Validator<f64> for NonEmptyStringValidator {
    fn validate(v: &f64) -> Result<&f64> {
        match *v >= 0.0 {
            true => Ok(v),
            false => Err(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string())),
        }
    }
}

#[test]
fn empty_test() {}

//#[test]
//fn string_validator_handles_non_empty_input() {
//    let input = "non-empty test value".to_string();
//    let expected_result = Some(input.clone());
//
//    assert_eq!(input.validate::<NonEmptyStringValidator>().ok(), expected_result);
//}
//
//#[test]
//fn string_validator_handles_empty_input() {
//    let input = String::new();
//    let expected_result = Some(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string()));
//
//    assert_eq!(input.validate::<NonEmptyStringValidator>().err(), expected_result);
//}

#[test]
fn string_validator_handles_positive_input() {
    let input = 5f64;
    let expected_result = Some(&input);

    assert_eq!(input.validate::<NonEmptyStringValidator>().ok(), expected_result);
}

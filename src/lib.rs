#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

mod consts;

#[cfg(test)]
mod unit_tests;

type GeneralError = Box<std::error::Error>;
type GeneralResult<T> = Result<T, GeneralError>;
type ValidatorResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FailedConstraint(String),
}

pub trait Validate<T> {
    fn is_valid(T) -> Result<Self, Error> where Self: Sized;
}

trait Validator: Sized {
    fn validate<T: Validate<Self>>(self) -> ValidatorResult<T>;
}

impl<T> Validator for T {
    fn validate<U: Validate<T>>(self) -> ValidatorResult<U> {
        U::validate(self)
    }
}

pub fn lib_main(_args: Vec<String>) -> GeneralResult<()>
{
    Ok(())
}

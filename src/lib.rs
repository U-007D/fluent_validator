#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

mod consts;

#[cfg(test)] mod unit_tests;

type ValidatorResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FailedConstraint(String),
}

pub trait Validator<T> {
    fn validate(T) -> Result<Self, Error> where Self: Sized;
}

pub trait FluentValidator: Sized {
    fn validate<T: Validator<Self>>(self) -> ValidatorResult<T>;
}


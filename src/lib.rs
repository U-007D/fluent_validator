#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

mod consts;

#[cfg(test)] mod unit_tests;


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyValue(String),
    IllegalValue(String),
    InvalidSize(String),
}

type Result<T> = std::result::Result<T, Error>;

pub trait Validator<T> {
    fn validate(T) -> Result<Self> where Self: Sized;
}

pub trait FluentValidator: Sized {
    fn validate<T: Validator<Self>>(self) -> Result<T>;
}

impl<T> FluentValidator for T {
    fn validate<U: Validator<T>>(self) -> Result<U> {
        U::validate(self)
    }
}

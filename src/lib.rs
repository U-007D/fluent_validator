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

pub trait Validator<GT> {
    fn validate(GT) -> Result<GT> where Self: Sized;
}

pub trait FluentValidator: Sized {
    fn validate<CT: Validator<Self>>(self) -> Result<Self> where Self: Sized;
}

impl<CT> FluentValidator for CT {
    fn validate<VT: Validator<CT>>(self) -> Result<Self> where Self: Sized {
        VT::validate(self)
    }
}

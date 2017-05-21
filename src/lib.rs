#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

mod consts;
use consts::msgs;

#[cfg(test)]
mod unit_tests;

pub fn lib_main(_args: Vec<String>) -> Result<()>
{
    Ok(())
}

pub trait Validator<T> {
    fn is_valid(value: &T) -> bool;
}

pub trait FluentValidator {
    fn validate<T>(self, validator: T) -> Option<Self> where T: Validator<Self>,
                                                             Self: Sized {
        match T::is_valid(&self) {
            true => Some(self),
            false => None,
        }
    }
}

impl<T> FluentValidator for T {}

pub mod msgs;
pub use self::msgs::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyValue(String),
    IllegalValue(String),
    InvalidSize(String),
}

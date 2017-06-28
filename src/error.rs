use std::error::Error as StdError;
use std::fmt;

/// This type represents all possible errors that can occur when validating a type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EmptyValue(String),
    IllegalValue(String),
    InvalidSize(String),
    MultipleErrors(Vec<Error>),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::EmptyValue(ref msg) => msg.as_str(),
            Error::IllegalValue(ref msg) => msg.as_str(),
            Error::InvalidSize(ref msg) => msg.as_str(),
            Error::MultipleErrors(_) => "Multiple Validation Errors",
        }
    }
}

/// Returns the error's `description()` prefixed by the error's type.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::EmptyValue(_) => f.write_str(format!("EmptyValue > {}", self.description()).as_str()),
            Error::IllegalValue(_) => f.write_str(format!("IllegalValue > {}", self.description()).as_str()),
            Error::InvalidSize(_) => f.write_str(format!("InvalidSize > {}", self.description()).as_str()),
            Error::MultipleErrors(ref slice) => {
                let mut msg = format!("MultipleErrors > {} error{}:", slice.len(), if slice.len() > 1 { "s" } else { "" });
                for err in slice {
                    msg.push_str(format!("\n\t# {}", &err.to_string()).as_str());
                }
                f.write_str(msg.as_str())
            },
        }
    }
}
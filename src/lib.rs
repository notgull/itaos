// MIT/Apache2 License

#[macro_use]
mod macros;

mod server;
pub use server::*;

use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Exclusive,
    Exception(objc::MessageError),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Exclusive => f.write_str("Cannot create a second ItaosThread in a program"),
            Error::Exception(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Exception(e) => Some(e),
            _ => None,
        }
    }
}

impl From<objc::MessageError> for Error {
    #[inline]
    fn from(m: objc::MessageError) -> Error {
        Error::Exception(m)
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;

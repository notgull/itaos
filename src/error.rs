// MIT/Apache2 License

use std::fmt;

/// An error returned by this library.
#[derive(Debug)]
pub enum Error {
    /// Failed to send directive.
    FailedToSendDirective,
    /// Wrapper around an Objective-C exception
    ObjcException(clever_graphics::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToSendDirective => f.write_str("Failed to send directive to server"),
            Self::ObjcException(e) => write!(f, "{}", e),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;

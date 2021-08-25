//! A `serde_dynamic_typing` related error

use ebacktrace::define_error;
use std::{
    error, result,
    fmt::{ self, Display, Formatter }
};


/// Creates a new variant
#[doc(hidden)]
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::ErrorImpl::with_string($kind, format!($($arg)*)) })
}
/// Creates a new `Error::Serde` kind
#[macro_export] macro_rules! eserde {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::SerdeError, $($arg)*) });
}
/// Creates a new `Error::InvalidData` kind
#[macro_export] macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InvalidData, $($arg)*) });
}
/// Creates a new `Error::TypeError` kind
#[macro_export] macro_rules! etype {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::TypeError, $($arg)*) });
}
/// Creates a new `Error::NoSuchKey` kind
#[macro_export] macro_rules! enokey {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::NoSuchKey, $($arg)*) });
}


/// A `serde_type_error` error kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    /// A serde related error
    SerdeError,
    /// Invalid encoded data
    InvalidData,
    /// Failed to convert a value from/to the given type
    TypeError,
    /// A specified key does not exist
    NoSuchKey
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SerdeError => write!(f, "Serde error"),
            Self::InvalidData => write!(f, "Invalid encoded data"),
            Self::TypeError => write!(f, "Failed to convert a value from/to the given type"),
            Self::NoSuchKey => write!(f, "There is no such key or no value for the given key")
        }
    }
}
impl error::Error for ErrorKind {
    /* Nothing to implement */
}


// Define our custom `serde_dynamic_typing` error
define_error!(ErrorImpl);
impl serde::de::Error for ErrorImpl<ErrorKind> {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self::with_string(ErrorKind::SerdeError, msg.to_string())
    }
}
impl serde::ser::Error for ErrorImpl<ErrorKind> {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self::with_string(ErrorKind::SerdeError, msg.to_string())
    }
}


/// A typealias for results with our error kind as specialization
pub type Error = ErrorImpl<ErrorKind>;
/// A typealias for results with our error as error-variant
pub type Result<T = ()> = result::Result<T, ErrorImpl<ErrorKind>>;
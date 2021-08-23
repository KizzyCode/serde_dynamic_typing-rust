//! A `serde_dynamic_typing` related error

use ebacktrace::define_error;
use std::{
    error, result,
    fmt::{ self, Display, Formatter }
};


/// Creates a new variant
#[doc(hidden)]
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::STLError::with_string($kind, format!($($arg)*)) })
}
/// Creates a new `Error::Serde` kind
#[macro_export] macro_rules! eserde {
    ($($arg:tt)*) => ({ e!($crate::error::STLErrorKind::SerdeError, $($arg)*) });
}
/// Creates a new `Error::InvalidData` kind
#[macro_export] macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::STLErrorKind::InvalidData, $($arg)*) });
}
/// Creates a new `Error::TypeError` kind
#[macro_export] macro_rules! etype {
    ($($arg:tt)*) => ({ e!($crate::error::STLErrorKind::TypeError, $($arg)*) });
}
/// Creates a new `Error::NoSuchKey` kind
#[macro_export] macro_rules! enokey {
    ($($arg:tt)*) => ({ e!($crate::error::STLErrorKind::NoSuchKey, $($arg)*) });
}


/// A `serde_type_error` error kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum STLErrorKind {
    /// A serde related error
    SerdeError,
    /// Invalid encoded data
    InvalidData,
    /// Failed to convert a value from/to the given type
    TypeError,
    /// A specified key does not exist
    NoSuchKey
}
impl Display for STLErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SerdeError => write!(f, "Serde error"),
            Self::InvalidData => write!(f, "Invalid encoded data"),
            Self::TypeError => write!(f, "Failed to convert a value from/to the given type"),
            Self::NoSuchKey => write!(f, "There is no such key or no value for the given key")
        }
    }
}
impl error::Error for STLErrorKind {
    /* Nothing to implement */
}


// Define our custom `serde_dynamic_typing` error
define_error!(STLError);
impl serde::de::Error for STLError<STLErrorKind> {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self::with_string(STLErrorKind::SerdeError, msg.to_string())
    }
}
impl serde::ser::Error for STLError<STLErrorKind> {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self::with_string(STLErrorKind::SerdeError, msg.to_string())
    }
}


/// A typealias for results with our error kind as specialization
pub type Error = STLError<STLErrorKind>;
/// A typealias for results with our error as error-variant
pub type Result<T = ()> = result::Result<T, STLError<STLErrorKind>>;
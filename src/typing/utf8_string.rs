//! Implements a type unifier for UTF-8 string like native types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{
    convert::TryFrom,
    fmt::{ self, Display, Formatter },
    ops::{ Deref, DerefMut }
};


/// A type unifier for UTF-8 string types
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Utf8String {
    /// The wrapped value
    inner: String
}
impl Utf8String {
    /// Creates a new `Utf8String` from `T`
    pub fn new<T>(value: T) -> Self where T: ToString {
        Self { inner: value.to_string() }
    }
}
impl From<String> for Utf8String {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl From<&str> for Utf8String {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
impl From<char> for Utf8String {
    fn from(value: char) -> Self {
        Self::new(value)
    }
}
impl TryFrom<AnyValue> for Utf8String {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Utf8String(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to UTF-8 string", value))
        }
    }
}
impl Deref for Utf8String {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Utf8String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl Display for Utf8String {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl AsRef<String> for Utf8String {
    fn as_ref(&self) -> &String {
        self
    }
}
impl AsMut<String> for Utf8String {
    fn as_mut(&mut self) -> &mut String {
        self
    }
}
impl AsRef<str> for Utf8String {
    fn as_ref(&self) -> &str {
        self
    }
}
impl AsMut<str> for Utf8String {
    fn as_mut(&mut self) -> &mut str {
        self
    }
}
impl From<Utf8String> for String {
    fn from(value: Utf8String) -> Self {
        value.inner
    }
}
impl TryFrom<Utf8String> for char {
    type Error = Error;
    fn try_from(value: Utf8String) -> Result<Self> {
        // Get the first char
        let mut chars = value.chars();
        let first = chars.next()
            .ok_or(etype!("Cannot convert an empty string to `char`"))?;

        // Ensure that the string is empty
        if !chars.next().is_none() {
            Err(etype!("Cannot convert a multichar string to a single `char`"))?;
        }
        Ok(first)
    }
}
impl From<Utf8String> for AnyValue {
    fn from(value: Utf8String) -> Self {
        AnyValue::Utf8String(value)
    }
}

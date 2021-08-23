//! Implements a type unifier for native byte types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{
    convert::TryFrom, iter::FromIterator,
    ops::{ Deref, DerefMut }
};


/// A type unifier for byte vectors
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Bytes {
    /// The wrapped value
    inner: Vec<u8>
}
impl Bytes {
    /// Creates a new `Bytes`-instance from `T`
    pub fn new<T>(value: T) -> Self where T: Into<Vec<u8>> {
        Self { inner: value.into() }
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self::new(value)
    }
}
impl<const N: usize> From<[u8; N]> for Bytes {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}
impl TryFrom<AnyValue> for Bytes {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Bytes(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to bytes", value))
        }
    }
}
impl Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Vec<u8>> for Bytes {
    fn as_ref(&self) -> &Vec<u8> {
        self
    }
}
impl AsMut<Vec<u8>> for Bytes {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        self
    }
}
impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self
    }
}
impl AsMut<[u8]> for Bytes {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}
impl From<Bytes> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        value.inner
    }
}
impl<const N: usize> TryFrom<Bytes> for [u8; N] {
    type Error = Error;
    fn try_from(value: Bytes) -> Result<Self> {
        Self::try_from(value.inner.as_slice())
            .map_err(|_| etype!("Cannot represent {} bytes as `[u8; {}]`", value.len(), N))
    }
}
impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl FromIterator<u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = u8>>(bytes: T) -> Self {
        Self { inner: bytes.into_iter().collect() }
    }
}
impl From<Bytes> for AnyValue {
    fn from(value: Bytes) -> Self {
        AnyValue::Bytes(value)
    }
}

//! Implements a type unifier for sequence like native types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{
    convert::TryFrom, iter::FromIterator,
    ops::{ Deref, DerefMut }
};


/// A type unifier for sequence likes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Sequence {
    /// The wrapped value
    inner: Vec<AnyValue>
}
impl Sequence {
    /// Creates a new `Sequence` from `T`
    pub fn new<T>(value: T) -> Self where T: Into<Vec<AnyValue>> {
        Self { inner: value.into() }
    }
}
impl From<Vec<AnyValue>> for Sequence {
    fn from(value: Vec<AnyValue>) -> Self {
        Self::new(value)
    }
}
impl From<&[AnyValue]> for Sequence {
    fn from(value: &[AnyValue]) -> Self {
        Self::new(value)
    }
}
impl<const N: usize> From<[AnyValue; N]> for Sequence {
    fn from(value: [AnyValue; N]) -> Self {
        Self::new(value)
    }
}
impl TryFrom<AnyValue> for Sequence {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Sequence(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to sequence", value))
        }
    }
}
impl Deref for Sequence {
    type Target = Vec<AnyValue>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Vec<AnyValue>> for Sequence {
    fn as_ref(&self) -> &Vec<AnyValue> {
        self
    }
}
impl AsMut<Vec<AnyValue>> for Sequence {
    fn as_mut(&mut self) -> &mut Vec<AnyValue> {
        self
    }
}
impl AsRef<[AnyValue]> for Sequence {
    fn as_ref(&self) -> &[AnyValue] {
        self
    }
}
impl AsMut<[AnyValue]> for Sequence {
    fn as_mut(&mut self) -> &mut [AnyValue] {
        self
    }
}
impl From<Sequence> for Vec<AnyValue> {
    fn from(value: Sequence) -> Self {
        value.inner
    }
}
impl<const N: usize> TryFrom<Sequence> for [AnyValue; N] {
    type Error = Error;
    fn try_from(value: Sequence) -> Result<Self> {
        let len = value.len();
        Self::try_from(value.inner)
            .map_err(|_| etype!("Cannot represent {} elements as `[AnyType; {}]`", len, N))
    }
}
impl IntoIterator for Sequence {
    type Item = AnyValue;
    type IntoIter = <Vec<AnyValue> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl FromIterator<AnyValue> for Sequence {
    fn from_iter<T: IntoIterator<Item = AnyValue>>(values: T) -> Self {
        Self { inner: values.into_iter().collect() }
    }
}
impl From<Sequence> for AnyValue {
    fn from(value: Sequence) -> Self {
        AnyValue::Sequence(value)
    }
}

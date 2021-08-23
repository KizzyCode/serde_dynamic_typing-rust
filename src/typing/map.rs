//! Implements a type unifier for map like native types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{
    collections::BTreeMap, convert::TryFrom, iter::FromIterator,
    ops::{ Deref, DerefMut }
};


/// A type unifier for key-value structures
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct Map {
    /// The wrapped value
    inner: BTreeMap<AnyValue, AnyValue>
}
impl From<BTreeMap<AnyValue, AnyValue>> for Map {
    fn from(value: BTreeMap<AnyValue, AnyValue>) -> Self {
        Self { inner: value }
    }
}
impl TryFrom<AnyValue> for Map {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Map(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to map", value))
        }
    }
}
impl Deref for Map {
    type Target = BTreeMap<AnyValue, AnyValue>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<BTreeMap<AnyValue, AnyValue>> for Map {
    fn as_ref(&self) -> &BTreeMap<AnyValue, AnyValue> {
        self
    }
}
impl AsMut<BTreeMap<AnyValue, AnyValue>> for Map {
    fn as_mut(&mut self) -> &mut BTreeMap<AnyValue, AnyValue> {
        self
    }
}
impl From<Map> for BTreeMap<AnyValue, AnyValue> {
    fn from(value: Map) -> Self {
        value.inner
    }
}
impl IntoIterator for Map {
    type Item = (AnyValue, AnyValue);
    type IntoIter = <BTreeMap<AnyValue, AnyValue> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl FromIterator<(AnyValue, AnyValue)> for Map {
    fn from_iter<T: IntoIterator<Item = (AnyValue, AnyValue)>>(values: T) -> Self {
        Self { inner: values.into_iter().collect() }
    }
}
impl From<Map> for AnyValue {
    fn from(value: Map) -> Self {
        AnyValue::Map(value)
    }
}

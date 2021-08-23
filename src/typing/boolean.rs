//! Implements a type unifier for boolean like native types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{
    convert::TryFrom,
    ops::{ Deref, DerefMut }
};


/// A type unifier for booleans
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Boolean {
    /// The wrapped value
    inner: bool
}
impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self { inner: value }
    }
}
impl TryFrom<AnyValue> for Boolean {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Bool(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to boolean", value))
        }
    }
}
impl Deref for Boolean {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Boolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<bool> for Boolean {
    fn as_ref(&self) -> &bool {
        self
    }
}
impl AsMut<bool> for Boolean {
    fn as_mut(&mut self) -> &mut bool {
        self
    }
}
impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.inner
    }
}
impl From<Boolean> for AnyValue {
    fn from(value: Boolean) -> Self {
        AnyValue::Bool(value)
    }
}

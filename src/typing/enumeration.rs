//! Implements a type unifier for enumerations

use crate::{
    error::{ Error, Result },
    typing::{ AnyValue, utf8_string::Utf8String }
};
use std::convert::TryFrom;


/// A type unifier for enumerations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Enumeration {
    /// The enum variant
    variant: Utf8String,
    /// The associated value
    value: Box<AnyValue>
}
impl Enumeration {
    /// Creates a new enum with the given variant
    pub fn new<T>(variant: T) -> Self where T: ToString {
        Self { variant: Utf8String::new(variant), value: Box::new(AnyValue::None) }
    }
    /// Creates a new enum with the given variant and an assoiated value
    pub fn with_value<T, V>(variant: T, value: V) -> Self where T: ToString, V: Into<AnyValue> {
        Self { variant: Utf8String::new(variant), value: Box::new(value.into()) }
    }

    /// The enum variant
    pub fn variant(&self) -> &String {
        &self.variant
    }
    /// The enum variant as mutable reference
    pub fn variant_mut(&mut self) -> &mut String {
        &mut self.variant
    }
    /// The associated value
    pub fn value(&self) -> &AnyValue {
        &self.value
    }
    /// The associated value as mutable reference
    pub fn value_mut(&mut self) -> &mut AnyValue {
        &mut self.value
    }

    /// Returns the underlying variant and the associated tuple
    pub fn into_inner(self) -> (Utf8String, AnyValue) {
        (self.variant, *self.value)
    }
}
impl TryFrom<AnyValue> for Enumeration {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Enum(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to enum", value))
        }
    }
}
impl From<Enumeration> for AnyValue {
    fn from(value: Enumeration) -> Self {
        AnyValue::Enum(value)
    }
}

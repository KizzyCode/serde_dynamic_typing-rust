//! Offers a simplified type model which can represent various data structures and allows dynamic runtime inspection

pub mod boolean;
pub mod bytes;
pub mod enumeration;
pub mod float;
pub mod sequence;
pub mod map;
pub mod integer;
pub mod utf8_string;

use crate::error::{ Error, Result };
use std::convert::TryFrom;
// Reexport types
pub use crate::typing::{
    boolean::Boolean, bytes::Bytes, enumeration::Enumeration, float::Float,
    sequence::Sequence, map::Map, integer::Integer, utf8_string::Utf8String
};


/// A simplified type model which can represent various data structures and allows dynamic runtime inspection
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum AnyValue {
    /// A boolean value
    Bool(Boolean),
    /// An integer value
    Integer(Integer),
    /// A floating point value
    Float(Float),
    /// An UTF-8 string value
    Utf8String(Utf8String),
    /// A binary vector
    Bytes(Bytes),
    /// A sequence
    Sequence(Sequence),
    /// A map
    Map(Map),
    /// An enum value
    Enum(Enumeration),
    /// A non-existant value
    None
}
impl From<()> for AnyValue {
    fn from(_unit: ()) -> Self {
        Self::None
    }
}
impl TryFrom<AnyValue> for () {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::None => Ok(()),
            value => Err(etype!("Cannot convert {:?} to unit", value))
        }
    }
}

//! Implements a type unifier for integer like native types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{ any, convert::TryFrom };


/// A type unifier for integers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Integer {
    /// An unsigned integer
    UnsignedInteger(u128),
    /// A signed integer
    SignedInteger(i128)
}
impl Default for Integer {
    fn default() -> Self {
        Self::UnsignedInteger(Default::default())
    }
}
impl TryFrom<AnyValue> for Integer {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Integer(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to integer", value))
        }
    }
}
impl From<Integer> for AnyValue {
    fn from(value: Integer) -> Self {
        AnyValue::Integer(value)
    }
}
macro_rules! impl_conversion {
    ($type:ty) => {
        impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                if let Ok(value) = u128::try_from(value) {
                    Self::UnsignedInteger(value)
                } else if let Ok(value) = i128::try_from(value) {
                    Self::SignedInteger(value)
                } else {
                    panic!("Cannot represent {} as `u128` nor as `i128`?!", value)
                }
            }
        }
        impl TryFrom<Integer> for $type {
            type Error = Error;
            fn try_from(value: Integer) -> Result<Self> {
                let type_name = any::type_name::<$type>();
                match value {
                    Integer::UnsignedInteger(value) => {
                        Self::try_from(value)
                            .map_err(|e| etype!("Cannot represent {} as `{}` ({})", value, type_name, e))
                    },
                    Integer::SignedInteger(value) => {
                        Self::try_from(value)
                            .map_err(|e| etype!("Cannot represent {} as `{}` ({})", value, type_name, e))
                    }
                }
            }
        }    
    };
}
impl_conversion!(u8);
impl_conversion!(u16);
impl_conversion!(u32);
impl_conversion!(u64);
impl_conversion!(u128);
impl_conversion!(usize);
impl_conversion!(i8);
impl_conversion!(i16);
impl_conversion!(i32);
impl_conversion!(i64);
impl_conversion!(i128);
impl_conversion!(isize);

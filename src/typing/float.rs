//! Implements a type unifier for float like native types

use crate::{
    typing::AnyValue,
    error::{ Error, Result }
};
use std::{
    cmp::Ordering, convert::TryFrom,
    ops::{ Deref, DerefMut }
};


/// A type unifier for floats
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct Float {
    /// The wrapped value
    inner: f64
}
impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self { inner: value }
    }
}
impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self { inner: value.into() }
    }
}
impl TryFrom<AnyValue> for Float {
    type Error = Error;
    fn try_from(value: AnyValue) -> Result<Self> {
        match value {
            AnyValue::Float(value) => Ok(value),
            value => Err(etype!("Cannot convert {:?} to float", value))
        }
    }
}
impl Deref for Float {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<f64> for Float {
    fn as_ref(&self) -> &f64 {
        self
    }
}
impl AsMut<f64> for Float {
    fn as_mut(&mut self) -> &mut f64 {
        self
    }
}
impl Eq for Float {
    /* Nothing to see here */
}
impl Ord for Float {
    /// Implement total ordering using the unstable stdlib implementation (see also 72599)
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left = self.to_bits() as i64;
        let mut right = other.to_bits() as i64;

        // In case of negatives, flip all the bits except the sign
        // to achieve a similar layout as two's complement integers
        //
        // Why does this work? IEEE 754 floats consist of three fields:
        // Sign bit, exponent and mantissa. The set of exponent and mantissa
        // fields as a whole have the property that their bitwise order is
        // equal to the numeric magnitude where the magnitude is defined.
        // The magnitude is not normally defined on NaN values, but
        // IEEE 754 totalOrder defines the NaN values also to follow the
        // bitwise order. This leads to order explained in the doc comment.
        // However, the representation of magnitude is the same for negative
        // and positive numbers – only the sign bit is different.
        // To easily compare the floats as signed integers, we need to
        // flip the exponent and mantissa bits in case of negative numbers.
        // We effectively convert the numbers to "two's complement" form.
        //
        // To do the flipping, we construct a mask and XOR against it.
        // We branchlessly calculate an "all-ones except for the sign bit"
        // mask from negative-signed values: right shifting sign-extends
        // the integer, so we "fill" the mask with sign bits, and then
        // convert to unsigned to push one more zero bit.
        // On positive values, the mask is all zeros, so it's a no-op.
        left ^= (((left >> 63) as u64) >> 1) as i64;
        right ^= (((right >> 63) as u64) >> 1) as i64;

        left.cmp(&right)
    }
}
impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        value.inner
    }
}
impl From<Float> for AnyValue {
    fn from(value: Float) -> Self {
        AnyValue::Float(value)
    }
}

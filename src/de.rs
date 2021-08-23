use crate::{
    error::{ Result, Error },
    typing::{ AnyValue, Boolean, Bytes, Enumeration, Float, Integer, Map, Sequence, Utf8String }
};
use serde::{
    Deserialize, Deserializer as _,
    de::{ self, DeserializeSeed, Visitor, SeqAccess, MapAccess, VariantAccess, EnumAccess }
};
use std::convert::{ TryFrom, TryInto };


/// A sequence reader
struct SequenceReader {
    /// The sequence objects
    values: <Sequence as IntoIterator>::IntoIter
}
impl SequenceReader {
    /// Creates a new sequence reader
    pub fn new(sequence: Sequence) -> Self {
        Self { values: sequence.into_iter() }
    }
}
impl<'de> SeqAccess<'de> for SequenceReader {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>> where T: DeserializeSeed<'de> {
        // Check if there are sequence objects left
        let value = match self.values.next() {
            Some(value) => value,
            None => return Ok(None)
        };

        // Deserialize the value
        let deserializer = Deserializer::new(value);
        let deserialized = seed.deserialize(deserializer)?;
        Ok(Some(deserialized))
    }
}


/// A key value reader
struct KeyValueReader {
    /// The next pending value
    pending_value: Option<AnyValue>,
    /// The underlying map to read from
    values: <Map as IntoIterator>::IntoIter
}
impl KeyValueReader {
    /// Creates a new map reader
    pub fn new(map: Map) -> Self {
        Self { pending_value: None, values: map.into_iter() }
    }
}
impl<'de> MapAccess<'de> for KeyValueReader {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>> where K: DeserializeSeed<'de> {
        // Get the next key
        assert!(self.pending_value.is_none(), "There is already a pending value?!");
        let (key, value) = match self.values.next() {
            Some((key, value)) => (key, value),
            None => return Ok(None)
        };
        self.pending_value = Some(value);

        // Get the next key
        let deserializer = Deserializer::new(key);
        let deserialized = seed.deserialize(deserializer)?;
        Ok(Some(deserialized))
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value> where V: DeserializeSeed<'de> {
        // Get the next value and deserialize it
        let value = self.pending_value.take().expect("There is no pending value yet?!");
        let deserializer = Deserializer::new(value);
        seed.deserialize(deserializer)
    }
}


/// An enum variant reader
struct EnumValueReader {
    /// The enum value
    value: AnyValue
}
impl EnumValueReader {
    /// Creates a new enum value reader that processes `value`
    pub const fn new(value: AnyValue) -> Self {
        Self { value }
    }
}
impl<'de> VariantAccess<'de> for EnumValueReader {
    type Error = Error;

    fn unit_variant(self) -> Result {
        match self.value {
            AnyValue::None => Ok(()),
            value => Err(etype!("Cannot deserialize {:?} as unit variant", value))
        }
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value> where T: DeserializeSeed<'de> {
        let deserializer = Deserializer::new(self.value);
        seed.deserialize(deserializer)
    }
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let deserializer = Deserializer::new(self.value);
        deserializer.deserialize_tuple(len, visitor)
    }
    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let deserializer = Deserializer::new(self.value);
        deserializer.deserialize_struct("/* unused */", fields, visitor)
    }
}


/// An enum variant reader
struct EnumReader {
    /// The enum itself
    enumeration: Enumeration
}
impl EnumReader {
    /// Creates a new enum reader that processes `enumeration`
    pub const fn new(enumeration: Enumeration) -> Self {
        Self { enumeration }
    }
}
impl<'de> EnumAccess<'de> for EnumReader {
    type Error = Error;
    type Variant = EnumValueReader;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)> where V: DeserializeSeed<'de> {
        // Destructure the enum
        let (variant, value) = self.enumeration.into_inner();
        let variant = AnyValue::Utf8String(variant);

        // Deserialize the variant and 
        let deserializer = Deserializer::new(variant);
        let variant = seed.deserialize(deserializer)?;
        Ok((variant, EnumValueReader::new(value)))
    }
}


/// A deserializer for `AnyTyped`
struct Deserializer {
    value: AnyValue
}
impl Deserializer {
    /// Creates a new deserializer that processes `value`
    pub const fn new(value: AnyValue) -> Self {
        Self { value }
    }
}
impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        match self.value {
            AnyValue::Bool(_) => self.deserialize_bool(visitor),
            AnyValue::Integer(Integer::UnsignedInteger(_)) => self.deserialize_u128(visitor),
            AnyValue::Integer(Integer::SignedInteger(_)) => self.deserialize_i128(visitor),
            AnyValue::Float(_) => self.deserialize_f64(visitor),
            AnyValue::Utf8String(_) => self.deserialize_string(visitor),
            AnyValue::Bytes(_) => self.deserialize_byte_buf(visitor),
            AnyValue::Sequence(_) => self.deserialize_seq(visitor),
            AnyValue::Map(_) => self.deserialize_map(visitor),
            AnyValue::Enum(_) => self.deserialize_enum("/* unused */", &[/* unused */], visitor),
            AnyValue::None => self.deserialize_unit(visitor)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Boolean::try_from(self.value)?;
        visitor.visit_bool(value.into())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_i8(value.try_into()?)
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_i16(value.try_into()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_i32(value.try_into()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_i64(value.try_into()?)
    }
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value> where V:Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_i128(value.try_into()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_u8(value.try_into()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_u16(value.try_into()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_u32(value.try_into()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_u64(value.try_into()?)
    }
    fn deserialize_u128<V>(self, visitor:V) -> Result<V::Value> where V:Visitor<'de> {
        let value = Integer::try_from(self.value)?;
        visitor.visit_u128(value.try_into()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Float::try_from(self.value)?;
        visitor.visit_f32(f64::from(value) as f32)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Float::try_from(self.value)?;
        visitor.visit_f64(value.into())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Utf8String::try_from(self.value)?;
        visitor.visit_char(value.try_into()?)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Utf8String::try_from(self.value)?;
        visitor.visit_str(&value)
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Utf8String::try_from(self.value)?;
        visitor.visit_string(value.into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Bytes::try_from(self.value)?;
        visitor.visit_bytes(&value)
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Bytes::try_from(self.value)?;
        visitor.visit_byte_buf(value.into())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        match self.value {
            AnyValue::None => visitor.visit_none(),
            _ => visitor.visit_some(self)
        }
    }
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let _unit = <()>::try_from(self.value)?;
        visitor.visit_unit()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let _unit = <()>::try_from(self.value)?;
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Sequence::try_from(self.value)?;
        visitor.visit_seq(SequenceReader::new(value))
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Sequence::try_from(self.value)?;
        visitor.visit_seq(SequenceReader::new(value))
    }
    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        let value = Sequence::try_from(self.value)?;
        visitor.visit_seq(SequenceReader::new(value))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Map::try_from(self.value)?;
        visitor.visit_map(KeyValueReader::new(value))
    }
    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V)
        -> Result<V::Value> where V: Visitor<'de>
    {
        let value = Map::try_from(self.value)?;
        visitor.visit_map(KeyValueReader::new(value))
    }
    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V)
        -> Result<V::Value> where V: Visitor<'de>
    {
        let value = Enumeration::try_from(self.value)?;
        visitor.visit_enum(EnumReader::new(value))
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let value = Utf8String::try_from(self.value)?;
        visitor.visit_string(value.into())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}


/// Deserializes a Rust value from a simplified data structure
pub fn from_typed<'a, T>(value: AnyValue) -> Result<T> where T: Deserialize<'a> {
    let deserializer = Deserializer::new(value);
    T::deserialize(deserializer)
}
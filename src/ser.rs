use crate::{
    error::{ Result, Error },
    typing::{ AnyValue, Enumeration, Sequence, Map }
};
use serde::{
    Serialize,
    ser::{
        self,
        SerializeSeq, SerializeStruct, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant, SerializeMap, SerializeStructVariant
    }
};


/// A sequence writer
struct SequenceWriter {
    /// The elements to write
    values: Sequence,
    /// An on-completion-handler
    on_end: Box<dyn FnOnce(AnyValue) -> AnyValue>
}
impl SequenceWriter {
    /// Creates a new sequence writer
    pub fn new() -> Self {
        let on_end = Box::new(|sequence| sequence);
        Self { values: Sequence::default(), on_end }
    }
    /// Creates a new sequence writer with a custom completion handler
    pub fn with_completion<T>(on_end: T) -> Self where T: FnOnce(AnyValue) -> AnyValue + 'static {
        let on_end = Box::new(on_end);
        Self { values: Sequence::default(), on_end }
    }
    
    /// Writes the next `value` to the internal buffer
    fn _serialize_element<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        let value = value.serialize(Serializer::new())?;
        self.values.push(value);
        Ok(())
    }
    /// Finalizes the sequence
    fn _end(self) -> Result<AnyValue> {
        let list = AnyValue::Sequence(self.values.into());
        Ok((self.on_end)(list))
    }
}
impl SerializeSeq for SequenceWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_element<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_element(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}
impl SerializeTuple for SequenceWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_element<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_element(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}
impl SerializeTupleStruct for SequenceWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_field<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_element(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}
impl SerializeTupleVariant for SequenceWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_field<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_element(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}


/// A key-value writer
struct KeyValueWriter {
    /// The pending key to insert
    pending_key: Option<AnyValue>,
    /// The map
    map: Map,
    /// An on-completion-handler
    on_end: Box<dyn FnOnce(AnyValue) -> AnyValue>
}
impl KeyValueWriter {
    /// Creates a new key-value writer
    pub fn new() -> Self {
        let on_end = Box::new(|map| map);
        Self { pending_key: None, map: Map::default(), on_end }
    }
    /// Creates a new sequence writer with a custom completion handler
    pub fn with_completion<T>(on_end: T) -> Self where T: FnOnce(AnyValue) -> AnyValue + 'static {
        let on_end = Box::new(on_end);
        Self { pending_key: None, map: Map::default(), on_end }
    }
    
    /// Writes the next `key` to the internal buffer
    fn _serialize_key<T>(&mut self, key: &T) -> Result where T: ?Sized + Serialize {
        assert!(self.pending_key.is_none(), "There is already a pending key?!");
        self.pending_key = Some(key.serialize(Serializer::new())?);
        Ok(())
    }
    /// Writes the associated `value` to the pending key
    fn _serialize_value<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        let key = self.pending_key.take().expect("There is no pending key yet?!");
        self.map.insert(key, value.serialize(Serializer::new())?);
        Ok(())
    }
    
    /// Finalizes the sequence
    fn _end(self) -> Result<AnyValue> {
        let map = AnyValue::Map(self.map);
        Ok((self.on_end)(map))
    }
}
impl SerializeMap for KeyValueWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_key<T>(&mut self, key: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_key(key)
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_value(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}
impl SerializeStructVariant for KeyValueWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_key(key)?;
        self._serialize_value(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}
impl SerializeStruct for KeyValueWriter {
    type Ok = AnyValue;
    type Error = Error;
    
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result where T: ?Sized + Serialize {
        self._serialize_key(key)?;
        self._serialize_value(value)
    }
    fn end(self) -> Result<Self::Ok> {
        self._end()
    }
}


/// A serializer for `AnyTyped`
struct Serializer;
impl Serializer {
    /// Creates a new serializer instance
    pub const fn new() -> Self {
        Self
    }
}
impl ser::Serializer for Serializer {
    type Ok = AnyValue;
    type Error = Error;
    
    type SerializeSeq = SequenceWriter;
    type SerializeTuple = SequenceWriter;
    type SerializeTupleStruct = SequenceWriter;
    type SerializeTupleVariant = SequenceWriter;
    type SerializeMap = KeyValueWriter;
    type SerializeStruct = KeyValueWriter;
    type SerializeStructVariant = KeyValueWriter;
    
    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        Ok(AnyValue::Bool(value.into()))
    }
    
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_i128(self, value: i128) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    
    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    fn serialize_u128(self, value: u128) -> Result<Self::Ok> {
        Ok(AnyValue::Integer(value.into()))
    }
    
    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        Ok(AnyValue::Float(value.into()))
    }
    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        Ok(AnyValue::Float(value.into()))
    }
    
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok(AnyValue::Utf8String(value.into()))
    }
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(AnyValue::Utf8String(value.into()))
    }
    
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        Ok(AnyValue::Bytes(value.into()))
    }
    
    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(AnyValue::None)
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok> where T: ?Sized + Serialize {
        value.serialize(self)
    }
    
    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(AnyValue::None)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(AnyValue::None)
    }
    
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str)
        -> Result<Self::Ok>
    {
        Ok(AnyValue::Enum(Enumeration::new(variant)))
    }
    
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T)
        -> Result<Self::Ok> where T: ?Sized + Serialize
    {
        let value = value.serialize(self)?;
        Ok(AnyValue::Enum(Enumeration::with_value(variant, value)))
    }
    
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SequenceWriter::new())
    }
    //noinspection RsUnresolvedReference
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }
    //noinspection RsUnresolvedReference
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize)
        -> Result<Self::SerializeTupleVariant>
    {
        let on_end = move |value| {
            let enumeration = Enumeration::with_value(variant, value);
            AnyValue::Enum(enumeration)
        };
        Ok(SequenceWriter::with_completion(on_end))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(KeyValueWriter::new())
    }
    //noinspection RsUnresolvedReference
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize)
        -> Result<Self::SerializeStructVariant>
    {
        let on_end = move |value| {
            let enumeration = Enumeration::with_value(variant, value);
            AnyValue::Enum(enumeration)
        };
        Ok(KeyValueWriter::with_completion(on_end))
    }
}


/// Serializes a Rust value into a simplified data structure
pub fn to_typed<T>(value: &T) -> Result<AnyValue> where T: ?Sized + Serialize {
    value.serialize(Serializer::new())
}

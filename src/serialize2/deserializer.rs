use std;
use serde;
use super::{ReadStream, DeserializeError};

pub struct Deserializer<R:ReadStream> {
   r: R,
}

impl <R:ReadStream> Deserializer<R> {
   pub fn new(r:R) -> Self {
      Self { r:r }
   }
   pub fn into_inner(self) -> R {
      self.r
   }

   fn deserialize_varint(&mut self, v:&mut u64) -> Result<usize, ::std::io::Error> {
      let mut x:u8 = 0;
      try!(self.r.read_u8(&mut x));
      if x < 253 {
         *v = x as u64;
         Ok(1)
      } else if x == 253 {
         let mut y:u16 = 0;
         try!(self.r.read_u16le(&mut y));
         *v = y as u64;
         Ok(3)
      } else if x == 254 {
         let mut y:u32 = 0;
         try!(self.r.read_u32le(&mut y));
         *v = y as u64;
         Ok(5)
      } else {
         try!(self.r.read_u64le(v));
         Ok(9)
      }
   }
}

impl <'a, R:ReadStream> serde::de::Deserializer for &'a mut Deserializer<R> {
   type Error = super::DeserializeError;

   fn deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }

   /// Hint that the `Deserialize` type is expecting a `bool` value.
   fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:u8 = 0;
      try!(self.r.read_u8(&mut x));
      visitor.visit_bool(x == 1)
      // std::error::Error から DeserializeError への変換は不要?
      //visitor.visit_bool(x == 1).map_err(|e:std::error::Error|{DeserializeError::custom(e)})
   }
   
   fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:u8 = 0;
      try!(self.r.read_u8(&mut x));
      visitor.visit_u8(x)
   }

   fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:u16 = 0;
      try!(self.r.read_u16le(&mut x));
      visitor.visit_u16(x)
   }
       
   fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:u32 = 0;
      try!(self.r.read_u32le(&mut x));
      visitor.visit_u32(x)
   }
      
   fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:u64 = 0;
      try!(self.r.read_u64le(&mut x));
      visitor.visit_u64(x)
   }
      
   fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:i8 = 0;
      try!(self.r.read_i8(&mut x));
      visitor.visit_i8(x)
   }
      
   fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:i16 = 0;
      try!(self.r.read_i16le(&mut x));
      visitor.visit_i16(x)
   }
      
   fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:i32 = 0;
      try!(self.r.read_i32le(&mut x));
      visitor.visit_i32(x)
   }

   fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      let mut x:i64 = 0;
      try!(self.r.read_i64le(&mut x));
      visitor.visit_i64(x)
   }
       
   fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
   fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
   fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }

   fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
   fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
      
    /// Hint that the `Deserialize` type is expecting a byte array and does not
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `serde::de::Visitor` would benefit from taking ownership of `Vec<u8>` data,
    /// indicate this to the `Deserializer` by using `deserialize_byte_buf`
    /// instead.
   fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      self.deserialize_
      deserialize_error!("not implemented")
   }
   fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
      

    /// Hint that the `Deserialize` type is expecting an optional value.
    ///
    /// This allows deserializers that encode an optional value as a nullable
    /// value to convert the null value into `None` and a regular value into
    /// `Some(value)`.
   fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
      

    /// Hint that the `Deserialize` type is expecting a unit value.
   fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
      

    /// Hint that the `Deserialize` type is expecting a unit struct with a
    /// particular name.
    fn deserialize_unit_struct<V>(self,
                                  name: &'static str,
                                  visitor: V)
                                  -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a newtype struct with a
    /// particular name.
    fn deserialize_newtype_struct<V>(self,
                                     name: &'static str,
                                     visitor: V)
                                     -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a sequence of values.
   fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a sequence of values and
    /// knows how many values there are without looking at the serialized data.
    fn deserialize_seq_fixed_size<V>(self,
                                     len: usize,
                                     visitor: V)
                                     -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a tuple value with a
    /// particular number of elements.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a tuple struct with a
    /// particular name and number of fields.
    fn deserialize_tuple_struct<V>(self,
                                   name: &'static str,
                                   len: usize,
                                   visitor: V)
                                   -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a map of key-value pairs.
   fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a struct with a particular
    /// name and fields.
    fn deserialize_struct<V>(self,
                             name: &'static str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting the name of a struct
    /// field.
    fn deserialize_struct_field<V>(self, visitor: V) -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn deserialize_enum<V>(self,
                           name: &'static str,
                           variants: &'static [&'static str],
                           visitor: V)
                           -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    ///
    /// Deserializers for non-self-describing formats may not support this mode.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor
   {
      deserialize_error!("not implemented")
   }
       
}

use std;
use serde;
use super::{ReadStream, DeserializeError};

pub struct Deserializer<R> where R: ReadStream {
   r: R,
   read_bytes: usize,
}

impl <R:ReadStream> Deserializer<R> {
   pub fn new(r:R) -> Self {
      Self { r:r, read_bytes:0 }
   }
   pub fn into_inner(self) -> R {
      self.r
   }

   fn deserialize_varint(&mut self) -> Result<usize, ::std::io::Error> {
      let mut x:u8 = 0;
      self.read_bytes += try!(self.r.read_u8(&mut x));
      if x < 253 {
         Ok(x as usize)
      } else if x == 253 {
         let mut y:u16 = 0;
         self.read_bytes += try!(self.r.read_u16le(&mut y));
         Ok(y as usize)
      } else if x == 254 {
         let mut y:u32 = 0;
         self.read_bytes += try!(self.r.read_u32le(&mut y));
         Ok(y as usize)
      } else {
         let mut y:u64 = 0;
         self.read_bytes += try!(self.r.read_u64le(&mut y));
         Ok(y as usize)
      }
   }
}

impl <'a, 'de, R:ReadStream> serde::de::Deserializer<'de> for &'a mut Deserializer<R> {
   type Error = super::DeserializeError;

   fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }

   fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:u8 = 0;
      self.read_bytes += try!(self.r.read_u8(&mut x));
      visitor.visit_bool(x == 1)
      // std::error::Error から DeserializeError への変換は不要?
      //visitor.visit_bool(x == 1).map_err(|e:std::error::Error|{DeserializeError::custom(e)})
   }
   
   fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:u8 = 0;
      self.read_bytes += try!(self.r.read_u8(&mut x));
      visitor.visit_u8(x)
   }

   fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:u16 = 0;
      self.read_bytes += try!(self.r.read_u16le(&mut x));
      visitor.visit_u16(x)
   }
       
   fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:u32 = 0;
      self.read_bytes += try!(self.r.read_u32le(&mut x));
      visitor.visit_u32(x)
   }
      
   fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:u64 = 0;
      self.read_bytes += try!(self.r.read_u64le(&mut x));
      visitor.visit_u64(x)
   }
      
   fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:i8 = 0;
      self.read_bytes += try!(self.r.read_i8(&mut x));
      visitor.visit_i8(x)
   }
      
   fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:i16 = 0;
      self.read_bytes += try!(self.r.read_i16le(&mut x));
      visitor.visit_i16(x)
   }
      
   fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:i32 = 0;
      self.read_bytes += try!(self.r.read_i32le(&mut x));
      visitor.visit_i32(x)
   }

   fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let mut x:i64 = 0;
      self.read_bytes += try!(self.r.read_i64le(&mut x));
      visitor.visit_i64(x)
   }
       
   fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }
   fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }
   fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }

   fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let len = try!(self.deserialize_varint());
      let mut buf:Vec<u8> = vec![0; len];
      let _ = try!(self.r.read_exact(buf.as_mut_slice()));
      self.read_bytes += len;
      let s = try!(std::str::from_utf8(&buf[0..]));
      visitor.visit_str(s)
   }
   fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let len = try!(self.deserialize_varint());
      let mut buf:Vec<u8> = vec![0; len];
      let _ = try!(self.r.read_exact(buf.as_mut_slice()));
      self.read_bytes += len;
      let s = try!(String::from_utf8(buf));
      visitor.visit_string(s)
   }
      
   fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      // Copy bytes to continuous buffer because a ReadStream may not have uncontinuous internal buffer.
      let len = try!(self.deserialize_varint());
      let mut buf:Vec<u8> = vec![0; len];
      let _ = try!(self.r.read_exact(buf.as_mut_slice()));
      self.read_bytes += len;
      visitor.visit_bytes(buf.as_slice())
   }
   fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      let len = try!(self.deserialize_varint());
      let mut buf:Vec<u8> = vec![0; len];
      let _ = try!(self.r.read_exact(buf.as_mut_slice()));
      self.read_bytes += len;
      visitor.visit_byte_buf(buf)
   }
      
    /// Hint that the `Deserialize` type is expecting an optional value.
    ///
    /// This allows deserializers that encode an optional value as a nullable
    /// value to convert the null value into `None` and a regular value into
    /// `Some(value)`.
   fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }
      
    /// Hint that the `Deserialize` type is expecting a unit value.
   fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }
      

    /// Hint that the `Deserialize` type is expecting a unit struct with a
    /// particular name.
    fn deserialize_unit_struct<V>(self,
                                  name: &'static str,
                                  visitor: V)
                                  -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor<'de>
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a newtype struct with a
    /// particular name.
    fn deserialize_newtype_struct<V>(self,
                                     name: &'static str,
                                     visitor: V)
                                     -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor<'de>
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a sequence of values.
   fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de>
   {
      let len = try!(self.deserialize_varint());
      let v = try!(visitor.visit_seq(DeserializerSeqAccess {
         de:self,
         len:Some(len as usize),
      }));
      Ok(v)
   }
       
    /// Hint that the `Deserialize` type is expecting a tuple value with a
    /// particular number of elements.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor<'de>
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
      where V: serde::de::Visitor<'de>
   {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a map of key-value pairs.
   fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
      deserialize_error!("not implemented")
   }
       

    /// Hint that the `Deserialize` type is expecting a struct with a particular
    /// name and fields.
    fn deserialize_struct<V>(self,
                             name: &'static str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor<'de>
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
      where V: serde::de::Visitor<'de>
   {
      deserialize_error!("not implemented")
   }
       
    /// Hint that the `Deserialize` type is expecting the name of a struct
    /// field or the discriminant of an enum variant.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>
   {
      deserialize_error!("not implemented")
   }

    /// Hint that the `Deserialize` type needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    ///
    /// Deserializers for non-self-describing formats may not support this mode.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
      where V: serde::de::Visitor<'de>
   {
      deserialize_error!("not implemented")
   }
       
}


struct DeserializerSeqAccess<'a, R: 'a + ReadStream> {
   de: &'a mut Deserializer<R>,
   len: Option<usize>,
}

impl<'a, 'de, R:ReadStream> serde::de::SeqAccess<'de> for DeserializerSeqAccess<'a, R>
{
   type Error = super::DeserializeError;

   fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
      where T: serde::de::DeserializeSeed<'de>
   {
      self.len = self.len.map(|len| len.saturating_sub(1));
      seed.deserialize(&mut *self.de).map(Some)
   }

   fn size_hint(&self) -> Option<usize> {
      self.len
   }
}

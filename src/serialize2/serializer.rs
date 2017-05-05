use serde::ser;
use super::{WriteStream, VarInt, SerializeError, DeserializeError};

mod result {
   pub type Ok    = usize;
   pub type Error = super::SerializeError;
   pub type Result = ::std::result::Result<Ok,Error>;
}

pub struct Serializer<W:WriteStream> {
   w: W,
}

impl <W:WriteStream> Serializer<W> {
   pub fn new(w:W) -> Self {
      Self { w:w }
   }
   pub fn into_inner(self) -> W {
      self.w
   }
}

impl <'a, W:WriteStream> ser::Serializer for &'a mut Serializer<W> {
   type Ok    = result::Ok;
   type Error = result::Error;
   type SerializeSeq = Compound<'a, W>;
   type SerializeTuple = Compound<'a, W>;
   type SerializeTupleStruct = Compound<'a, W>;
   type SerializeTupleVariant = Compound<'a, W>;
   type SerializeMap = Compound<'a, W>;
   type SerializeStruct = Compound<'a, W>;
   type SerializeStructVariant = Compound<'a, W>;
   
   fn serialize_bool(self, v:bool) -> result::Result {
      let v = if v {1u8} else {0u8};
      let n = self.w.write_u8(v)?;
      Ok(n)
   }
   fn serialize_i8(self, v:i8) -> result::Result {
      let n = self.w.write_i8(v)?;
      Ok(n)
   }
   fn serialize_i16(self, v:i16) -> result::Result {
      let n = self.w.write_i16le(v)?;
      Ok(n)
   }
   fn serialize_i32(self, v:i32) -> result::Result {
      let n = self.w.write_i32le(v)?;
      Ok(n)
   }
   fn serialize_i64(self, v:i64) -> result::Result {
      let n = self.w.write_i64le(v)?;
      Ok(n)
   }
   fn serialize_u8(self, v:u8) -> result::Result {
      let n = self.w.write_u8(v)?;
      Ok(n)
   }
   fn serialize_u16(self, v:u16) -> result::Result {
      let n = self.w.write_u16le(v)?;
      Ok(n)
   }
   fn serialize_u32(self, v:u32) -> result::Result {
      let n = self.w.write_u32le(v)?;
      Ok(n)
   }
   fn serialize_u64(self, v:u64) -> result::Result {
      let n = self.w.write_u64le(v)?;
      Ok(n)
   }
   fn serialize_f32(self, _v:f32) -> result::Result {
      serialize_error!("not implemented")
   }
   fn serialize_f64(self, _v:f64) -> result::Result {
      serialize_error!("not implemented")
   }
   fn serialize_bytes(self, v:&[u8]) -> result::Result {
      let n = self.w.write(v)?;
      Ok(n)
   }
   fn serialize_char(self, _v:char) -> result::Result {
      serialize_error!("not implemented")
   }
   fn serialize_str(self, _v:&str) -> result::Result {
      serialize_error!("not implemented")
   }
   fn serialize_none(self) -> result::Result {
      serialize_error!("not implemented")
   }
   fn serialize_some<T: ?Sized + ser::Serialize>(self, _v:&T) -> result::Result {
      serialize_error!("not implemented")
   }
   fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
   fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
   fn serialize_unit_variant(
      self,
      _name: &'static str,
      _variant_index: usize,
      _variant: &'static str
   ) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
   fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
      self,
      _name: &'static str,
      _value: &T
   ) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
   fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
      self,
      _name: &'static str,
      _variant_index: usize,
      _variant: &'static str,
      _value: &T
   ) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
   fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      use serde::ser::SerializeSeq;
      let mut r = Compound::new(self, 0);
      if let Some(size) = len {
         try!(r.serialize_element(&VarInt(size as u64)));
      }
      Ok(r)
   }
   
   fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq, Self::Error> {
      use serde::ser::SerializeSeq;
      let mut r = Compound::new(self, 0);
      r.serialize_element(&VarInt(size as u64));
      Ok(r)
   }
   
   fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      use serde::ser::SerializeTuple;
      let mut r = Compound::new(self, 0);
      Ok(r)
   }
   fn serialize_tuple_struct(
      self,
      _name: &'static str,
      _len: usize
   ) -> Result<Self::SerializeTupleStruct, Self::Error> {
      serialize_error!("not implemented")
   }
   fn serialize_tuple_variant(
      self,
      _name: &'static str,
      _variant_index: usize,
      _variant: &'static str,
      _len: usize
   ) -> Result<Self::SerializeTupleVariant, Self::Error>
   {
      serialize_error!("not implemented")
   }
   fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      serialize_error!("not implemented")
   }
      
   fn serialize_struct(
      self,
      _name: &'static str,
      _len: usize
   ) -> Result<Self::SerializeStruct, Self::Error> {
      serialize_error!("not implemented")
   }
      
   fn serialize_struct_variant(
      self,
      _name: &'static str,
      _variant_index: usize,
      _variant: &'static str,
      _len: usize
   ) -> Result<Self::SerializeStructVariant, Self::Error> {
      serialize_error!("not implemented")
   }
}

#[doc(hidden)]
pub struct Compound<'a, W:'a + WriteStream> {
   ser: &'a mut Serializer<W>,
   size: usize,
}
impl <'a, W:WriteStream> Compound<'a, W> {
   pub fn new(ser:&'a mut Serializer<W>, size:usize) -> Self {
      Self {
         ser: ser,
         size: size
      }
   }
}

impl <'a, W:WriteStream> ser::SerializeSeq for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;
   
   fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
      self.size += try!(value.serialize(&mut *self.ser));
      Ok(())
   }
   
   fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(self.size)
   }
}

impl <'a, W:WriteStream> ser::SerializeTuple for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;

   fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
      self.size += try!(value.serialize(&mut *self.ser));
      Ok(())
   }

   fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(self.size)
   }
}

impl <'a, W:WriteStream> ser::SerializeTupleStruct for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;

   fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
      serialize_error!("not implemented")
   }
   fn end(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
}

impl <'a, W:WriteStream> ser::SerializeTupleVariant for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;
   fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
      serialize_error!("not implemented")
   }
   fn end(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
}

impl <'a, W:WriteStream> ser::SerializeMap for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;

   fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
      serialize_error!("not implemented")
   }
      
   fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
      serialize_error!("not implemented")
   }
      

   /*
   fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(&mut self,
                                                                     key: &K,
                                                                     value: &V)
                                                                     -> Result<(), Self::Error>;
    */

    fn end(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
}

impl <'a, W:WriteStream> ser::SerializeStruct for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;
   fn serialize_field<T: ?Sized + ser::Serialize>(
      &mut self,
      _key: &'static str,
      _value: &T
   ) -> Result<(), Self::Error> {
      serialize_error!("not implemented")
   }
   
   fn end(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
}

impl <'a, W:WriteStream> ser::SerializeStructVariant for Compound<'a, W> {
   type Ok    = result::Ok;
   type Error = result::Error;
   fn serialize_field<T: ?Sized + ser::Serialize>(
      &mut self,
      _key: &'static str,
      _value: &T
   ) -> Result<(), Self::Error> {
      serialize_error!("not implemented")
   }

   fn end(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented")
   }
}


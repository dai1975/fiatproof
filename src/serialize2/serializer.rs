use serde::ser;
use super::{WriteStream, VarInt};

pub struct Serializer<W:WriteStream> {
   w: W,
   tmp_size: usize,
}

impl <W:WriteStream> Serializer<W> {
   pub fn new(w:W) -> Self {
      Self { w:w }
   }
   pub fn into_inner(self) -> W {
      self.w
   }
}

impl <'a, W:WriteStream> ser::Serializer
   for &'a mut Serializer<W>
{
   type Ok    = usize;
   type Error = ::Error;
   type SerializeSeq = ser::SerializeSeq;
   type SerializeTuple = ser::SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
   type SerializeTupleStruct = ser::SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
   type SerializeTupleVariant = ser::SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>;
   type SerializeMap = ser::SerializeMap<Ok = Self::Ok, Error = Self::Error>;
   type SerializeStruct = ser::SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
   type SerializeStructVariant = ser::SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>;
   
   fn serialize_bool(self, v:bool) -> ::Result<usize> {
      let v = if v {1u8} else {0u8};
      let n = self.w.write_u8(v)?;
      Ok(n)
   }
   fn serialize_i8(self, v:i8) -> ::Result<usize> {
      let n = self.w.write_i8(v)?;
      Ok(n)
   }
   fn serialize_i16(self, v:i16) -> ::Result<usize> {
      let n = self.w.write_i16le(v)?;
      Ok(n)
   }
   fn serialize_i32(self, v:i32) -> ::Result<usize> {
      let n = self.w.write_i16le(v)?;
      Ok(n)
   }
   fn serialize_i64(self, v:i64) -> ::Result<usize> {
      let n = self.w.write_i64le(v)?;
      Ok(n)
   }
   fn serialize_u8(self, v:u8) -> ::Result<usize> {
      let n = self.w.write_u8(v)?;
      Ok(n)
   }
   fn serialize_u16(self, v:u16) -> ::Result<usize> {
      let n = self.w.write_u16le(v)?;
      Ok(n)
   }
   fn serialize_u32(self, v:u32) -> ::Result<usize> {
      let n = self.w.write_u16le(v)?;
      Ok(n)
   }
   fn serialize_u64(self, v:u64) -> ::Result<usize> {
      let n = self.w.write_u64le(v)?;
      Ok(n)
   }
   fn serialize_bytes(self, v:&[u8]) -> ::Result<usize> {
      let n = self.w.write(v)?;
      Ok(n)
   }
   fn serialize_char(self, v:char) -> ::Result<usize> {
      serialize_error!("not implemented");
   }
   fn serialize_str(self, v:&str) -> ::Result<usize> {
      serialize_error!("not implemented");
   }
   fn serialize_none(self) -> ::Result<usize> {
      serialize_error!("not implemented");
   }
   fn serialize_some<T: ?Sized + ser::Serialize>(self, v:&T) -> ::Result<usize> {
      serialize_error!("not implemented");
   }
   fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_unit_variant(self,
                             name: &'static str,
                             variant_index: usize,
                             variant: &'static str)
                             -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self,
                                                      name: &'static str,
                                                      value: &T)
                                                      -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(self,
                                                       name: &'static str,
                                                       variant_index: usize,
                                                       variant: &'static str,
                                                       value: &T)
                                                       -> Result<Self::Ok, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      self.tmp_size = 0;
      let r = self as Self::SerializeSeq;
      if let Some(size) = len {
         r.serialize_element(VarInt(size as u64));
      }
      Ok(r)
   }
   
   fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq, Self::Error> {
      self.tmp_size = 0;
      let r = self as Self::SerializeSeq;
      r.serialize_element(VarInt(size as u64));
      Ok(r)
   }
   
   fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      self.tmp_size = 0;
      Ok(self)
   }
   fn serialize_tuple_struct(self,
                             name: &'static str,
                             len: usize)
                             -> Result<Self::SerializeTupleStruct, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_tuple_variant(self,
                              name: &'static str,
                              variant_index: usize,
                              variant: &'static str,
                              len: usize)
                              -> Result<Self::SerializeTupleVariant, Self::Error> {
      serialize_error!("not implemented");
   }
   fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      serialize_error!("not implemented");
   }
      
   fn serialize_struct(self,
                       name: &'static str,
                       len: usize)
                       -> Result<Self::SerializeStruct, Self::Error> {
      serialize_error!("not implemented");
   }
      
   fn serialize_struct_variant(self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize)
                               -> Result<Self::SerializeStructVariant, Self::Error> {
      serialize_error!("not implemented");
   }
}


impl <W:WriteStream> ser::SerializeSeq
   for &'a mut Serializer<W>
{
   type Ok    = Serializer::Ok;
   type Error = Serializer::Error;
   
   fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
      self.tmp_size += value.serialize(&mut **self)?;
      Ok(())
   }
   
   fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(self.tmp_size)
   }
}

impl <W:WriteStream> ser::SerializeTuple
   for &'a mut Serializer<W>
{
   type Ok    = Serializer::Ok;
   type Error = Serializer::Error;

   fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
      self.tmp_size += value.serialize(&mut **self)?;
      Ok(())
   }

   fn end(self) -> Result<Self::Ok, Self::Error> {
   }
}
   

/*
   fn serialize_varint(&mut self, w:&mut WriteStream, _m:&Media, v:u64) -> ::Result<usize> {
      if v < 253 {
         try!(w.write_u8(v as u8));
         Ok(1)
      } else if v <= 0xFFFF {
         try!(w.write_u8(253u8));
         try!(w.write_u16le(v as u16));
         Ok(3)
      } else if v <= 0xFFFFFFFF {
         try!(w.write_u8(254u8));
         try!(w.write_u32le(v as u32));
         Ok(5)
      } else {
         try!(w.write_u8(255u8));
         try!(w.write_u64le(v));
         Ok(9)
      }
   }
   fn serialize_array_u8(&mut self, w:&mut WriteStream, _m:&Media, v:&[u8]) -> ::Result<usize> {
      try!(w.write(v));
      Ok(v.len())
   }
   fn serialize_sequence_u8(&mut self, w:&mut WriteStream, m:&Media, v:&[u8]) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.serialize_varint(w, m, v.len() as u64));
      try!(w.write(v));
      r += v.len();
      Ok(r)
   }
*/



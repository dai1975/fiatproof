use serde::ser;
use super::{Medium, Endian, WriteStream};

pub struct Serializer<W:WriteStream, E = Endian::Little> {
   w: W,
}

impl <W:WriteStream, E> Serializer<W,E> {
   pub fn new(w:W) -> Self {
      Self { w:w }
   }
   pub fn into_inner(self) -> W {
      self.w
   }
}

impl <'a, W:WriteStream> ser::Serializer for &'a mut Serializer<W, E=Endian::Little> {
   type Ok    = usize;
   type Error = ::Error;
   
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
      srialize_error("not implemented");
   }
   fn serialize_str(self, v:&str) -> ::Result<usize> {
      srialize_error("not implemented");
   }
   fn serialize_none(self) -> ::Result<usize> {
      srialize_error("not implemented");
   }
   fn serialize_some<T: ?Sized + ser::Serialize>(self, v:&T) -> ::Result<usize> {
      srialize_error("not implemented");
   }
}


pub struct Serializer<W:WriteStream> {
   w: W,
   media: Media,
}

impl <W:WriteStream> Serializer<W> {
   pub fn new(w:W, m:&str) -> Self {
      Self { w:w, media: Medium::new(m) }
   }
   pub fn into_inner(self) -> W {
      self.w
   }
   pub fn medium(&self) -> Medium {
      self.medium
   }
   pub fn set_medium(&mut self, m:&Medium) -> Medium {
      let old = self.medium.clone();
      self.medium.set(m);
      old
   }
   pub fn update_medium<F>(&mut self, f:F) -> Medium
      where F:Fn(&mut Medium)
   {
      let old = self.medium.clone();
      f(&mut self.medium);
      old
   }
}

impl <'a, W:WriteStream> ser::Serializer for &'a mut Serializer<W> {
   type Ok    = usize;
   type Error = ::Error;
   
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
      srialize_error("not implemented");
   }
   fn serialize_str(self, v:&str) -> ::Result<usize> {
      srialize_error("not implemented");
   }
   fn serialize_none(self) -> ::Result<usize> {
      srialize_error("not implemented");
   }
   fn serialize_some<T: ?Sized + ser::Serialize>(self, v:&T) -> ::Result<usize> {
      srialize_error("not implemented");
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



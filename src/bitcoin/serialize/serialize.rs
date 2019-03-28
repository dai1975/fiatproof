use ::iostream::{ WriteStream };
use super::Medium;

pub struct Serializer {
   medium: Medium,
}
pub trait Serializee {
   type P;
   fn serialize(&self, param:&Self::P, enc: &Serializer, ws: &mut WriteStream) -> ::Result<usize>;
}

macro_rules! def_serialize_proxy {
   ($f:ident, $f2:ident, $t:ty) => {
      #[inline] pub fn $f(&self, ws: &mut WriteStream, v:$t) -> ::Result<usize> {
         Ok(ws.$f2(v)?)
      }
   }
}

impl Serializer {
   pub fn new(m:&Medium) -> Self {
      Self { medium: m.clone() }
   }
   pub fn medium(&self) -> &Medium {
      let ref r = self.medium;
      r
   }
   pub fn update_media<F>(&mut self, f:F) -> Medium
      where F: Fn(Medium)->Medium
   {
      let r = self.medium.clone();
      self.medium = f(self.medium.clone());
      r
   }

   def_serialize_proxy! { serialize_skip,  write_skip, usize }
   def_serialize_proxy! { serialize_u8,    write_u8,    u8 }
   def_serialize_proxy! { serialize_u16le, write_u16le, u16 }
   def_serialize_proxy! { serialize_u32le, write_u32le, u32 }
   def_serialize_proxy! { serialize_u64le, write_u64le, u64 }
   def_serialize_proxy! { serialize_u16be, write_u16be, u16 }
   def_serialize_proxy! { serialize_u32be, write_u32be, u32 }
   def_serialize_proxy! { serialize_u64be, write_u64be, u64 }
   def_serialize_proxy! { serialize_i16le, write_i16le, i16 }
   def_serialize_proxy! { serialize_i32le, write_i32le, i32 }
   def_serialize_proxy! { serialize_i64le, write_i64le, i64 }
   def_serialize_proxy! { serialize_i16be, write_i16be, i16 }
   def_serialize_proxy! { serialize_i32be, write_i32be, i32 }
   def_serialize_proxy! { serialize_i64be, write_i64be, i64 }

   #[inline] pub fn serialize_bool(&self, ws: &mut WriteStream, v:bool) -> ::Result<usize> {
      let v = if v { 1 } else { 0 };
      Ok(ws.write_u8(v)?)
   }
   
   pub fn serialize_var_int(&self, ws:&mut WriteStream, v:u64) -> ::Result<usize> {
      let mut r = 0;
      if v < 253 {
         r += ws.write_u8(v as u8)?;
      } else if v <= 0xFFFF {
         r += ws.write_u8(253u8)?;
         r += ws.write_u16le(v as u16)?;
      } else if v <= 0xFFFFFFFF {
         r += ws.write_u8(254u8)?;
         r += ws.write_u32le(v as u32)?;
      } else {
         r += ws.write_u8(255u8)?;
         r += ws.write_u64le(v)?;
      }
      Ok(r)
   }
   pub fn serialize_octets(&self, ws:&mut WriteStream, v:&[u8]) -> ::Result<usize> {
      let r = ws.write(v)?;
      Ok(r)
   }
   pub fn serialize_var_octets(&self, ws:&mut WriteStream, v:&[u8], lim:usize) -> ::Result<usize> {
      if lim < v.len() {
         raise_serialize_error!(format!("sequence exceeds limit: {} but {}", lim, v.len()));
      }
      let mut r:usize = 0;
      r += self.serialize_var_int(ws, v.len() as u64)?;
      r += self.serialize_octets(ws, v)?;
      Ok(r)
   }
   pub fn serialize_var_string(&self, ws:&mut WriteStream, v:&str, lim:usize) -> ::Result<usize> {
      self.serialize_var_octets(ws, v.as_bytes(), lim)
   }
   pub fn serialize_var_array<T:Serializee>(&self, param:&T::P, ws:&mut WriteStream, v:&[T], lim:usize) -> ::Result<usize> {
      if lim < v.len() {
         raise_serialize_error!(format!("sequence exceeds limit: {} but {}", lim, v.len()));
      }
      let mut r:usize = 0;
      r += self.serialize_var_int(ws, v.len() as u64)?;
      for item in v.iter() {
         r += item.serialize(param, self, ws)?;
      }
      Ok(r)
   }
}

#[test]
fn test_serialize_var_int() {
   use ::iostream::{VecWriteStream};
   let mut ws = VecWriteStream::default();
   let m = Medium::new("net").unwrap();
   {
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 0u64), Ok(1));
      assert_matches!(e.serialize_var_int(&mut ws, 252u64), Ok(1));
   }
   assert_eq!(&ws.get_ref()[0..2], &[0, 252]);

   ws.rewind();
   {
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 253u64), Ok(3));    //lower limit
      assert_matches!(e.serialize_var_int(&mut ws, 0x0102u64), Ok(3)); //endian test
      assert_matches!(e.serialize_var_int(&mut ws, 0xFFFFu64), Ok(3)); //higher limit
   }
   assert_eq!(&ws.get_ref()[0..9], &[253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF]);

   ws.rewind();
   {
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 0x10000u64), Ok(5));
      assert_matches!(e.serialize_var_int(&mut ws, 0x01020304u64), Ok(5));
      assert_matches!(e.serialize_var_int(&mut ws, 0xFFFFFFFFu64), Ok(5));
   }
   assert_eq!(&ws.get_ref()[0..15],
              &[254, 0x00, 0x00, 0x01, 0x00,
               254, 0x04, 0x03, 0x02, 0x01,
               254, 0xFF, 0xFF, 0xFF, 0xFF]);
   ws.rewind();
   {
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 0x100000000u64), Ok(9));
      assert_matches!(e.serialize_var_int(&mut ws, 0x0102030405060708u64), Ok(9));
      assert_matches!(e.serialize_var_int(&mut ws, 0xFFFFFFFFFFFFFFFFu64), Ok(9));
   }
   assert_eq!(&ws.get_ref()[0..27],
              &[255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_serialize_var_octets() {
   let data = [0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75];
   use ::iostream::{VecWriteStream};
   let mut ws = VecWriteStream::default();
   
   {
      let m = Medium::new("net").unwrap();
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_octets(&mut ws, &data, 100), Ok(13));
   }
   assert_eq!(ws.get_ref()[0], 12);
   assert_eq!(&ws.get_ref()[1..], &data);
}

#[cfg(test)]
mod tests {
   use ::iostream::{ WriteStream };
   use super::{Serializer, Serializee};

   struct Foo { n:usize }
   impl Serializee for Foo {
      type P = ();
      fn serialize(&self, _p:&Self::P, e:&Serializer, ws:&mut WriteStream) -> ::Result<usize> {
         let n = self.n * 3;
         e.serialize_skip(ws, n)
      }
   }
   #[test]
   fn test_serialize_size() {
      use ::iostream::SizeWriteStream;
      use ::bitcoin::serialize::{Medium, Serializer};
      let f = Foo{ n:2 };
      let mut ws = SizeWriteStream::new();
      {
         let e = Serializer::new(&Medium::default().set_net());
         assert_matches!(f.serialize(&(), &e, &mut ws), Ok(6));
      }
      assert_eq!(ws.size(), 6);
   }
}

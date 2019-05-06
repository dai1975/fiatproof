use super::Medium;

pub struct Serializer {
   medium: Medium,
}
pub trait Serializee {
   type P;
   fn serialize<W: std::io::Write +?Sized>(&self, param:&Self::P, enc: &Serializer, ws: &mut W) -> crate::Result<usize>;
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
   
   #[inline(always)]
   pub fn serialize_u8<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u8) -> crate::Result<usize> {
      let buf: &[u8;1] = unsafe { std::mem::transmute(&v) };
      ws.write_all(buf)?;
      Ok(1)
   }
   #[inline(always)]
   pub fn serialize_i8<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i8) -> crate::Result<usize> {
      let buf: &[u8;1] = unsafe { std::mem::transmute(&v) };
      ws.write_all(buf)?;
      Ok(1)
   }

   
   #[inline(always)]
   pub fn serialize_u16le<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u16) -> crate::Result<usize> {
      let tmp = v.to_le();
      let buf: &[u8;2] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   pub fn serialize_u32le<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u32) -> crate::Result<usize> {
      let tmp = v.to_le();
      let buf: &[u8;4] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   pub fn serialize_u64le<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u64) -> crate::Result<usize> {
      let tmp = v.to_le();
      let buf: &[u8;8] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(8)
   }
   #[inline(always)]
   pub fn serialize_i16le<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i16) -> crate::Result<usize> {
      let tmp = v.to_le();
      let buf: &[u8;2] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   pub fn serialize_i32le<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i32) -> crate::Result<usize> {
      let tmp = v.to_le();
      let buf: &[u8;4] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   pub fn serialize_i64le<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i64) -> crate::Result<usize> {
      let tmp = v.to_le();
      let buf: &[u8;8] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(8)
   }

   #[inline(always)]
   pub fn serialize_u16be<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u16) -> crate::Result<usize> {
      let tmp = v.to_be();
      let buf: &[u8;2] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   pub fn serialize_u32be<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u32) -> crate::Result<usize> {
      let tmp = v.to_be();
      let buf: &[u8;4] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   pub fn serialize_u64be<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:u64) -> crate::Result<usize> {
      let tmp = v.to_be();
      let buf: &[u8;8] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(8)
   }
   #[inline(always)]
   pub fn serialize_i16be<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i16) -> crate::Result<usize> {
      let tmp = v.to_be();
      let buf: &[u8;2] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   pub fn serialize_i32be<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i32) -> crate::Result<usize> {
      let tmp = v.to_be();
      let buf: &[u8;4] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   pub fn serialize_i64be<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:i64) -> crate::Result<usize> {
      let tmp = v.to_be();
      let buf: &[u8;8] = unsafe { std::mem::transmute(&tmp) };
      ws.write_all(buf)?;
      Ok(8)
   }

   #[inline(always)]
   pub fn serialize_zeros<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:usize) -> crate::Result<usize> {
      let mut buf = Vec::<u8>::with_capacity(v);
      buf.resize(v, 0);
      let r = ws.write(buf.as_slice())?;
      Ok(r)
   }
   
   #[inline] pub fn serialize_bool<W: std::io::Write + ?Sized>(&self, ws: &mut W, v:bool) -> crate::Result<usize> {
      let v = if v { 1 } else { 0 };
      Ok(self.serialize_u8(ws, v)?)
   }
   
   pub fn serialize_var_int<W: std::io::Write + ?Sized>(&self, ws:&mut W, v:u64) -> crate::Result<usize> {
      let mut r = 0;
      if v < 253 {
         r += self.serialize_u8(ws, v as u8)?;
      } else if v <= 0xFFFF {
         r += self.serialize_u8(ws, 253u8)?;
         r += self.serialize_u16le(ws, v as u16)?;
      } else if v <= 0xFFFFFFFF {
         r += self.serialize_u8(ws, 254u8)?;
         r += self.serialize_u32le(ws, v as u32)?;
      } else {
         r += self.serialize_u8(ws, 255u8)?;
         r += self.serialize_u64le(ws, v)?;
      }
      Ok(r)
   }
   pub fn serialize_octets<W: std::io::Write + ?Sized>(&self, ws:&mut W, v:&[u8]) -> crate::Result<usize> {
      let r = ws.write(v)?;
      Ok(r)
   }
   pub fn serialize_var_octets<W: std::io::Write + ?Sized>(&self, ws:&mut W, v:&[u8], lim:usize) -> crate::Result<usize> {
      if lim < v.len() {
         raise_serialize_error!(format!("sequence exceeds limit: {} but {}", lim, v.len()));
      }
      let mut r:usize = 0;
      r += self.serialize_var_int(ws, v.len() as u64)?;
      r += self.serialize_octets(ws, v)?;
      Ok(r)
   }
   pub fn serialize_var_string<W: std::io::Write + ?Sized>(&self, ws:&mut W, v:&str, lim:usize) -> crate::Result<usize> {
      self.serialize_var_octets(ws, v.as_bytes(), lim)
   }
   pub fn serialize_var_array<W: std::io::Write + ?Sized, T:Serializee>(&self, param:&T::P, ws:&mut W, v:&[T], lim:usize) -> crate::Result<usize> {
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
   let mut v = Vec::<u8>::new();
   let m = Medium::new("net").unwrap();
   {
      //let mut ws:&mut [u8] = v.as_mut_slice();
      let mut ws = v.as_mut_slice();
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 0u64), Ok(1));
      assert_matches!(e.serialize_var_int(&mut ws, 252u64), Ok(1));
   }
   assert_eq!(&v[0..2], &[0, 252]);

   let mut v = Vec::<u8>::new();
   {
      let mut ws = v.as_mut_slice();
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 253u64), Ok(3));    //lower limit
      assert_matches!(e.serialize_var_int(&mut ws, 0x0102u64), Ok(3)); //endian test
      assert_matches!(e.serialize_var_int(&mut ws, 0xFFFFu64), Ok(3)); //higher limit
   }
   assert_eq!(&v[0..9], &[253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF]);

   let mut v = Vec::<u8>::new();
   {
      let mut ws = v.as_mut_slice();
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 0x10000u64), Ok(5));
      assert_matches!(e.serialize_var_int(&mut ws, 0x01020304u64), Ok(5));
      assert_matches!(e.serialize_var_int(&mut ws, 0xFFFFFFFFu64), Ok(5));
   }
   assert_eq!(&v[0..15],
              &[254, 0x00, 0x00, 0x01, 0x00,
                254, 0x04, 0x03, 0x02, 0x01,
                254, 0xFF, 0xFF, 0xFF, 0xFF]);
   
   let mut v = Vec::<u8>::new();
   {
      let mut ws = v.as_mut_slice();
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_int(&mut ws, 0x100000000u64), Ok(9));
      assert_matches!(e.serialize_var_int(&mut ws, 0x0102030405060708u64), Ok(9));
      assert_matches!(e.serialize_var_int(&mut ws, 0xFFFFFFFFFFFFFFFFu64), Ok(9));
   }
   assert_eq!(&v[0..27],
              &[255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_serialize_var_octets() {
   let data = [0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75];
   let mut v = Vec::<u8>::new();
   {
      let mut ws = v.as_mut_slice();
      let m = Medium::new("net").unwrap();
      let e = Serializer::new(&m);
      assert_matches!(e.serialize_var_octets(&mut ws, &data, 100), Ok(13));
   }
   assert_eq!(v[0], 12);
   assert_eq!(&v[1..], &data);
}

#[cfg(test)]
mod tests {
   use super::{Serializer, Serializee};

   struct Foo { n:usize }
   impl Serializee for Foo {
      type P = ();
      fn serialize<W: std::io::Write +?Sized>(&self, _p:&Self::P, e:&Serializer, ws:&mut W) -> crate::Result<usize> {
         let n = self.n * 3;
         e.serialize_zeros(ws, n)
      }
   }
   #[test]
   fn test_serialize_size() {
      use crate::bitcoin::serialize::{Medium, Serializer};
      let f = Foo{ n:2 };
      let mut ws = std::io::sink();
      {
         let e = Serializer::new(&Medium::default().set_net());
         assert_matches!(f.serialize(&(), &e, &mut ws), Ok(6));
      }
   }
}

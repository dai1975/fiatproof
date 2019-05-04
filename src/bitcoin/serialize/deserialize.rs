use super::Medium;

pub struct Deserializer {
   medium: Medium,
}
pub trait Deserializee {
   type P;
   fn deserialize<R: std::io::Read>(&mut self, param:&Self::P, dec: &Deserializer, rs:&mut R) -> crate::Result<usize>;
}

impl Deserializer {
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
   pub fn deserialize_u8<R: std::io::Read>(&self, rs: &mut R, v:&mut u8) -> crate::Result<usize> {
      let buf: &mut [u8;1] = unsafe { ::std::mem::transmute(v) };
      rs.read_exact(buf)?;
      Ok(1)
   }
   #[inline(always)]
   pub fn deserialize_i8<R: std::io::Read>(&self, rs: &mut R, v: &mut i8) -> crate::Result<usize> {
      let mut tmp:i8 = 0;
      let buf: &mut [u8;1] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = tmp;
      Ok(1)
   }
   
   #[inline(always)]
   pub fn deserialize_u16le<R: std::io::Read>(&self, rs: &mut R, v:&mut u16) -> crate::Result<usize> {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = u16::from_le(tmp);
      Ok(2)
   }
   #[inline(always)]
   pub fn deserialize_u32le<R: std::io::Read>(&self, rs: &mut R, v:&mut u32) -> crate::Result<usize> {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = u32::from_le(tmp);
      Ok(4)
   }
   #[inline(always)]
   pub fn deserialize_u64le<R: std::io::Read>(&self, rs: &mut R, v:&mut u64) -> crate::Result<usize> {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = u64::from_le(tmp);
      Ok(8)
   }
   #[inline(always)]
   pub fn deserialize_u16be<R: std::io::Read>(&self, rs: &mut R, v:&mut u16) -> crate::Result<usize> {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = u16::from_be(tmp);
      Ok(2)
   }
   #[inline(always)]
   pub fn deserialize_u32be<R: std::io::Read>(&self, rs: &mut R, v:&mut u32) -> crate::Result<usize> {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = u32::from_be(tmp);
      Ok(4)
   }
   #[inline(always)]
   pub fn deserialize_u64be<R: std::io::Read>(&self, rs: &mut R, v:&mut u64) -> crate::Result<usize> {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = u64::from_be(tmp);
      Ok(8)
   }

   #[inline(always)]
   pub fn deserialize_i16le<R: std::io::Read>(&self, rs: &mut R, v:&mut i16) -> crate::Result<usize> {
      let mut tmp:i16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = i16::from_le(tmp);
      Ok(2)
   }
   #[inline(always)]
   pub fn deserialize_i32le<R: std::io::Read>(&self, rs: &mut R, v:&mut i32) -> crate::Result<usize> {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = i32::from_le(tmp);
      Ok(4)
   }
   #[inline(always)]
   pub fn deserialize_i64le<R: std::io::Read>(&self, rs: &mut R, v:&mut i64) -> crate::Result<usize> {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = i64::from_le(tmp);
      Ok(8)
   }
   #[inline(always)]
   pub fn deserialize_i16be<R: std::io::Read>(&self, rs: &mut R, v:&mut i16) -> crate::Result<usize> {
      let mut tmp:i16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = i16::from_be(tmp);
      Ok(2)
   }
   #[inline(always)]
   pub fn deserialize_i32be<R: std::io::Read>(&self, rs: &mut R, v:&mut i32) -> crate::Result<usize> {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = i32::from_be(tmp);
      Ok(4)
   }
   #[inline(always)]
   pub fn deserialize_i64be<R: std::io::Read>(&self, rs: &mut R, v:&mut i64) -> crate::Result<usize> {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      rs.read_exact(buf)?;
      *v = i64::from_be(tmp);
      Ok(8)
   }
   
   pub fn deserialize_skip<R: std::io::Read>(&self, rs: &mut R, v:usize) -> crate::Result<usize> {
      // take() raises "E0507 cannot move out of borrowed content" why?
      //let r = std::io::copy(&mut rs.by_ref().take(v as u64), &mut std::io::sink())?;
      let mut buf = Vec::<u8>::with_capacity(v);
      buf.resize(v, 0);
      rs.read_exact(buf.as_mut_slice())?;
      Ok(v)
   }
   pub fn deserialize_bool<R: std::io::Read>(&self, rs: &mut R, v:&mut bool) -> crate::Result<usize> {
      let mut x:u8 = 0;
      let r = self.deserialize_u8(rs, &mut x)?;
      *v = x == 1;
      Ok(r)
   }
   pub fn deserialize_var_int<R: std::io::Read>(&self, rs: &mut R, v:&mut u64) -> crate::Result<usize> {
      let mut x:u8 = 0;
      let mut r = self.deserialize_u8(rs, &mut x)?;
      if x < 253 {
         *v = x as u64;
      } else if x == 253 {
         let mut y:u16 = 0;
         r += self.deserialize_u16le(rs, &mut y)?;
         *v = y as u64;
      } else if x == 254 {
         let mut y:u32 = 0;
         r += self.deserialize_u32le(rs, &mut y)?;
         *v = y as u64;
      } else {
         r += self.deserialize_u64le(rs, v)?;
      }
      Ok(r)
   }
   pub fn deserialize_octets<R: std::io::Read>(&self, rs: &mut R, v:&mut [u8]) -> crate::Result<usize> {
      let r = rs.read(v)?;
      if r != v.len() {
         deserialize_error!(format!("length mismatch: {} but {}", v.len(), r));
      }
      Ok(r)
   }
   pub fn deserialize_var_octets<R: std::io::Read>(&self, rs: &mut R, v:&mut Vec<u8>, lim:usize) -> crate::Result<usize> {
      let mut r:usize = 0;

      let size:usize = {
         let mut size:u64 = 0;
         r += self.deserialize_var_int(rs, &mut size)?;
         size as usize
      };
      if lim < size { deserialize_error!("sequence is too long"); }

      v.resize(size, 0);
      r += rs.read(v.as_mut_slice())?;
      Ok(r)
   }
   pub fn deserialize_to_end<R: std::io::Read>(&self, rs: &mut R, v:&mut Vec<u8>) -> crate::Result<usize> {
      let r = rs.read_to_end(v)?;
      Ok(r)
   }
   pub fn deserialize_var_string<R: std::io::Read>(&self, rs: &mut R, v:&mut String, lim:usize) -> crate::Result<usize> {
      let mut r:usize = 0;

      let size = {
         let mut size:u64 = 0;
         r += self.deserialize_var_int(rs, &mut size)?;
         size as usize
      };
      if lim < size { raise_deserialize_error!("string is too long") }

      let mut tmp = vec![0u8; size];
      r += self.deserialize_octets(rs, tmp.as_mut_slice())?;
      *v = String::from_utf8(tmp)?;

      Ok(r)
   }
   pub fn deserialize_var_array<R: std::io::Read, T: Deserializee>(&self, param:&T::P, rs: &mut R, v_:&mut Vec<T>, lim:usize) -> crate::Result<usize>
      where T: Deserializee+Default
   {
      let mut r:usize = 0;

      let size:usize = {
         let mut size:u64 = 0;
         r += self.deserialize_var_int(rs, &mut size)?;
         size as usize
      };
      if lim < size { deserialize_error!("sequence is too long"); }

      let mut v:Vec<T> = Vec::with_capacity(size);
      for _i in 0..size {
         let mut item = T::default();
         r += item.deserialize(param, self, rs)?;
         v.push(item);
      };
      *v_ = v;
      Ok(r)
   }
}

#[test]
fn test_deserialize_var_int() {
   use crate::iostream::{Slicestd::io::Read};
   {
      let buf:&[u8] = &[1,252];
      let mut r = Slicestd::io::Read::new(buf);
      let d = Deserializer::new(&Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(1));
      assert_eq!(v, 1);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(1));
      assert_eq!(v, 252);
   }
   {
      let buf:&[u8] = &[
         253, 253, 0,
         253, 0x02, 0x01,
         253, 0xFF, 0xFF
      ];
      let mut r = Slicestd::io::Read::new(buf);
      let d = Deserializer::new(&Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(3));    //lower limit
      assert_eq!(v, 253);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(3)); //endian test
      assert_eq!(v, 0x0102u64);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(3)); //higher limit
      assert_eq!(v, 0xFFFFu64);
   }
   {
      let buf:&[u8] = &[
         254, 0x00, 0x00, 0x01, 0x00,
         254, 0x04, 0x03, 0x02, 0x01,
         254, 0xFF, 0xFF, 0xFF, 0xFF
      ];
      let mut r = Slicestd::io::Read::new(buf);
      let d = Deserializer::new(&Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(5));
      assert_eq!(v, 0x10000u64);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(5));
      assert_eq!(v, 0x01020304u64);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(5));
      assert_eq!(v, 0xFFFFFFFFu64);
   }
   {
      let buf:&[u8] = &[
         255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
         255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
         255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
      ];
      let mut r = Slicestd::io::Read::new(buf);
      let d = Deserializer::new(&Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(9));
      assert_eq!(v, 0x100000000u64);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(9));
      assert_eq!(v, 0x0102030405060708u64);
      assert_matches!(d.deserialize_var_int(&mut r, &mut v), Ok(9));
      assert_eq!(v, 0xFFFFFFFFFFFFFFFFu64);
   }
}

#[cfg(test)]
mod tests {
   use crate::iostream::{ std::io::Read };
   use crate::bitcoin::serialize::{ Deserializer, Deserializee };

   struct Foo { n:usize }
   impl Deserializee for Foo {
      type P = ();
      fn deserialize(&mut self, _p:&Self::P, d:&Deserializer, rs:&mut std::io::Read) -> crate::Result<usize>
      {
         d.deserialize_skip(rs, self.n * 3)
      }
   }
   #[test]
   fn test_deserialize_size() {
      use crate::iostream::Sizestd::io::Read;
      use crate::bitcoin::serialize::{ Medium, Deserializer, Deserializee };
      let mut f = Foo{ n:2 };
      let mut r = Sizestd::io::Read::new();
      {
         let d = Deserializer::new(&Medium::default().set_net());
         assert_matches!(f.deserialize(&(), &d, &mut r), Ok(6));
      }
      assert_eq!(r.size(), 6);
   }
}

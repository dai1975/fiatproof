use crate::iostream::ReadStream;
use super::Medium;

pub struct Deserializer {
   medium: Medium,
}
pub trait Deserializee {
   type P;
   fn deserialize(&mut self, param:&Self::P, dec: &Deserializer, rs:&mut ReadStream) -> crate::Result<usize>;
}

macro_rules! def_deserialize_proxy {
   ($f:ident, $f2:ident, $t:ty) => {
      #[inline] pub fn $f(&self, rs: &mut ReadStream, v:&mut $t) -> crate::Result<usize> {
         Ok(rs.$f2(v)?)
      }
   }
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

   def_deserialize_proxy! { deserialize_u8,    read_u8,    u8 }
   def_deserialize_proxy! { deserialize_u16le, read_u16le, u16 }
   def_deserialize_proxy! { deserialize_u32le, read_u32le, u32 }
   def_deserialize_proxy! { deserialize_u64le, read_u64le, u64 }
   def_deserialize_proxy! { deserialize_u16be, read_u16be, u16 }
   def_deserialize_proxy! { deserialize_u32be, read_u32be, u32 }
   def_deserialize_proxy! { deserialize_u64be, read_u64be, u64 }
   def_deserialize_proxy! { deserialize_i16le, read_i16le, i16 }
   def_deserialize_proxy! { deserialize_i32le, read_i32le, i32 }
   def_deserialize_proxy! { deserialize_i64le, read_i64le, i64 }
   def_deserialize_proxy! { deserialize_i16be, read_i16be, i16 }
   def_deserialize_proxy! { deserialize_i32be, read_i32be, i32 }
   def_deserialize_proxy! { deserialize_i64be, read_i64be, i64 }
   
   pub fn deserialize_skip(&self, rs: &mut ReadStream, v:usize) -> crate::Result<usize> {
      Ok(rs.read_skip(v)?)
   }
   pub fn deserialize_bool(&self, rs: &mut ReadStream, v:&mut bool) -> crate::Result<usize> {
      let mut x:u8 = 0;
      let r = rs.read_u8(&mut x)?;
      *v = x == 1;
      Ok(r)
   }
   pub fn deserialize_var_int(&self, rs: &mut ReadStream, v:&mut u64) -> crate::Result<usize> {
      let mut x:u8 = 0;
      let mut r = rs.read_u8(&mut x)?;
      if x < 253 {
         *v = x as u64;
      } else if x == 253 {
         let mut y:u16 = 0;
         r += rs.read_u16le(&mut y)?;
         *v = y as u64;
      } else if x == 254 {
         let mut y:u32 = 0;
         r += rs.read_u32le(&mut y)?;
         *v = y as u64;
      } else {
         r += rs.read_u64le(v)?;
      }
      Ok(r)
   }
   pub fn deserialize_octets(&self, rs: &mut ReadStream, v:&mut [u8]) -> crate::Result<usize> {
      let r = rs.read(v)?;
      if r != v.len() {
         deserialize_error!(format!("length mismatch: {} but {}", v.len(), r));
      }
      Ok(r)
   }
   pub fn deserialize_var_octets(&self, rs: &mut ReadStream, v:&mut Vec<u8>, lim:usize) -> crate::Result<usize> {
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
   pub fn deserialize_to_end(&self, rs: &mut ReadStream, v:&mut Vec<u8>) -> crate::Result<usize> {
      let r = rs.read_to_end(v)?;
      Ok(r)
   }
   pub fn deserialize_var_string(&self, rs: &mut ReadStream, v:&mut String, lim:usize) -> crate::Result<usize> {
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
   pub fn deserialize_var_array<T>(&self, param:&T::P, rs: &mut ReadStream, v_:&mut Vec<T>, lim:usize) -> crate::Result<usize>
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
   use crate::iostream::{SliceReadStream};
   {
      let buf:&[u8] = &[1,252];
      let mut r = SliceReadStream::new(buf);
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
      let mut r = SliceReadStream::new(buf);
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
      let mut r = SliceReadStream::new(buf);
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
      let mut r = SliceReadStream::new(buf);
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
   use crate::iostream::{ ReadStream };
   use crate::bitcoin::serialize::{ Deserializer, Deserializee };

   struct Foo { n:usize }
   impl Deserializee for Foo {
      type P = ();
      fn deserialize(&mut self, _p:&Self::P, d:&Deserializer, rs:&mut ReadStream) -> crate::Result<usize>
      {
         d.deserialize_skip(rs, self.n * 3)
      }
   }
   #[test]
   fn test_deserialize_size() {
      use crate::iostream::SizeReadStream;
      use crate::bitcoin::serialize::{ Medium, Deserializer, Deserializee };
      let mut f = Foo{ n:2 };
      let mut r = SizeReadStream::new();
      {
         let d = Deserializer::new(&Medium::default().set_net());
         assert_matches!(f.deserialize(&(), &d, &mut r), Ok(6));
      }
      assert_eq!(r.size(), 6);
   }
}

use super::super::ReadStream;
use super::Medium;

pub struct Decoder<'a> {
   stream: &'a mut ReadStream,
   medium: Medium,
}
pub trait Decodee {
   fn decode(&mut self, dec: &mut Decoder) -> ::Result<usize>;
}
impl <'a> Decoder<'a> {
   pub fn new(s: &'a mut ReadStream, m:&Medium) -> Self {
      Self { stream:s, medium: m.clone() }
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

   pub fn decode_skip(&mut self, n:usize) -> ::Result<usize> {
      let r = try!(self.stream.read_skip(n));
      Ok(r)
   }
   
   pub fn decode_u8(&mut self, v:&mut u8) -> ::Result<usize> {
      let r = try!(self.stream.read_u8(v));
      Ok(r)
   }
   pub fn decode_u16le(&mut self, v:&mut u16) -> ::Result<usize> {
      let r = try!(self.stream.read_u16le(v));
      Ok(r)
   }
   pub fn decode_u32le(&mut self, v:&mut u32) -> ::Result<usize> {
      let r = try!(self.stream.read_u32le(v));
      Ok(r)
   }
   pub fn decode_u64le(&mut self, v:&mut u64) -> ::Result<usize> {
      let r = try!(self.stream.read_u64le(v));
      Ok(r)
   }
   pub fn decode_u16be(&mut self, v:&mut u16) -> ::Result<usize> {
      let r = try!(self.stream.read_u16be(v));
      Ok(r)
   }
   pub fn decode_u32be(&mut self, v:&mut u32) -> ::Result<usize> {
      let r = try!(self.stream.read_u32be(v));
      Ok(r)
   }
   pub fn decode_u64be(&mut self, v:&mut u64) -> ::Result<usize> {
      let r = try!(self.stream.read_u64be(v));
      Ok(r)
   }
   
   pub fn decode_i8(&mut self, v:&mut i8) -> ::Result<usize> {
      let r = try!(self.stream.read_i8(v));
      Ok(r)
   }
   pub fn decode_i16le(&mut self, v:&mut i16) -> ::Result<usize> {
      let r = try!(self.stream.read_i16le(v));
      Ok(r)
   }
   pub fn decode_i32le(&mut self, v:&mut i32) -> ::Result<usize> {
      let r = try!(self.stream.read_i32le(v));
      Ok(r)
   }
   pub fn decode_i64le(&mut self, v:&mut i64) -> ::Result<usize> {
      let r = try!(self.stream.read_i64le(v));
      Ok(r)
   }
   pub fn decode_i16be(&mut self, v:&mut i16) -> ::Result<usize> {
      let r = try!(self.stream.read_i16be(v));
      Ok(r)
   }
   pub fn decode_i32be(&mut self, v:&mut i32) -> ::Result<usize> {
      let r = try!(self.stream.read_i32be(v));
      Ok(r)
   }
   pub fn decode_i64be(&mut self, v:&mut i64) -> ::Result<usize> {
      let r = try!(self.stream.read_i64be(v));
      Ok(r)
   }
   
   pub fn decode_bool(&mut self, v:&mut bool) -> ::Result<usize> {
      let mut x:u8 = 0;
      let r = try!(self.stream.read_u8(&mut x));
      *v = x == 1;
      Ok(r)
   }
   pub fn decode_var_int(&mut self, v:&mut u64) -> ::Result<usize> {
      let mut x:u8 = 0;
      let mut r = try!(self.stream.read_u8(&mut x));
      if x < 253 {
         *v = x as u64;
      } else if x == 253 {
         let mut y:u16 = 0;
         r += try!(self.stream.read_u16le(&mut y));
         *v = y as u64;
      } else if x == 254 {
         let mut y:u32 = 0;
         r += try!(self.stream.read_u32le(&mut y));
         *v = y as u64;
      } else {
         r += try!(self.stream.read_u64le(v));
      }
      Ok(r)
   }
   pub fn decode_octets(&mut self, v:&mut [u8]) -> ::Result<usize> {
      let r = try!(self.stream.read(v));
      if r != v.len() {
         decode_error!(format!("length mismatch: {} but {}", v.len(), r));
      }
      Ok(r)
   }
   pub fn decode_var_octets(&mut self, v:&mut Vec<u8>, lim:usize) -> ::Result<usize> {
      let mut r:usize = 0;

      let size:usize = {
         let mut size:u64 = 0;
         r += try!(self.decode_var_int(&mut size));
         size as usize
      };
      if lim < size { decode_error!("sequence is too long"); }

      v.resize(size, 0);
      r += try!(self.stream.read(v.as_mut_slice()));
      Ok(r)
   }
   pub fn decode_to_end(&mut self, v:&mut Vec<u8>) -> ::Result<usize> {
      let r = try!(self.stream.read_to_end(v));
      Ok(r)
   }
   pub fn decode_var_string(&mut self, v:&mut String, lim:usize) -> ::Result<usize> {
      let mut r:usize = 0;

      let size = {
         let mut size:u64 = 0;
         r += try!(self.decode_var_int(&mut size));
         size as usize
      };
      if lim < size { encode_error!("string is too long") }

      let mut tmp = vec![0u8; size];
      r += try!(self.decode_octets(tmp.as_mut_slice()));
      *v = try!(String::from_utf8(tmp));

      Ok(r)
   }
   pub fn decode_var_array<T>(&mut self, v_:&mut Vec<T>, lim:usize) -> ::Result<usize>
      where T: Decodee + Default 
   {
      let mut r:usize = 0;

      let size:usize = {
         let mut size:u64 = 0;
         r += try!(self.decode_var_int(&mut size));
         size as usize
      };
      if lim < size { decode_error!("sequence is too long"); }

      let mut v:Vec<T> = Vec::with_capacity(size);
      for _i in 0..size {
         let mut item = T::default();
         r += try!(item.decode(self));
         v.push(item);
      };
      *v_ = v;
      Ok(r)
   }
}

#[test]
fn test_decode_var_int() {
   use ::serialize::{SliceReadStream};
   {
      let buf:&[u8] = &[1,252];
      let mut r = SliceReadStream::new(buf);
      let mut d = Decoder::new(&mut r, &Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.decode_var_int(&mut v), Ok(1));
      assert_eq!(v, 1);
      assert_matches!(d.decode_var_int(&mut v), Ok(1));
      assert_eq!(v, 252);
   }
   {
      let buf:&[u8] = &[
         253, 253, 0,
         253, 0x02, 0x01,
         253, 0xFF, 0xFF
      ];
      let mut r = SliceReadStream::new(buf);
      let mut d = Decoder::new(&mut r, &Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.decode_var_int(&mut v), Ok(3));    //lower limit
      assert_eq!(v, 253);
      assert_matches!(d.decode_var_int(&mut v), Ok(3)); //endian test
      assert_eq!(v, 0x0102u64);
      assert_matches!(d.decode_var_int(&mut v), Ok(3)); //higher limit
      assert_eq!(v, 0xFFFFu64);
   }
   {
      let buf:&[u8] = &[
         254, 0x00, 0x00, 0x01, 0x00,
         254, 0x04, 0x03, 0x02, 0x01,
         254, 0xFF, 0xFF, 0xFF, 0xFF
      ];
      let mut r = SliceReadStream::new(buf);
      let mut d = Decoder::new(&mut r, &Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.decode_var_int(&mut v), Ok(5));
      assert_eq!(v, 0x10000u64);
      assert_matches!(d.decode_var_int(&mut v), Ok(5));
      assert_eq!(v, 0x01020304u64);
      assert_matches!(d.decode_var_int(&mut v), Ok(5));
      assert_eq!(v, 0xFFFFFFFFu64);
   }
   {
      let buf:&[u8] = &[
         255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
         255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
         255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
      ];
      let mut r = SliceReadStream::new(buf);
      let mut d = Decoder::new(&mut r, &Medium::default().set_net());
      let mut v = 0u64;
      assert_matches!(d.decode_var_int(&mut v), Ok(9));
      assert_eq!(v, 0x100000000u64);
      assert_matches!(d.decode_var_int(&mut v), Ok(9));
      assert_eq!(v, 0x0102030405060708u64);
      assert_matches!(d.decode_var_int(&mut v), Ok(9));
      assert_eq!(v, 0xFFFFFFFFFFFFFFFFu64);
   }
}

#[cfg(test)]
mod tests {
   use ::serialize::bitcoin::{ Decoder, Decodee };

   struct Foo { n:usize }
   impl Decodee for Foo {
      fn decode(&mut self, d:&mut Decoder) -> ::Result<usize>
      {
         d.decode_skip(self.n * 3)
      }
   }
   #[test]
   fn test_decode_size() {
      use ::serialize::SizeReadStream;
      use ::serialize::bitcoin::{ Medium, Decoder, Decodee };
      let mut f = Foo{ n:2 };
      let mut r = SizeReadStream::new();
      {
         let mut d = Decoder::new(&mut r, &Medium::default().set_net());
         assert_matches!(f.decode(&mut d), Ok(6));
      }
      assert_eq!(r.size(), 6);
   }
}

use super::super::{ WriteStream };
use super::Medium;

pub struct Encoder<'a> {
   stream: &'a mut WriteStream,
   medium: Medium,
}
pub trait Encodee {
   fn encode(&self, enc: &mut Encoder) -> ::Result<usize>;
}

impl <'a> Encoder<'a> {
   pub fn new(s: &'a mut WriteStream, m:&Medium) -> Self {
      Self { stream:s, medium: m.clone() }
   }
   pub fn media(&self) -> &Medium {
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
   
   pub fn encode_skip(&mut self, n:usize) -> ::Result<usize> {
      let r = try!(self.stream.write_skip(n));
      Ok(r)
   }
   
   pub fn encode_u8(&mut self, v:u8) -> ::Result<usize> {
      let r = try!(self.stream.write_u8(v));
      Ok(r)
   }
   pub fn encode_u16le(&mut self, v:u16) -> ::Result<usize> {
      let r = try!(self.stream.write_u16le(v));
      Ok(r)
   }
   pub fn encode_u32le(&mut self, v:u32) -> ::Result<usize> {
      let r = try!(self.stream.write_u32le(v));
      Ok(r)
   }
   pub fn encode_u64le(&mut self, v:u64) -> ::Result<usize> {
      let r = try!(self.stream.write_u64le(v));
      Ok(r)
   }
   pub fn encode_u16be(&mut self, v:u16) -> ::Result<usize> {
      let r = try!(self.stream.write_u16be(v));
      Ok(r)
   }
   pub fn encode_u32be(&mut self, v:u32) -> ::Result<usize> {
      let r = try!(self.stream.write_u32be(v));
      Ok(r)
   }
   pub fn encode_u64be(&mut self, v:u64) -> ::Result<usize> {
      let r = try!(self.stream.write_u64le(v));
      Ok(r)
   }

   pub fn encode_i8(&mut self, v:i8) -> ::Result<usize> {
      let r = try!(self.stream.write_i8(v));
      Ok(r)
   }
   pub fn encode_i16le(&mut self, v:i16) -> ::Result<usize> {
      let r = try!(self.stream.write_i16le(v));
      Ok(r)
   }
   pub fn encode_i32le(&mut self, v:i32) -> ::Result<usize> {
      let r = try!(self.stream.write_i32le(v));
      Ok(r)
   }
   pub fn encode_i64le(&mut self, v:i64) -> ::Result<usize> {
      let r = try!(self.stream.write_i64le(v));
      Ok(r)
   }
   pub fn encode_i16be(&mut self, v:i16) -> ::Result<usize> {
      let r = try!(self.stream.write_i16be(v));
      Ok(r)
   }
   pub fn encode_i32be(&mut self, v:i32) -> ::Result<usize> {
      let r = try!(self.stream.write_i32be(v));
      Ok(r)
   }
   pub fn encode_i64be(&mut self, v:i64) -> ::Result<usize> {
      let r = try!(self.stream.write_i64be(v));
      Ok(r)
   }
   
   pub fn encode_bool(&mut self, v:bool) -> ::Result<usize> {
      let r = try!(self.stream.write_u8(if v {1u8} else {0u8}));
      Ok(r)
   }
   
   pub fn encode_varint(&mut self, v:u64) -> ::Result<usize> {
      let mut r = 0;
      if v < 253 {
         r += try!(self.stream.write_u8(v as u8));
      } else if v <= 0xFFFF {
         r += try!(self.stream.write_u8(253u8));
         r += try!(self.stream.write_u16le(v as u16));
      } else if v <= 0xFFFFFFFF {
         r += try!(self.stream.write_u8(254u8));
         r += try!(self.stream.write_u32le(v as u32));
      } else {
         r += try!(self.stream.write_u8(255u8));
         r += try!(self.stream.write_u64le(v));
      }
      Ok(r)
   }
   pub fn encode_array_u8(&mut self, v:&[u8]) -> ::Result<usize> {
      let r = try!(self.stream.write(v));
      Ok(r)
   }
   pub fn encode_sequence_u8(&mut self, v:&[u8], lim:Option<usize>) -> ::Result<usize> {
      if let Some(n) = lim {
         if n < v.len() {
            encode_error!(format!("sequence exceeds limit: {} but {}", n, v.len()));
         }
      }
      let mut r:usize = 0;
      r += try!(self.encode_varint(v.len() as u64));
      r += try!(self.encode_array_u8(v));
      Ok(r)
   }
   pub fn encode_string(&mut self, v:&str, lim:Option<usize>) -> ::Result<usize> {
      self.encode_sequence_u8(v.as_bytes(), lim)
   }
   pub fn encode_sequence<T:Encodee>(&mut self, v:&[T], lim:Option<usize>) -> ::Result<usize> {
      if let Some(n) = lim {
         if n < v.len() {
            encode_error!(format!("sequence exceeds limit: {} but {}", n, v.len()));
         }
      }
      let mut r:usize = 0;
      r += try!(self.encode_varint(v.len() as u64));
      for item in v.iter() {
         r += try!(item.encode(self));
      }
      Ok(r)
   } 
}

#[test]
fn test_encode_varint() {
   use ::serialize::{VecWriteStream};
   let mut w = VecWriteStream::default();
   let m = Medium::default().set_net();
   {
      let mut e = Encoder::new(&mut w, &m);
      assert_matches!(e.encode_varint(0u64), Ok(1));
      assert_matches!(e.encode_varint(252u64), Ok(1));
   }
   assert_eq!(&w.get_ref()[0..2], &[0, 252]);

   w.rewind();
   {
      let mut e = Encoder::new(&mut w, &m);
      assert_matches!(e.encode_varint(253u64), Ok(3));    //lower limit
      assert_matches!(e.encode_varint(0x0102u64), Ok(3)); //endian test
      assert_matches!(e.encode_varint(0xFFFFu64), Ok(3)); //higher limit
   }
   assert_eq!(&w.get_ref()[0..9], &[253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF]);

   w.rewind();
   {
      let mut e = Encoder::new(&mut w, &m);
      assert_matches!(e.encode_varint(0x10000u64), Ok(5));
      assert_matches!(e.encode_varint(0x01020304u64), Ok(5));
      assert_matches!(e.encode_varint(0xFFFFFFFFu64), Ok(5));
   }
   assert_eq!(&w.get_ref()[0..15],
              &[254, 0x00, 0x00, 0x01, 0x00,
               254, 0x04, 0x03, 0x02, 0x01,
               254, 0xFF, 0xFF, 0xFF, 0xFF]);
   w.rewind();
   {
      let mut e = Encoder::new(&mut w, &m);
      assert_matches!(e.encode_varint(0x100000000u64), Ok(9));
      assert_matches!(e.encode_varint(0x0102030405060708u64), Ok(9));
      assert_matches!(e.encode_varint(0xFFFFFFFFFFFFFFFFu64), Ok(9));
   }
   assert_eq!(&w.get_ref()[0..27],
              &[255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

/*
#[cfg(test)]
mod tests {
   use super::{Encoder, Encodee};

   struct Foo { n:usize }
   struct FooParam { m:usize }
   impl Encodee for Foo {
      type P = FooParam;
      fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, p:BP) -> ::Result<usize> {
         Ok(self.n * p.borrow().m)
      }
   }
   #[test]
   fn test_encode_size() {
      use ::serialize::SizeWriteStream;
      let f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut e = BitcoinEncodeStream::new(SizeWriteStream::new(), Medium::default().set_net());
      assert_matches!(f.encode(&mut e, &p), Ok(6));
   }
}

 */

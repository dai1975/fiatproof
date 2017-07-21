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
   pub fn decode_varint(&mut self, v:&mut u64) -> ::Result<usize> {
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
   pub fn decode_array_u8(&mut self, v:&mut [u8]) -> ::Result<usize> {
      let r = try!(self.stream.read(v));
      Ok(r)
   }
   pub fn decode_sequence_u8(&mut self, v:&mut Vec<u8>, lim:Option<usize>) -> ::Result<usize> {
      let lim = lim.unwrap_or(::std::usize::MAX);
      let mut r:usize = 0;

      let size = {
         let mut size:u64 = 0;
         r += try!(self.decode_varint(&mut size));
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
   pub fn decode_string(&mut self, v:&mut String, lim:Option<usize>) -> ::Result<usize> {
      let lim = lim.unwrap_or(::std::usize::MAX);
      let mut r:usize = 0;

      let size = {
         let mut size:u64 = 0;
         r += try!(self.decode_varint(&mut size));
         size as usize
      };
      if lim < size { encode_error!("string is too long") }

      let mut tmp = vec![0u8; size];
      r += try!(self.decode_array_u8(tmp.as_mut_slice()));
      *v = try!(String::from_utf8(tmp));

      Ok(r)
   }
}

/*
#[test]
fn test_decode_varint() {
   use ::serialize::{BitcoinDecoder, SliceReadStream, Medium};
   let mut d = BitcoinDecoder::new();
   let mut r = SliceReadStream::new(vec![0u8; 100]);
   let m = Medium::default().set_net();
   
   let mut v = 0u64;
   r.get_mut().as_mut_slice()[..2].copy_from_slice(&[1,252]);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(1));
   assert_eq!(v, 1);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(1));
   assert_eq!(v, 252);

   r.rewind();
   r.get_mut().as_mut_slice()[..9].copy_from_slice(&[
      253, 253, 0,
      253, 0x02, 0x01,
      253, 0xFF, 0xFF]);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(3));    //lower limit
   assert_eq!(v, 253);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(3)); //endian test
   assert_eq!(v, 0x0102u64);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(3)); //higher limit
   assert_eq!(v, 0xFFFFu64);

   r.rewind();
   r.get_mut().as_mut_slice()[..15].copy_from_slice(&[
      254, 0x00, 0x00, 0x01, 0x00,
      254, 0x04, 0x03, 0x02, 0x01,
      254, 0xFF, 0xFF, 0xFF, 0xFF]);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(5));
   assert_eq!(v, 0x10000u64);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(5));
   assert_eq!(v, 0x01020304u64);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(5));
   assert_eq!(v, 0xFFFFFFFFu64);

   r.rewind();
   r.get_mut().as_mut_slice()[..27].copy_from_slice(&[
      255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
      255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(9));
   assert_eq!(v, 0x100000000u64);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(9));
   assert_eq!(v, 0x0102030405060708u64);
   assert_matches!(d.decode_varint(&mut r, &m, &mut v), Ok(9));
   assert_eq!(v, 0xFFFFFFFFFFFFFFFFu64);
}

#[cfg(test)]
mod tests {
   use ::std::borrow::Borrow;
   use ::serialize::{Encodee, EncodeStream, BitcoinEncodeStream, Decodee, DecodeStream, BitcoinDecodeStream, Medium};

   struct Foo { n:usize }
   struct FooParam { m:usize }
   impl Decodee for Foo {
      type P = FooParam;
      fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, p:BP) -> ::Result<usize>
      {
         Ok(self.n * p.borrow().m)
      }
   }
   #[test]
   fn test_decode_size() {
      use ::serialize::SizeReadStream;
      let mut f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut d = BitcoinDecodeStream::new(SizeReadStream::new(), Medium::default().set_net());
      assert_matches!(f.decode(&mut d, &p), Ok(6));
   }
}

 */

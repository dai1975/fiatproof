use super::{WriteStream, ReadStream};

pub struct Encoder<'a> {
   w: &'a mut WriteStream
}
pub trait Encodee {
   fn encode(&self, enc: &mut Encoder) -> ::Result<usize>;
}
impl <'a> Encoder<'a> {
   pub fn new(w: &'a mut WriteStream) -> Self {
      Self { w:w }
   }

   pub fn encode_skip(&mut self, n:usize) -> ::Result<usize> {
      let r = try!(self.w.write_skip(n));
      Ok(r)
   }
   
   pub fn encode_u8(&mut self, v:u8) -> ::Result<usize> {
      let r = try!(self.w.write_u8(v));
      Ok(r)
   }
   pub fn encode_u16le(&mut self, v:u16) -> ::Result<usize> {
      let r = try!(self.w.write_u16le(v));
      Ok(r)
   }
   pub fn encode_u32le(&mut self, v:u32) -> ::Result<usize> {
      let r = try!(self.w.write_u32le(v));
      Ok(r)
   }
   pub fn encode_u64le(&mut self, v:u64) -> ::Result<usize> {
      let r = try!(self.w.write_u64le(v));
      Ok(r)
   }
   pub fn encode_u16be(&mut self, v:u16) -> ::Result<usize> {
      let r = try!(self.w.write_u16be(v));
      Ok(r)
   }
   pub fn encode_u32be(&mut self, v:u32) -> ::Result<usize> {
      let r = try!(self.w.write_u32be(v));
      Ok(r)
   }
   pub fn encode_u64be(&mut self, v:u64) -> ::Result<usize> {
      let r = try!(self.w.write_u64le(v));
      Ok(r)
   }

   pub fn encode_i8(&mut self, v:i8) -> ::Result<usize> {
      let r = try!(self.w.write_i8(v));
      Ok(r)
   }
   pub fn encode_i16le(&mut self, v:i16) -> ::Result<usize> {
      let r = try!(self.w.write_i16le(v));
      Ok(r)
   }
   pub fn encode_i32le(&mut self, v:i32) -> ::Result<usize> {
      let r = try!(self.w.write_i32le(v));
      Ok(r)
   }
   pub fn encode_i64le(&mut self, v:i64) -> ::Result<usize> {
      let r = try!(self.w.write_i64le(v));
      Ok(r)
   }
   pub fn encode_i16be(&mut self, v:i16) -> ::Result<usize> {
      let r = try!(self.w.write_i16be(v));
      Ok(r)
   }
   pub fn encode_i32be(&mut self, v:i32) -> ::Result<usize> {
      let r = try!(self.w.write_i32be(v));
      Ok(r)
   }
   pub fn encode_i64be(&mut self, v:i64) -> ::Result<usize> {
      let r = try!(self.w.write_i64be(v));
      Ok(r)
   }
   
   pub fn encode_bool(&mut self, v:bool) -> ::Result<usize> {
      let r = try!(self.w.write_u8(if v {1u8} else {0u8}));
      Ok(r)
   }
   
   pub fn encode_varint(&mut self, v:u64) -> ::Result<usize> {
      let mut r = 0;
      if v < 253 {
         r += try!(self.w.write_u8(v as u8));
      } else if v <= 0xFFFF {
         r += try!(self.w.write_u8(253u8));
         r += try!(self.w.write_u16le(v as u16));
      } else if v <= 0xFFFFFFFF {
         r += try!(self.w.write_u8(254u8));
         r += try!(self.w.write_u32le(v as u32));
      } else {
         r += try!(self.w.write_u8(255u8));
         r += try!(self.w.write_u64le(v));
      }
      Ok(r)
   }
   pub fn encode_array_u8(&mut self, v:&[u8]) -> ::Result<usize> {
      let r = try!(self.w.write(v));
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

pub struct Decoder<'a> {
   r: &'a mut ReadStream,
}
pub trait Decodee {
   fn decode(&mut self, dec: &mut Decoder) -> ::Result<usize>;
}
impl <'a> Decoder<'a> {
   pub fn new(r: &'a mut ReadStream) -> Self {
      Self { r:r }
   }

   pub fn decode_skip(&mut self, n:usize) -> ::Result<usize> {
      let r = try!(self.r.read_skip(n));
      Ok(r)
   }
   
   pub fn decode_u8(&mut self, v:&mut u8) -> ::Result<usize> {
      let r = try!(self.r.read_u8(v));
      Ok(r)
   }
   pub fn decode_u16le(&mut self, v:&mut u16) -> ::Result<usize> {
      let r = try!(self.r.read_u16le(v));
      Ok(r)
   }
   pub fn decode_u32le(&mut self, v:&mut u32) -> ::Result<usize> {
      let r = try!(self.r.read_u32le(v));
      Ok(r)
   }
   pub fn decode_u64le(&mut self, v:&mut u64) -> ::Result<usize> {
      let r = try!(self.r.read_u64le(v));
      Ok(r)
   }
   pub fn decode_u16be(&mut self, v:&mut u16) -> ::Result<usize> {
      let r = try!(self.r.read_u16be(v));
      Ok(r)
   }
   pub fn decode_u32be(&mut self, v:&mut u32) -> ::Result<usize> {
      let r = try!(self.r.read_u32be(v));
      Ok(r)
   }
   pub fn decode_u64be(&mut self, v:&mut u64) -> ::Result<usize> {
      let r = try!(self.r.read_u64be(v));
      Ok(r)
   }
   
   pub fn decode_i8(&mut self, v:&mut i8) -> ::Result<usize> {
      let r = try!(self.r.read_i8(v));
      Ok(r)
   }
   pub fn decode_i16le(&mut self, v:&mut i16) -> ::Result<usize> {
      let r = try!(self.r.read_i16le(v));
      Ok(r)
   }
   pub fn decode_i32le(&mut self, v:&mut i32) -> ::Result<usize> {
      let r = try!(self.r.read_i32le(v));
      Ok(r)
   }
   pub fn decode_i64le(&mut self, v:&mut i64) -> ::Result<usize> {
      let r = try!(self.r.read_i64le(v));
      Ok(r)
   }
   pub fn decode_i16be(&mut self, v:&mut i16) -> ::Result<usize> {
      let r = try!(self.r.read_i16be(v));
      Ok(r)
   }
   pub fn decode_i32be(&mut self, v:&mut i32) -> ::Result<usize> {
      let r = try!(self.r.read_i32be(v));
      Ok(r)
   }
   pub fn decode_i64be(&mut self, v:&mut i64) -> ::Result<usize> {
      let r = try!(self.r.read_i64be(v));
      Ok(r)
   }
   
   pub fn decode_bool(&mut self, v:&mut bool) -> ::Result<usize> {
      let mut x:u8 = 0;
      let r = try!(self.r.read_u8(&mut x));
      *v = x == 1;
      Ok(r)
   }
   pub fn decode_varint(&mut self, v:&mut u64) -> ::Result<usize> {
      let mut x:u8 = 0;
      let mut r = try!(self.r.read_u8(&mut x));
      if x < 253 {
         *v = x as u64;
      } else if x == 253 {
         let mut y:u16 = 0;
         r += try!(self.r.read_u16le(&mut y));
         *v = y as u64;
      } else if x == 254 {
         let mut y:u32 = 0;
         r += try!(self.r.read_u32le(&mut y));
         *v = y as u64;
      } else {
         r += try!(self.r.read_u64le(v));
      }
      Ok(r)
   }
   pub fn decode_array_u8(&mut self, v:&mut [u8]) -> ::Result<usize> {
      let r = try!(self.r.read(v));
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
      r += try!(self.r.read(v.as_mut_slice()));
      Ok(r)
   }
   pub fn decode_to_end(&mut self, v:&mut Vec<u8>) -> ::Result<usize> {
      let r = try!(self.r.read_to_end(v));
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

// TODO あとで ToBytes などの trait に統合
pub fn to_bytes(obj:&Encodee) -> ::Result<Vec<u8>> {
   let mut w = super::VecWriteStream::default();
   {
      let mut enc = Encoder::new(&mut w);
      let r = try!(obj.encode(&mut enc));
   }
   Ok(w.into_inner())
}
pub fn from_bytes<T:Decodee+Default>(v: &Vec<u8>) -> ::Result<T> {
   let mut ret = T::default();
   let mut r = ::std::io::Cursor::new(v);
   let mut d = Decoder::new(&mut r);
   let _ = try!(ret.decode(&mut d));
   Ok(ret)
}

#[test]
fn test_encode_varint() {
   use ::serialize::{VecWriteStream, Encoder};
   let mut w = VecWriteStream::default();
   let mut e = Encoder::new();
   let m = Media::default().set_net();

   assert_matches!(e.encode_varint(&mut w, &m, 0u64), Ok(1));
   assert_matches!(e.encode_varint(&mut w, &m, 252u64), Ok(1));
   assert_eq!(&w.get_ref()[0..2], &[0, 252]);

   w.rewind();
   assert_matches!(e.encode_varint(&mut w, &m, 253u64), Ok(3));    //lower limit
   assert_matches!(e.encode_varint(&mut w, &m, 0x0102u64), Ok(3)); //endian test
   assert_matches!(e.encode_varint(&mut w, &m, 0xFFFFu64), Ok(3)); //higher limit
   assert_eq!(&w.get_ref()[0..9], &[253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF]);

   w.rewind();
   assert_matches!(e.encode_varint(&mut w, &m, 0x10000u64), Ok(5));
   assert_matches!(e.encode_varint(&mut w, &m, 0x01020304u64), Ok(5));
   assert_matches!(e.encode_varint(&mut w, &m, 0xFFFFFFFFu64), Ok(5));
   assert_eq!(&w.get_ref()[0..15],
              &[254, 0x00, 0x00, 0x01, 0x00,
               254, 0x04, 0x03, 0x02, 0x01,
               254, 0xFF, 0xFF, 0xFF, 0xFF]);
   w.rewind();
   assert_matches!(e.encode_varint(&mut w, &m, 0x100000000u64), Ok(9));
   assert_matches!(e.encode_varint(&mut w, &m, 0x0102030405060708u64), Ok(9));
   assert_matches!(e.encode_varint(&mut w, &m, 0xFFFFFFFFFFFFFFFFu64), Ok(9));
   assert_eq!(&w.get_ref()[0..27],
              &[255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_decode_varint() {
   use ::serialize::{BitcoinDecoder, SliceReadStream, Media};
   let mut d = BitcoinDecoder::new();
   let mut r = SliceReadStream::new(vec![0u8; 100]);
   let m = Media::default().set_net();
   
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
   use ::serialize::{Encodee, EncodeStream, BitcoinEncodeStream, Decodee, DecodeStream, BitcoinDecodeStream, Media};

   struct Foo { n:usize }
   struct FooParam { m:usize }
   impl Encodee for Foo {
      type P = FooParam;
      fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, p:BP) -> ::Result<usize> {
         Ok(self.n * p.borrow().m)
      }
   }
   impl Decodee for Foo {
      type P = FooParam;
      fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, p:BP) -> ::Result<usize>
      {
         Ok(self.n * p.borrow().m)
      }
   }
   #[test]
   fn test_encode_size() {
      use ::serialize::SizeWriteStream;
      let f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut e = BitcoinEncodeStream::new(SizeWriteStream::new(), Media::default().set_net());
      assert_matches!(f.encode(&mut e, &p), Ok(6));
   }
   #[test]
   fn test_decode_size() {
      use ::serialize::SizeReadStream;
      let mut f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut d = BitcoinDecodeStream::new(SizeReadStream::new(), Media::default().set_net());
      assert_matches!(f.decode(&mut d, &p), Ok(6));
   }
}


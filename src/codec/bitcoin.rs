use super::{WriteStream, ReadStream, Encoder, Decoder, EncodeStream, DecodeStream, Media};

pub struct BitcoinEncoder { }
impl BitcoinEncoder {
   pub fn new() -> Self { BitcoinEncoder { } }
}

impl Encoder for BitcoinEncoder {
   fn encode_skip(&mut self, w:&mut WriteStream, _m:&Media, n:usize) -> ::Result<usize> {
      try!(w.write_skip(n)); Ok(n)
   }
   
   fn encode_u8(&mut self, w:&mut WriteStream, _m:&Media, v:u8) -> ::Result<usize> {
      try!(w.write_u8(v)); Ok(1usize)
   }
   fn encode_u16le(&mut self, w:&mut WriteStream, _m:&Media, v:u16) -> ::Result<usize> {
      try!(w.write_u16le(v)); Ok(2usize)
   }
   fn encode_u32le(&mut self, w:&mut WriteStream, _m:&Media, v:u32) -> ::Result<usize> {
      try!(w.write_u32le(v)); Ok(4usize)
   }
   fn encode_u64le(&mut self, w:&mut WriteStream, _m:&Media, v:u64) -> ::Result<usize> {
      try!(w.write_u64le(v)); Ok(8usize)
   }
   fn encode_u16be(&mut self, w:&mut WriteStream, _m:&Media, v:u16) -> ::Result<usize> {
      try!(w.write_u16be(v)); Ok(2usize)
   }
   fn encode_u32be(&mut self, w:&mut WriteStream, _m:&Media, v:u32) -> ::Result<usize> {
      try!(w.write_u32be(v)); Ok(4usize)
   }
   fn encode_u64be(&mut self, w:&mut WriteStream, _m:&Media, v:u64) -> ::Result<usize> {
      try!(w.write_u64le(v)); Ok(8usize)
   }

   fn encode_i8(&mut self, w:&mut WriteStream, _m:&Media, v:i8) -> ::Result<usize> {
      try!(w.write_i8(v)); Ok(1usize)
   }
   fn encode_i16le(&mut self, w:&mut WriteStream, _m:&Media, v:i16) -> ::Result<usize> {
      try!(w.write_i16le(v)); Ok(2usize)
   }
   fn encode_i32le(&mut self, w:&mut WriteStream, _m:&Media, v:i32) -> ::Result<usize> {
      try!(w.write_i32le(v)); Ok(4usize)
   }
   fn encode_i64le(&mut self, w:&mut WriteStream, _m:&Media, v:i64) -> ::Result<usize> {
      try!(w.write_i64le(v)); Ok(8usize)
   }
   fn encode_i16be(&mut self, w:&mut WriteStream, _m:&Media, v:i16) -> ::Result<usize> {
      try!(w.write_i16be(v)); Ok(2usize)
   }
   fn encode_i32be(&mut self, w:&mut WriteStream, _m:&Media, v:i32) -> ::Result<usize> {
      try!(w.write_i32be(v)); Ok(4usize)
   }
   fn encode_i64be(&mut self, w:&mut WriteStream, _m:&Media, v:i64) -> ::Result<usize> {
      try!(w.write_i64be(v)); Ok(8usize)
   }
   
   fn encode_bool(&mut self, w:&mut WriteStream, _m:&Media, v:bool) -> ::Result<usize> {
      try!(w.write_u8(if v {1u8} else {0u8}));
      Ok(1usize)
   }
   
   fn encode_varint(&mut self, w:&mut WriteStream, _m:&Media, v:u64) -> ::Result<usize> {
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
   fn encode_array_u8(&mut self, w:&mut WriteStream, _m:&Media, v:&[u8]) -> ::Result<usize> {
      try!(w.write(v));
      Ok(v.len())
   }
   fn encode_sequence_u8(&mut self, w:&mut WriteStream, m:&Media, v:&[u8]) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.encode_varint(w, m, v.len() as u64));
      try!(w.write(v));
      r += v.len();
      Ok(r)
   }
}

pub struct BitcoinDecoder { }
impl BitcoinDecoder {
   pub fn new() -> Self { BitcoinDecoder { } }
}

impl Decoder for BitcoinDecoder {
   fn decode_skip(&mut self, r:&mut ReadStream, _m:&Media, n:usize) -> ::Result<usize> {
      try!(r.read_skip(n)); Ok(n)
   }
   
   fn decode_u8(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u8) -> ::Result<usize> {
      try!(r.read_u8(v)); Ok(1usize)
   }
   fn decode_u16le(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u16) -> ::Result<usize> {
      try!(r.read_u16le(v)); Ok(2usize)
   }
   fn decode_u32le(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u32) -> ::Result<usize> {
      try!(r.read_u32le(v)); Ok(4usize)
   }
   fn decode_u64le(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u64) -> ::Result<usize> {
      try!(r.read_u64le(v)); Ok(8usize)
   }
   fn decode_u16be(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u16) -> ::Result<usize> {
      try!(r.read_u16be(v)); Ok(2usize)
   }
   fn decode_u32be(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u32) -> ::Result<usize> {
      try!(r.read_u32be(v)); Ok(4usize)
   }
   fn decode_u64be(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u64) -> ::Result<usize> {
      try!(r.read_u64be(v)); Ok(8usize)
   }
   
   fn decode_i8(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i8) -> ::Result<usize> {
      try!(r.read_i8(v)); Ok(1usize)
   }
   fn decode_i16le(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i16) -> ::Result<usize> {
      try!(r.read_i16le(v)); Ok(2usize)
   }
   fn decode_i32le(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i32) -> ::Result<usize> {
      try!(r.read_i32le(v)); Ok(4usize)
   }
   fn decode_i64le(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i64) -> ::Result<usize> {
      try!(r.read_i64le(v)); Ok(8usize)
   }
   fn decode_i16be(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i16) -> ::Result<usize> {
      try!(r.read_i16be(v)); Ok(2usize)
   }
   fn decode_i32be(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i32) -> ::Result<usize> {
      try!(r.read_i32be(v)); Ok(4usize)
   }
   fn decode_i64be(&mut self, r:&mut ReadStream, _m:&Media, v:&mut i64) -> ::Result<usize> {
      try!(r.read_i64be(v)); Ok(8usize)
   }
   
   fn decode_bool(&mut self, r:&mut ReadStream, _m:&Media, v:&mut bool) -> ::Result<usize> {
      let mut x:u8 = 0;
      try!(r.read_u8(&mut x));
      *v = x == 1;
      Ok(1usize)
   }
   fn decode_varint(&mut self, r:&mut ReadStream, _m:&Media, v:&mut u64) -> ::Result<usize> {
      let mut x:u8 = 0;
      try!(r.read_u8(&mut x));
      if x < 253 {
         *v = x as u64;
         Ok(1)
      } else if x == 253 {
         let mut y:u16 = 0;
         try!(r.read_u16le(&mut y));
         *v = y as u64;
         Ok(3)
      } else if x == 254 {
         let mut y:u32 = 0;
         try!(r.read_u32le(&mut y));
         *v = y as u64;
         Ok(5)
      } else {
         try!(r.read_u64le(v));
         Ok(9)
      }
   }
   fn decode_array_u8(&mut self, r:&mut ReadStream, _m:&Media, v:&mut [u8]) -> ::Result<usize> {
      let a = try!(r.read(v));
      Ok(a)
   }
   fn decode_sequence_u8(&mut self, r:&mut ReadStream, m:&Media, v:&mut Vec<u8>) -> ::Result<usize> {
      let mut a:usize = 0;
      {
         let mut x:u64 = 0;
         a += try!(self.decode_varint(r, m, &mut x));
         v.resize(x as usize, 0);
      }
      a += try!(r.read(v.as_mut_slice()));
      Ok(a)
   }
}

pub struct BitcoinEncodeStream<W:WriteStream+Sized> {
   pub w: W,
   pub e: BitcoinEncoder,
   pub m: Media,
}
impl <W:WriteStream+Sized> BitcoinEncodeStream<W> {
   pub fn new(w:W, m:Media) -> Self {
      BitcoinEncodeStream { w:w, e:BitcoinEncoder::new(), m:m }
   }
}
impl <W:WriteStream+Sized> EncodeStream for BitcoinEncodeStream<W> {
   type W = W;
   type E = BitcoinEncoder;
   fn stream(&mut self)  -> &mut Self::W { &mut self.w }
   fn encoder(&mut self) -> &mut Self::E { &mut self.e }
   fn media(&self)       -> &Media { &self.m }
   fn then<F>(&mut self, f:F) -> ::Result<usize> where F: Fn(&mut Self::W, &mut Self::E, &Media) -> ::Result<usize> {
      f(&mut self.w, &mut self.e, &self.m)
   }
}

pub struct BitcoinDecodeStream<R:ReadStream+Sized> {
   pub r: R,
   pub d: BitcoinDecoder,
   pub m: Media,
}
impl <R:ReadStream+Sized> BitcoinDecodeStream<R> {
   pub fn new(r:R, m:Media) -> Self {
      BitcoinDecodeStream { r:r, d:BitcoinDecoder::new(), m:m }
   }
}
impl <R:ReadStream+Sized> DecodeStream for BitcoinDecodeStream<R> {
   type R = R;
   type D = BitcoinDecoder;
   fn stream(&mut self)  -> &mut Self::R { &mut self.r }
   fn decoder(&mut self) -> &mut Self::D { &mut self.d }
   fn media(&self)       -> &Media { &self.m }
   fn then<F>(&mut self, mut f:F) -> ::Result<usize> where F: FnMut(&mut Self::R, &mut Self::D, &Media) -> ::Result<usize> {
      f(&mut self.r, &mut self.d, &self.m)
   }
}


#[test]
fn test_encode_varint() {
   use ::encode::{VecWriteStream, BitcoinEncoder, Media};
   let mut w = VecWriteStream::default();
   let mut e = BitcoinEncoder::new();
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
   use ::encode::{BitcoinDecoder, SliceReadStream, Media};
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
   use ::encode::{Encodee, EncodeStream, BitcoinEncodeStream, Decodee, DecodeStream, BitcoinDecodeStream, Media};

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
   fn test_encode() {
      use ::encode::SizeWriteStream;
      let f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut e = BitcoinEncodeStream::new(SizeWriteStream::new(), Media::default().set_net());
      assert_matches!(f.encode(&mut e, &p), Ok(6));
   }
   #[test]
   fn test_decode() {
      use ::encode::SizeReadStream;
      let mut f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut d = BitcoinDecodeStream::new(SizeReadStream::new(), Media::default().set_net());
      assert_matches!(f.decode(&mut d, &p), Ok(6));
   }
}


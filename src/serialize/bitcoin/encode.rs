use ::Error;
use super::super::{Encoder, WriteStream, Serializer };
use super::BitcoinEncodeParam;

pub trait BitcoinEncoder: Encoder<P = BitcoinEncodeParam> {
   fn encode_varint<W:WriteStream>(&mut self, w:&mut W, v:u64, p:&BitcoinEncodeParam) -> Result<usize, Error>;
}

pub trait BitcoinEncodee {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&E::P) -> Result<usize, Error>;
}   

impl <E:BitcoinEncoder, W:WriteStream> Serializer<E,W> {
   #[inline(always)]
   pub fn serialize_bitcoin<A:BitcoinEncodee>(&mut self, obj:&A, p:&E::P) -> Result<usize, Error> {
      self.flat_map(|e,w| { obj.encode(e, w, p) })
   }
   pub fn serialize_varint(&mut self, v:u64, p:&E::P) -> Result<usize, Error> {
      //self.e.encode_varint(&mut self.w, v, p) // cannot access to private members because this impl is treated as external.
      self.flat_map(|e,w| { e.encode_varint(w, v, p) })
   }
}

#[derive(Default)]
pub struct BitcoinEncoderImpl { }

impl Encoder for BitcoinEncoderImpl {
   type P = BitcoinEncodeParam;
   #[inline(always)]
   fn encode_bytes<W:WriteStream>(&mut self, w:&mut W, v:&[u8], _p:&Self::P) -> Result<usize, Error> {
      try!(w.write(v));
      Ok(v.len())
   }
   #[inline(always)]
   fn encode_bool<W:WriteStream>(&mut self, w:&mut W, v:bool, _p:&Self::P) -> Result<usize, Error> {
      try!(w.write_u8(if v {1u8} else {0u8}));
      Ok(1usize)
   }
   #[inline(always)]
   fn encode_u8<W:WriteStream>(&mut self, w:&mut W, v:u8, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_u8(v));
      Ok(1usize)
   }
   #[inline(always)]
   fn encode_u16<W:WriteStream>(&mut self, w:&mut W, v:u16, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_u16le(v));
      Ok(2usize)
   }
   #[inline(always)]
   fn encode_u32<W:WriteStream>(&mut self, w:&mut W, v:u32, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_u32le(v));
      Ok(4usize)
   }
   #[inline(always)]
   fn encode_u64<W:WriteStream>(&mut self, w:&mut W, v:u64, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_u64le(v));
      Ok(8usize)
   }
   #[inline(always)]
   fn encode_i8<W:WriteStream>(&mut self, w:&mut W, v:i8, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_i8(v));
      Ok(1usize)
   }
   #[inline(always)]
   fn encode_i16<W:WriteStream>(&mut self, w:&mut W, v:i16, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_i16le(v));
      Ok(2usize)
   }
   #[inline(always)]
   fn encode_i32<W:WriteStream>(&mut self, w:&mut W, v:i32, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_i32le(v));
      Ok(4usize)
   }
   #[inline(always)]
   fn encode_i64<W:WriteStream>(&mut self, w:&mut W, v:i64, _p:&BitcoinEncodeParam) -> Result<usize, Error> {
      try!(w.write_i64le(v));
      Ok(8usize)
   }
}
impl BitcoinEncoder for BitcoinEncoderImpl {
   fn encode_varint<W:WriteStream>(&mut self, w:&mut W, v:u64, p:&BitcoinEncodeParam) -> Result<usize, Error> {
      let mut r = 0usize;
      if v < 253 {
         r += try!(self.encode_u8(w, v as u8, p));
      } else if v <= 0xFFFF {
         r += try!(self.encode_u8(w, 253u8, p));
         r += try!(self.encode_u16(w, v as u16, p));
      } else if v <= 0xFFFFFFFF {
         r += try!(self.encode_u8(w, 254u8, p));
         r += try!(self.encode_u32(w, v as u32, p));
      } else {
         r += try!(self.encode_u8(w, 255u8, p));
         r += try!(self.encode_u64(w, v, p));
      }
      Ok(r)
   }
}

pub type BitcoinSerializer<W:WriteStream> = Serializer<BitcoinEncoderImpl, W>;
impl <W:WriteStream> BitcoinSerializer<W> {
   pub fn new_with(w:W) -> Self {
      Serializer::new_with_with(BitcoinEncoderImpl::default(), w)
   }
}

use super::super::{ FixedSerializer, SizeSerializer, DHash256Serializer };
pub type FixedBitcoinSerializer = FixedSerializer<BitcoinEncoderImpl>;
impl FixedBitcoinSerializer {
   pub fn new(size:usize) -> Self { FixedBitcoinSerializer::new_fixed(BitcoinEncoderImpl::default(), size) }
}

pub type SizeBitcoinSerializer = SizeSerializer<BitcoinEncoderImpl>;
impl SizeBitcoinSerializer {
   pub fn new() -> Self { SizeBitcoinSerializer::new_size(BitcoinEncoderImpl::default()) }
}

pub type DHash256BitcoinSerializer = DHash256Serializer<BitcoinEncoderImpl>;
impl DHash256BitcoinSerializer {
   pub fn new() -> Self { Self::new_with_default(BitcoinEncoderImpl::default()) }
}

#[test]
fn test_encoder_vec() {
   use std::io::Cursor;
   let mut ws  = Cursor::new(Vec::<u8>::with_capacity(100));
   let mut enc = BitcoinEncoderImpl { };
   let     ep  = BitcoinEncodeParam::new_net();
   assert_eq!(0, ws.get_ref().len());
   assert_matches!(enc.encode_bool(&mut ws, true, &ep),  Ok(1));
   assert_matches!(enc.encode_bool(&mut ws, false, &ep), Ok(1));
   assert_eq!(2, ws.get_ref().len());
   assert_eq!([0x01, 0x00], &ws.get_ref()[0..2]);
}

#[test]
fn test_serializer_vec() {
   use std::io::Cursor;
   let     ws  = Cursor::new(Vec::<u8>::with_capacity(100));
   let mut ser = BitcoinSerializer::new_with(ws);
   let     ep  = BitcoinEncodeParam::new_net();
   assert_eq!(0, ser.get_ref().get_ref().len());
   assert_matches!(ser.serialize_bool(true, &ep),  Ok(1));
   assert_matches!(ser.serialize_bool(false, &ep), Ok(1));
   assert_eq!(2, ser.get_ref().get_ref().len());
   assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
}

#[test]
fn test_serializer_slice() {
   use super::super::SliceWriteStream;
   {
      let     ws  = SliceWriteStream::new([0u8;32]);
      let mut ser = BitcoinSerializer::new_with(ws);
      let     ep  = BitcoinEncodeParam::new_net();
      assert_eq!(32, ser.get_ref().get_ref().len());
      assert_matches!(ser.serialize_bool(true, &ep),  Ok(1));
      assert_matches!(ser.serialize_bool(false, &ep), Ok(1));
      assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
   }
   {
      let mut v = Vec::<u8>::with_capacity(100);
      unsafe { v.set_len(100); }
      let     ws  = SliceWriteStream::new(v);
      let mut ser = BitcoinSerializer::new_with(ws);
      let     ep  = BitcoinEncodeParam::new_net();
      assert_eq!(100, ser.get_ref().get_ref().len());
      assert_matches!(ser.serialize_bool(true, &ep),  Ok(1));
      assert_matches!(ser.serialize_bool(false, &ep), Ok(1));
      assert_eq!(100, ser.get_ref().get_ref().len());
      assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
   }
}

#[test]
fn test_serializer_fixed() {
   let mut ser = FixedBitcoinSerializer::new(100);
   let     ep  = BitcoinEncodeParam::new_net();
   assert_eq!(100, ser.get_ref().get_ref().len());
   assert_matches!(ser.serialize_bool(true, &ep),  Ok(1));
   assert_matches!(ser.serialize_bool(false, &ep), Ok(1));
   assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
}

#[test]
fn test_serializer_size() {
   let mut ser = SizeBitcoinSerializer::new();
   let     ep  = BitcoinEncodeParam::new_net();
   assert_eq!(0, ser.size());
   assert_matches!(ser.serialize_bool(true, &ep),  Ok(1));
   assert_matches!(ser.serialize_bool(false, &ep), Ok(1));
   assert_eq!(2, ser.size());
}

#[test]
fn test_serializer_hash() {
   let mut ser = DHash256BitcoinSerializer::new();
   let     ep  = BitcoinEncodeParam::new_net();
   assert_matches!(ser.serialize_bool(true, &ep),  Ok(1));
   assert_matches!(ser.serialize_bool(false, &ep), Ok(1));
   assert_eq!("677b2d718464ee0121475600b929c0b4155667486577d1320b18c2dc7d4b4f99", ser.hash_hexresult());
}


#[cfg(test)]
mod test {
   // If you want to implements new encoding format, define dedicated Encoder, EncoderImpl, Encodee, Serializer.
   // Note that the Serializer have a serialize_<encoding name> method which is given Encodee object.
   use ::Error;
   use super::super::super::{Encoder, WriteStream, Serializer};
   pub trait FooEncoder: Encoder<P = ()> {
      //define new primitive encoding format if need.
      //fn encode_foo<W:WriteStream>(&mut self, w:&mut W, v:&Foo, p:&()) -> Result<usize, Error>;
   }
   pub struct FooEncoderImpl { }
   impl Encoder for FooEncoderImpl {
      type P = ();
      fn encode_bytes<W:WriteStream>(&mut self, w:&mut W, v:&[u8], p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_bool< W:WriteStream>(&mut self, w:&mut W, v:bool,  p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_u8<   W:WriteStream>(&mut self, w:&mut W, v:u8,    p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_u16<  W:WriteStream>(&mut self, w:&mut W, v:u16,   p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_u32<  W:WriteStream>(&mut self, w:&mut W, v:u32,   p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_u64<  W:WriteStream>(&mut self, w:&mut W, v:u64,   p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_i8<   W:WriteStream>(&mut self, w:&mut W, v:i8,    p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_i16<  W:WriteStream>(&mut self, w:&mut W, v:i16,   p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_i32<  W:WriteStream>(&mut self, w:&mut W, v:i32,   p:&Self::P) -> Result<usize, Error> { Ok(0) }
      fn encode_i64<  W:WriteStream>(&mut self, w:&mut W, v:i64,   p:&Self::P) -> Result<usize, Error> { Ok(0) }
   }
   impl FooEncoder for FooEncoderImpl { }
   pub trait FooEncodee {
      fn encode<E:FooEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&E::P) -> Result<usize, Error>;
   }   
   impl <E:FooEncoder, W:WriteStream> Serializer<E,W> {
      pub fn serialize_foo<A:FooEncodee>(&mut self, obj:&A, p:&E::P) -> Result<usize, Error> {
         self.flat_map(|e,w| { obj.encode(e, w, p) })
      }
   }

   // Then, add Encodee implementations to any type you want to encode.
   struct X(u16);
   impl FooEncodee for X {
      fn encode<E:FooEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
         e.encode_u16be(w, self.0, ep)
      }
   }
   // You can implements multiple encoders to one type.
   use super::super::{BitcoinEncoder, BitcoinEncodee};
   impl BitcoinEncodee for X {
      fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
         e.encode_u16le(w, self.0, ep)
      }
   }

   #[test]
   fn test_multiple_encode() {
      use super::super::super::SliceWriteStream;

      let mut f_ser = Serializer::new_with_with(FooEncoderImpl{}, SliceWriteStream::new([0u8;32]));
      let     f_sp  = ();

      use super::super::{BitcoinSerializer, BitcoinEncodeParam};
      let mut b_ser = BitcoinSerializer::new_with(SliceWriteStream::new([0u8;32]));
      let     b_sp  = BitcoinEncodeParam::new_net();

      let val = X(0x1234u16);
      assert_matches!(f_ser.serialize_foo(&val, &f_sp), Ok(2));
      assert_eq!([0x12, 0x34], f_ser.get_ref().get_ref()[0..2]); //big endian
      
      assert_matches!(b_ser.serialize_bitcoin(&val, &b_sp), Ok(2)); //little endian
      assert_eq!([0x34, 0x12], b_ser.get_ref().get_ref()[0..2]);
   }
}

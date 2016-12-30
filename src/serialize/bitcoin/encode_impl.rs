use std;
use ::{Error, UInt256};
use super::super::{Encoder, WriteStream, Serializer};
use super::{BitcoinEncoder, BitcoinEncodeParam, BitcoinEncodee};

#[derive(Default)]
pub struct BitcoinEncoderImpl { }

impl Encoder for BitcoinEncoderImpl {
   type P = BitcoinEncodeParam;
}

macro_rules! def_encode {
   ($n:ident, $t:ty, $size:expr) => ( interpolate_idents! {
      #[inline(always)]
      fn [encode_ $n]<W:WriteStream>(&mut self, v:$t, w:&mut W, _p:&Self::P) -> Result<usize, Error> {
         try!(w.[write_ $n](v));
         Ok($size as usize)
      }
   } )
}
impl BitcoinEncoder for BitcoinEncoderImpl {
   #[inline(always)]
   fn encode_bool<W:WriteStream>(&mut self, v:bool, w:&mut W, _p:&Self::P) -> Result<usize, Error> {
      try!(w.write_u8(if v {1u8} else {0u8}));
      Ok(1usize)
   }

   def_encode!{u8,     u8, 1}
   def_encode!{u16le, u16, 2}
   def_encode!{u32le, u32, 4}
   def_encode!{u64le, u64, 8}
   def_encode!{u16be, u16, 2}
   def_encode!{u32be, u32, 4}
   def_encode!{u64be, u64, 8}

   def_encode!{i8,     i8, 1}
   def_encode!{i16le, i16, 2}
   def_encode!{i32le, i32, 4}
   def_encode!{i64le, i64, 8}
   def_encode!{i16be, i16, 2}
   def_encode!{i32be, i32, 4}
   def_encode!{i64be, i64, 8}

   fn encode_varint<W:WriteStream>(&mut self, v:u64, w:&mut W, _p:&Self::P) -> Result<usize, Error> {
      let mut r = 0usize;
      if v < 253 {
         try!(w.write_u8(v as u8));
         r += 1;
      } else if v <= 0xFFFF {
         try!(w.write_u8(253u8));
         try!(w.write_u16le(v as u16));
         r += 3
      } else if v <= 0xFFFFFFFF {
         try!(w.write_u8(254u8));
         try!(w.write_u32le(v as u32));
         r += 5;
      } else {
         try!(w.write_u8(255u8));
         try!(w.write_u64le(v));
         r += 9;
      }
      Ok(r)
   }

   #[inline(always)]
   fn encode_uint256<W:WriteStream>(&mut self, v:&UInt256, w:&mut W, p:&Self::P) -> Result<usize, Error> {
      self.encode_array_u8(&v.data, w, p)
   }
   
   #[inline(always)]
   fn encode_array_u8<W:WriteStream>(&mut self, v:&[u8], w:&mut W, _p:&Self::P) -> Result<usize, Error> {
      try!(w.write(v));
      Ok(v.len())
   }
   
   #[inline(always)]
   fn encode_sequence_u8<W:WriteStream>(&mut self, v:&[u8], w:&mut W, p:&Self::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.encode_varint(v.len() as u64, w, p));
      try!(w.write(v));
      r += v.len();
      Ok(r)
   }

   #[inline(always)]
   fn encode_sequence<A:BitcoinEncodee, W:WriteStream>(&mut self, v:&[A], w:&mut W, p:&Self::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.encode_varint(v.len() as u64, w, p));
      //r += v.iter().fold(0usize, |acc,obj| { acc + try!(obj.encode(self, w, p)) }
      for obj in v.iter() {
         r += try!(obj.encode(self, w, p));
      }
      Ok(r)
   }

   #[inline(always)]
   fn encode_limited_string<W:WriteStream>(&mut self, v:&str, lim:u32, w:&mut W, p:&Self::P) -> Result<usize, Error> {
      let bytes = v.as_bytes();
      let size  = std::cmp::min(lim as usize, bytes.len());
      let mut r:usize = 0;
      r += try!(self.encode_varint(size as u64, w, p));
      r += try!(self.encode_array_u8(&bytes[0..size], w, p));
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
   assert_matches!(enc.encode_bool(true,  &mut ws, &ep),  Ok(1));
   assert_matches!(enc.encode_bool(false, &mut ws, &ep), Ok(1));
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
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(true, w, &ep) }),  Ok(1));
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(false, w, &ep) }),  Ok(1));
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
      assert_matches!(ser.flat_map(|e,w| { e.encode_bool(true, w, &ep) }),  Ok(1));
      assert_matches!(ser.flat_map(|e,w| { e.encode_bool(false, w, &ep) }),  Ok(1));
      assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
   }
   {
      let mut v = Vec::<u8>::with_capacity(100);
      unsafe { v.set_len(100); }
      let     ws  = SliceWriteStream::new(v);
      let mut ser = BitcoinSerializer::new_with(ws);
      let     ep  = BitcoinEncodeParam::new_net();
      assert_eq!(100, ser.get_ref().get_ref().len());
      assert_matches!(ser.flat_map(|e,w| { e.encode_bool(true, w, &ep) }),  Ok(1));
      assert_matches!(ser.flat_map(|e,w| { e.encode_bool(false, w, &ep) }),  Ok(1));
      assert_eq!(100, ser.get_ref().get_ref().len());
      assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
   }
}

#[test]
fn test_serializer_fixed() {
   let mut ser = FixedBitcoinSerializer::new(100);
   let     ep  = BitcoinEncodeParam::new_net();
   assert_eq!(100, ser.get_ref().get_ref().len());
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(true, w, &ep) }),  Ok(1));
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(false, w, &ep) }),  Ok(1));
   assert_eq!([0x01, 0x00], &ser.get_ref().get_ref()[0..2]);
}

#[test]
fn test_serializer_size() {
   let mut ser = SizeBitcoinSerializer::new();
   let     ep  = BitcoinEncodeParam::new_net();
   assert_eq!(0, ser.size());
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(true, w, &ep) }),  Ok(1));
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(false, w, &ep) }),  Ok(1));
   assert_eq!(2, ser.size());
}

#[test]
fn test_serializer_hash() {
   let mut ser = DHash256BitcoinSerializer::new();
   let     ep  = BitcoinEncodeParam::new_gethash();
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(true, w, &ep) }),  Ok(1));
   assert_matches!(ser.flat_map(|e,w| { e.encode_bool(false, w, &ep) }),  Ok(1));
   assert_eq!("677b2d718464ee0121475600b929c0b4155667486577d1320b18c2dc7d4b4f99", ser.hash_hexresult());
}

#[test]
fn test_varint() {
   use super::super::FixedWriteStream;
   let mut ws  = FixedWriteStream::new(100);
   let mut enc = BitcoinEncoderImpl { };
   let     ep  = BitcoinEncodeParam::new_net();

   assert_matches!(enc.encode_varint(0u64, &mut ws, &ep), Ok(1));
   assert_matches!(enc.encode_varint(252u64, &mut ws, &ep), Ok(1));
   assert_eq!([0, 252], &ws.get_ref()[0..2]);

   ws.reset();
   assert_matches!(enc.encode_varint(253u64, &mut ws, &ep), Ok(3));    //lower limit
   assert_matches!(enc.encode_varint(0x0102u64, &mut ws, &ep), Ok(3)); //endian test
   assert_matches!(enc.encode_varint(0xFFFFu64, &mut ws, &ep), Ok(3)); //higher limit
   assert_eq!([253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF], &ws.get_ref()[0..9]);

   ws.reset();
   assert_matches!(enc.encode_varint(0x10000u64, &mut ws, &ep), Ok(5));
   assert_matches!(enc.encode_varint(0x01020304u64, &mut ws, &ep), Ok(5));
   assert_matches!(enc.encode_varint(0xFFFFFFFFu64, &mut ws, &ep), Ok(5));
   assert_eq!([254, 0x00, 0x00, 0x01, 0x00,
               254, 0x04, 0x03, 0x02, 0x01,
               254, 0xFF, 0xFF, 0xFF, 0xFF], &ws.get_ref()[0..15]);
   ws.reset();
   assert_matches!(enc.encode_varint(0x100000000u64, &mut ws, &ep), Ok(9));
   assert_matches!(enc.encode_varint(0x0102030405060708u64, &mut ws, &ep), Ok(9));
   assert_matches!(enc.encode_varint(0xFFFFFFFFFFFFFFFFu64, &mut ws, &ep), Ok(9));
   assert_eq!([255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], &ws.get_ref()[0..27]);
}

#[cfg(test)]
mod test {
   // If you want to implements new encoding format, define dedicated Encoder, EncoderImpl, Encodee, Serializer.
   // Note that the Serializer have a serialize_<encoding name> method which is given Encodee object.
   use ::Error;
   use super::super::super::{Encoder, WriteStream, Serializer};
   pub trait FooEncoder: Encoder<P = ()> {
      //declare encoders for primitive encoding types of this format.
      fn encode_u16be<  W:WriteStream>(&mut self, v:u16, w:&mut W, _p:&Self::P) -> Result<usize, Error>;
   }
   pub struct FooEncoderImpl { }
   impl Encoder for FooEncoderImpl {
      type P = ();
   }
   impl FooEncoder for FooEncoderImpl {
      fn encode_u16be<  W:WriteStream>(&mut self, v:u16, w:&mut W, _p:&Self::P) -> Result<usize, Error> {
         try!(w.write_u16be(v));
         Ok(2)
      }
   }
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
         e.encode_u16be(self.0, w, ep)
      }
   }
   // You can implements multiple encoders to one type.
   use super::super::{BitcoinEncoder, BitcoinEncodee};
   impl BitcoinEncodee for X {
      fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
         e.encode_u16le(self.0, w, ep)
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

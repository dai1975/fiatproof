use ::Error;
use super::super::{Encoder, Encodee, WriteStream, Serializer };
use super::BitcoinEncodeParam;

pub trait BitcoinEncoder: Encoder<P = BitcoinEncodeParam> {
   fn encode_varint<W:WriteStream>(&mut self, w:&mut W, v:u64, p:&BitcoinEncodeParam) -> Result<usize, Error>;
}

impl <E:BitcoinEncoder, W:WriteStream> Serializer<E,W> {
   pub fn serialize_varint(&mut self, v:u64, p:&E::P) -> Result<usize, Error> {
      //self.e.encode_varint(&mut self.w, v, p) // cannot access to private members because this impl is treated as external.
      self.encode(|e,w| { e.encode_varint(w, v, p) })
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

pub type BitcoinEncodee = Encodee<BitcoinEncoder<P = BitcoinEncodeParam>>; // it seems not be required that P=compiler. compiler bug?
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


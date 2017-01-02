use ::{Error, UInt256};
use super::{BitcoinEncoder, BitcoinEncodeParam};
use super::{WriteStream};

pub struct BitcoinSerializer<W:WriteStream> {
   w: W,
   p: BitcoinEncodeParam,
}
impl <W:WriteStream> BitcoinSerializer<W> {
   pub fn new_with(w:W) -> Self { BitcoinSerializer {w:w, p:BitcoinEncodeParam::new()} }
   pub fn writestream(&self) -> &W { &self.w }
   pub fn mut_param(&mut self) -> &mut BitcoinEncodeParam { &mut self.p }
}

macro_rules! def_encode {
   ($n:ident, $t:ty, $size:expr) => ( interpolate_idents! {
      #[inline(always)]
      fn [encode_ $n](&mut self, v:$t) -> Result<usize, Error> {
         try!(self.w.[write_ $n](v));
         Ok($size as usize)
      }
   } )
}

impl <W:WriteStream> BitcoinEncoder for BitcoinSerializer<W> {
   fn param(&self) -> &BitcoinEncodeParam { &self.p }
   
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

   #[inline(always)]
   fn encode_bool(&mut self, v:bool) -> Result<usize, Error> {
      try!(self.w.write_u8(if v {1u8} else {0u8}));
      Ok(1usize)
   }

   fn encode_varint(&mut self, v:u64) -> Result<usize, Error> {
      if v < 253 {
         try!(self.w.write_u8(v as u8));
         Ok(1)
      } else if v <= 0xFFFF {
         try!(self.w.write_u8(253u8));
         try!(self.w.write_u16le(v as u16));
         Ok(3)
      } else if v <= 0xFFFFFFFF {
         try!(self.w.write_u8(254u8));
         try!(self.w.write_u32le(v as u32));
         Ok(5)
      } else {
         try!(self.w.write_u8(255u8));
         try!(self.w.write_u64le(v));
         Ok(9)
      }
   }

   #[inline(always)]
   fn encode_uint256(&mut self, v:&UInt256) -> Result<usize, Error> {
      self.encode_array_u8(&v.data)
   }
   
   #[inline(always)]
   fn encode_array_u8(&mut self, v:&[u8]) -> Result<usize, Error> {
      try!(self.w.write(v));
      Ok(v.len())
   }
   
   #[inline(always)]
   fn encode_sequence_u8(&mut self, v:&[u8]) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.encode_varint(v.len() as u64));
      try!(self.w.write(v));
      r += v.len();
      Ok(r)
   }
}

//use super::BitcoinEncodee;
//pub trait BitcoinSerializee<W:WriteStream> = BitcoinEncodee<BitcoinSerializer<W>>;

use std::borrow::BorrowMut;
use super::SliceWriteStream;
pub type SliceBitcoinSerializer<T: BorrowMut<[u8]>> = BitcoinSerializer<SliceWriteStream<T>>;
impl <T: BorrowMut<[u8]>> SliceBitcoinSerializer<T> {
   pub fn new(inner:T) -> Self { BitcoinSerializer::new_with( SliceWriteStream::new(inner) ) }
   pub fn len(&self) -> usize { self.w.len() }
   pub fn as_slice(&self) -> &[u8] { self.w.as_slice() }
   pub fn rewind(&mut self) { self.w.rewind() }
}

use super::FixedWriteStream;
pub type FixedBitcoinSerializer = BitcoinSerializer<FixedWriteStream>;
impl FixedBitcoinSerializer {
   pub fn new(size:usize) -> Self { BitcoinSerializer::new_with( FixedWriteStream::new(size) ) }
   pub fn len(&self) -> usize { self.w.len() }
   pub fn as_slice(&self) -> &[u8] { self.w.as_slice() }
   pub fn rewind(&mut self) { self.w.rewind() }
}

use super::write_stream::SizeSink;
pub type SizeBitcoinSerializer = BitcoinSerializer<SizeSink>;
impl SizeBitcoinSerializer {
   pub fn new() -> Self { BitcoinSerializer::new_with(SizeSink::new()) }
   pub fn size(&self) -> usize { self.w.size() }
   pub fn rewind(&mut self) { self.w.rewind() }
}

use super::HashWriteStream;
use ::crypto::DHash256;
pub type DHash256BitcoinSerializer = BitcoinSerializer<HashWriteStream<DHash256>>;
impl DHash256BitcoinSerializer {
   pub fn new() -> Self { Self::new_with(HashWriteStream::new(DHash256::default())) }
   pub fn hash_result(&mut self) -> Box<[u8]> { self.w.result() }
   pub fn hash_hexresult(&mut self) -> String { self.w.hexresult() }
   pub fn rewind(&mut self) { self.w.rewind() }
}

#[test]
fn test_cursor_vec() {
   use std::io::Cursor;
   let mut ser = BitcoinSerializer::new_with(Cursor::new(Vec::<u8>::with_capacity(100)));
   
   assert_eq!(0, ser.writestream().get_ref().len());
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!(2, ser.writestream().get_ref().len());
   assert_eq!([0x01, 0x00], &ser.writestream().get_ref()[0..2]);
}

#[test]
fn test_slice() {
   {
      let mut ser = SliceBitcoinSerializer::new([0u8; 32]);
      assert_eq!(32, ser.len());
      assert_matches!(ser.encode_bool(true),  Ok(1));
      assert_matches!(ser.encode_bool(false), Ok(1));
      assert_eq!(32, ser.len());
      assert_eq!([0x01, 0x00], &ser.as_slice()[0..2]);
   }
   {
      let mut v = Vec::<u8>::with_capacity(100);
      unsafe { v.set_len(100); }
      let mut ser = SliceBitcoinSerializer::new(v);
      assert_eq!(100, ser.len());
      assert_matches!(ser.encode_bool(true),  Ok(1));
      assert_matches!(ser.encode_bool(false), Ok(1));
      assert_eq!(100, ser.len());
      assert_eq!([0x01, 0x00], &ser.as_slice()[0..2]);
   }
}

#[test]
fn test_serializer_fixed() {
   let mut ser = FixedBitcoinSerializer::new(100);
   assert_eq!(100, ser.len());
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!(100, ser.len());
   assert_eq!([0x01, 0x00], &ser.as_slice()[0..2]);
}

#[test]
fn test_serializer_size() {
   let mut ser = SizeBitcoinSerializer::new();
   assert_eq!(0, ser.size());
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!(2, ser.size());
}

#[test]
fn test_serializer_hash() {
   let mut ser = DHash256BitcoinSerializer::new();
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!("677b2d718464ee0121475600b929c0b4155667486577d1320b18c2dc7d4b4f99", ser.hash_hexresult());
}

#[test]
fn test_varint() {
   let mut ser = FixedBitcoinSerializer::new(100);

   assert_matches!(ser.encode_varint(0u64), Ok(1));
   assert_matches!(ser.encode_varint(252u64), Ok(1));
   assert_eq!([0, 252], &ser.as_slice()[0..2]);

   ser.rewind();
   assert_matches!(ser.encode_varint(253u64), Ok(3));    //lower limit
   assert_matches!(ser.encode_varint(0x0102u64), Ok(3)); //endian test
   assert_matches!(ser.encode_varint(0xFFFFu64), Ok(3)); //higher limit
   assert_eq!([253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF], &ser.as_slice()[0..9]);

   ser.rewind();
   assert_matches!(ser.encode_varint(0x10000u64), Ok(5));
   assert_matches!(ser.encode_varint(0x01020304u64), Ok(5));
   assert_matches!(ser.encode_varint(0xFFFFFFFFu64), Ok(5));
   assert_eq!([254, 0x00, 0x00, 0x01, 0x00,
               254, 0x04, 0x03, 0x02, 0x01,
               254, 0xFF, 0xFF, 0xFF, 0xFF], &ser.as_slice()[0..15]);
   ser.rewind();
   assert_matches!(ser.encode_varint(0x100000000u64), Ok(9));
   assert_matches!(ser.encode_varint(0x0102030405060708u64), Ok(9));
   assert_matches!(ser.encode_varint(0xFFFFFFFFFFFFFFFFu64), Ok(9));
   assert_eq!([255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], &ser.as_slice()[0..27]);
}


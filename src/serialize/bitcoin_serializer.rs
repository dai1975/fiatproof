use ::Error;
use super::{WriteStream,Encoder};
           
pub struct BitcoinSerializer<T:WriteStream> {
   ws: T,
}

impl <T:WriteStream> BitcoinSerializer<T> {
   pub fn new(ws:T) -> Self { BitcoinSerializer { ws:ws } }

   pub fn inner(self) -> T { self.ws }
   pub fn get_ref(&self) -> &T { &self.ws }
   pub fn get_mut(&mut self) -> &mut T { &mut self.ws }
}

impl <T:WriteStream> Encoder for BitcoinSerializer<T> {
   #[inline(always)]
   fn encode_bool(&mut self, v:bool) -> Result<usize, Error> {
      try!(self.ws.write_u8(if v {1u8} else {0u8}));
      Ok(1usize)
   }
   #[inline(always)]
   fn encode_u8(&mut self, v:u8) -> Result<usize, Error> {
      try!(self.ws.write_u8(v));
      Ok(1usize)
   }
   #[inline(always)]
   fn encode_u16(&mut self, v:u16) -> Result<usize, Error> {
      try!(self.ws.write_u16le(v));
      Ok(2usize)
   }
   #[inline(always)]
   fn encode_u32(&mut self, v:u32) -> Result<usize, Error> {
      try!(self.ws.write_u32le(v));
      Ok(4usize)
   }
   #[inline(always)]
   fn encode_u64(&mut self, v:u64) -> Result<usize, Error> {
      try!(self.ws.write_u64le(v));
      Ok(8usize)
   }
   #[inline(always)]
   fn encode_i8(&mut self, v:i8) -> Result<usize, Error> {
      try!(self.ws.write_i8(v));
      Ok(1usize)
   }
   #[inline(always)]
   fn encode_i16(&mut self, v:i16) -> Result<usize, Error> {
      try!(self.ws.write_i16le(v));
      Ok(2usize)
   }
   #[inline(always)]
   fn encode_i32(&mut self, v:i32) -> Result<usize, Error> {
      try!(self.ws.write_i32le(v));
      Ok(4usize)
   }
   #[inline(always)]
   fn encode_i64(&mut self, v:i64) -> Result<usize, Error> {
      try!(self.ws.write_i64le(v));
      Ok(8usize)
   }
}

#[test]
fn test_vec() {
   use std::io::Cursor;
   let ws = Cursor::new(Vec::<u8>::with_capacity(100));
   let mut enc = BitcoinSerializer::new(ws);
   assert_matches!(enc.encode_bool(true),  Ok(1));
   assert_matches!(enc.encode_bool(false), Ok(1));
   assert_eq!([0x01, 0x00], &enc.get_ref().get_ref()[0..2]);
}

#[test]
fn test_slice_out_stream() {
   use super::SliceWriteStream;
   {
      let ws = SliceWriteStream::new([0u8;32]);
      let mut enc = BitcoinSerializer::new(ws);
      assert_matches!(enc.encode_bool(true), Ok(1));
      assert_matches!(enc.encode_bool(false), Ok(1));
      assert_eq!([0x01, 0x00], &enc.get_ref().get_ref()[0..2]);
   }
   {
      let mut v = Vec::<u8>::with_capacity(100);
      unsafe { v.set_len(100); }
      let mut enc = BitcoinSerializer::new(SliceWriteStream::new(v));
      assert_matches!(enc.encode_bool(true),  Ok(1));
      assert_matches!(enc.encode_bool(false), Ok(1));
      assert_eq!([0x01, 0x00], &enc.get_ref().get_ref()[0..2]);
   }
}


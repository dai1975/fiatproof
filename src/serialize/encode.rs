use ::Error;
use super::{WriteStream, FixedWriteStream};

pub trait Encoder {
   type P;
   fn encode_bool< W:WriteStream>(&mut self, v:bool,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u8<   W:WriteStream>(&mut self, v:u8,    w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u16<  W:WriteStream>(&mut self, v:u16,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u32<  W:WriteStream>(&mut self, v:u32,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u64<  W:WriteStream>(&mut self, v:u64,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i8<   W:WriteStream>(&mut self, v:i8,    w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i16<  W:WriteStream>(&mut self, v:i16,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i32<  W:WriteStream>(&mut self, v:i32,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i64<  W:WriteStream>(&mut self, v:i64,   w:&mut W, p:&Self::P) -> Result<usize, Error>;

   fn encode_u16le<W:WriteStream>(&mut self, v:u16,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_u16le(v)); Ok(2) }
   fn encode_u32le<W:WriteStream>(&mut self, v:u32,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_u32le(v)); Ok(4) }
   fn encode_u64le<W:WriteStream>(&mut self, v:u64,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_u64le(v)); Ok(8) }
   fn encode_i16le<W:WriteStream>(&mut self, v:i16,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_i16le(v)); Ok(2) }
   fn encode_i32le<W:WriteStream>(&mut self, v:i32,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_i32le(v)); Ok(4) }
   fn encode_i64le<W:WriteStream>(&mut self, v:i64,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_i64le(v)); Ok(8) }

   fn encode_u16be<W:WriteStream>(&mut self, v:u16,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_u16be(v)); Ok(2) }
   fn encode_u32be<W:WriteStream>(&mut self, v:u32,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_u32be(v)); Ok(4) }
   fn encode_u64be<W:WriteStream>(&mut self, v:u64,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_u64be(v)); Ok(8) }
   fn encode_i16be<W:WriteStream>(&mut self, v:i16,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_i16be(v)); Ok(2) }
   fn encode_i32be<W:WriteStream>(&mut self, v:i32,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_i32be(v)); Ok(4) }
   fn encode_i64be<W:WriteStream>(&mut self, v:i64,  w:&mut W, _p:&Self::P) -> Result<usize, Error> { try!(w.write_i64be(v)); Ok(8) }
}

pub struct Serializer<E:Encoder, W:WriteStream> {
   e: E,
   w: W,
}

impl <E:Encoder, W:WriteStream> Serializer<E,W> {
   pub fn new_with_with(e:E, w:W) -> Self { Serializer { e:e, w:w } }
   pub fn inner(self) -> W { self.w }
   pub fn get_ref(&self) -> &W { &self.w }
   pub fn get_mut(&mut self) -> &mut W { &mut self.w }

   #[inline(always)]
   pub fn flat_map<R,F>(&mut self, mut f:F) -> R
      where F: FnMut(&mut E, &mut W) -> R
   {
      f(&mut self.e, &mut self.w)
   }

   #[inline(always)]
   pub fn serialize_bool(&mut self, v:bool, p:&E::P) -> Result<usize, Error> {
      self.e.encode_bool(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_u8(&mut self, v:u8, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u8(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_u16(&mut self, v:u16, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u16(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_u32(&mut self, v:u32, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u32(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_u64(&mut self, v:u64, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u64(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_i8(&mut self, v:i8, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i8(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_i16(&mut self, v:i16, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i16(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_i32(&mut self, v:i32, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i32(v, &mut self.w, p)
   }
   #[inline(always)]
   pub fn serialize_i64(&mut self, v:i64, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i64(v, &mut self.w, p)
   }
}

pub type FixedSerializer<E:Encoder> = Serializer<E, FixedWriteStream>;
impl <E:Encoder> FixedSerializer<E> {
   pub fn new_fixed(e:E, size:usize) -> Self {
      Self::new_with_with(e, FixedWriteStream::new(size))
   }
}

use super::write_stream::SizeSink;
pub type SizeSerializer<E:Encoder> = Serializer<E, SizeSink>;
impl <E:Encoder> SizeSerializer<E> {
   pub fn new_size(e:E) -> Self {
      Self::new_with_with(e, SizeSink::new())
   }
   pub fn reset_size(&mut self) { self.get_mut().reset_size(); }
   pub fn size(&self) -> usize { self.get_ref().size() }
}

use super::HashWriteStream;
use ::crypto::{ Hasher, DHash256 };
pub type HashSerializer<E:Encoder, H:Hasher> = Serializer<E, HashWriteStream<H>>;
impl <E:Encoder, H:Hasher> HashSerializer<E,H> {
   pub fn new_with_default(e:E) -> Self {
      Self::new_with_with(e, HashWriteStream::new(H::default()))
   }
   pub fn hash_reset(&mut self) { self.get_mut().reset() }
   pub fn hash_result(&mut self) -> Box<[u8]> { self.get_mut().result() }
   pub fn hash_hexresult(&mut self) -> String { self.get_mut().hexresult() }
}
pub type DHash256Serializer<E:Encoder> = HashSerializer<E, DHash256>;


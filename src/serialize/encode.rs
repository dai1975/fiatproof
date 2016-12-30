use ::Error;
use super::{WriteStream, FixedWriteStream};

pub trait Encoder {
   type P;
   fn encode_bytes<W:WriteStream>(&mut self, w:&mut W, v:&[u8], p:&Self::P) -> Result<usize, Error>;
   fn encode_bool< W:WriteStream>(&mut self, w:&mut W, v:bool,  p:&Self::P) -> Result<usize, Error>;
   fn encode_u8<   W:WriteStream>(&mut self, w:&mut W, v:u8,    p:&Self::P) -> Result<usize, Error>;
   fn encode_u16<  W:WriteStream>(&mut self, w:&mut W, v:u16,   p:&Self::P) -> Result<usize, Error>;
   fn encode_u32<  W:WriteStream>(&mut self, w:&mut W, v:u32,   p:&Self::P) -> Result<usize, Error>;
   fn encode_u64<  W:WriteStream>(&mut self, w:&mut W, v:u64,   p:&Self::P) -> Result<usize, Error>;
   fn encode_i8<   W:WriteStream>(&mut self, w:&mut W, v:i8,    p:&Self::P) -> Result<usize, Error>;
   fn encode_i16<  W:WriteStream>(&mut self, w:&mut W, v:i16,   p:&Self::P) -> Result<usize, Error>;
   fn encode_i32<  W:WriteStream>(&mut self, w:&mut W, v:i32,   p:&Self::P) -> Result<usize, Error>;
   fn encode_i64<  W:WriteStream>(&mut self, w:&mut W, v:i64,   p:&Self::P) -> Result<usize, Error>;
}

pub trait Encodee<E:Encoder> {
   fn encode<W:WriteStream>(&self, w:&mut W, e:&mut E, ep:&E::P) -> Result<usize, Error>;
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
   pub fn encode<R,F>(&mut self, mut f:F) -> R
      where F: FnMut(&mut E, &mut W) -> R
   {
      f(&mut self.e, &mut self.w)
   }

   #[inline(always)]
   pub fn serialize<A:Encodee<E>>(&mut self, obj:&A, p:&E::P) -> Result<usize, Error> {
      obj.encode(&mut self.w, &mut self.e, p)
   }
   #[inline(always)]
   pub fn serialize_bytes(&mut self, v:&[u8], p:&E::P) -> Result<usize, Error> {
      self.e.encode_bytes(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_bool(&mut self, v:bool, p:&E::P) -> Result<usize, Error> {
      self.e.encode_bool(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_u8(&mut self, v:u8, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u8(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_u16(&mut self, v:u16, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u16(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_u32(&mut self, v:u32, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u32(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_u64(&mut self, v:u64, p:&E::P) -> Result<usize, Error> {
      self.e.encode_u64(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_i8(&mut self, v:i8, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i8(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_i16(&mut self, v:i16, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i16(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_i32(&mut self, v:i32, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i32(&mut self.w, v, p)
   }
   #[inline(always)]
   pub fn serialize_i64(&mut self, v:i64, p:&E::P) -> Result<usize, Error> {
      self.e.encode_i64(&mut self.w, v, p)
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





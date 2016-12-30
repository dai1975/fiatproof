use ::Error;
use super::{WriteStream, FixedWriteStream};

pub trait Encoder: Default {
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
   pub fn new(w:W) -> Self { Serializer { e:E::default(), w:w } }
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
   pub fn new_with_size(size:usize) -> Self {
      FixedSerializer::new(FixedWriteStream::new(size))
   }
}

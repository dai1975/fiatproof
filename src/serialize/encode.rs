use super::{WriteStream, FixedWriteStream};

pub trait Encoder {
   type P;
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
}

pub type FixedSerializer<E:Encoder> = Serializer<E, FixedWriteStream>;
impl <E:Encoder> FixedSerializer<E> {
   pub fn new_fixed(e:E, size:usize) -> Self {
      Self::new_with_with(e, FixedWriteStream::new(size))
   }
   pub fn get_ref_ref(&self) -> &[u8] { self.w.get_ref() }
   pub fn reset(&mut self) { self.w.reset() }
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


use ::{Error, UInt256};
use super::super::{Encoder, WriteStream, Serializer };
use super::BitcoinEncodeParam;

pub trait BitcoinEncoder: Encoder<P = BitcoinEncodeParam> {
   fn encode_bool< W:WriteStream>(&mut self, v:bool, w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u8<   W:WriteStream>(&mut self, v:u8,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i8<   W:WriteStream>(&mut self, v:i8,   w:&mut W, p:&Self::P) -> Result<usize, Error>;

   fn encode_u16le<W:WriteStream>(&mut self, v:u16,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u32le<W:WriteStream>(&mut self, v:u32,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u64le<W:WriteStream>(&mut self, v:u64,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i16le<W:WriteStream>(&mut self, v:i16,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i32le<W:WriteStream>(&mut self, v:i32,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i64le<W:WriteStream>(&mut self, v:i64,  w:&mut W, p:&Self::P) -> Result<usize, Error>;

   fn encode_u16be<W:WriteStream>(&mut self, v:u16,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u32be<W:WriteStream>(&mut self, v:u32,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_u64be<W:WriteStream>(&mut self, v:u64,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i16be<W:WriteStream>(&mut self, v:i16,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i32be<W:WriteStream>(&mut self, v:i32,  w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_i64be<W:WriteStream>(&mut self, v:i64,  w:&mut W, p:&Self::P) -> Result<usize, Error>;

   fn encode_varint        <W:WriteStream>(&mut self, v:u64,           w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_uint256       <W:WriteStream>(&mut self, v:&UInt256,      w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_array_u8      <W:WriteStream>(&mut self, v:&[u8],         w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_sequence_u8   <W:WriteStream>(&mut self, v:&[u8],         w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_limited_string<W:WriteStream>(&mut self, v:&str, lim:u32, w:&mut W, p:&Self::P) -> Result<usize, Error>;

   fn encode         <W:WriteStream, A:BitcoinEncodee>(&mut self, obj:&A,   w:&mut W, p:&Self::P) -> Result<usize, Error>;
   fn encode_sequence<W:WriteStream, A:BitcoinEncodee>(&mut self, ary:&[A], w:&mut W, p:&Self::P) -> Result<usize, Error>;
}

pub trait BitcoinEncodee {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&E::P) -> Result<usize, Error>;
}   

impl <E:BitcoinEncoder, W:WriteStream> Serializer<E,W> {
   // You would to use flat_map directly, but I define wrapper functions for convinience.
   #[inline(always)]
   pub fn serialize_bitcoin<A:BitcoinEncodee>(&mut self, obj:&A, p:&E::P) -> Result<usize, Error> {
      self.flat_map(|e,w| { obj.encode(e, w, p) })
   }
}


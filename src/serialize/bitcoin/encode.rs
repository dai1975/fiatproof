use ::{Error, UInt256};
use super::super::{Encoder, WriteStream, Serializer };
use super::BitcoinEncodeParam;

pub trait BitcoinEncoder: Encoder<P = BitcoinEncodeParam> {
   fn encode_array_u8<W:WriteStream>(&mut self, v:&[u8], w:&mut W, p:&BitcoinEncodeParam) -> Result<usize, Error>;
   fn encode_varint<W:WriteStream>(&mut self, v:u64, w:&mut W, p:&BitcoinEncodeParam) -> Result<usize, Error>;
   fn encode_sequence_u8<W:WriteStream>(&mut self, v:&[u8], w:&mut W, p:&BitcoinEncodeParam) -> Result<usize, Error>;
   fn encode_sequence<A:BitcoinEncodee, W:WriteStream>(&mut self, v:&[A], w:&mut W, p:&BitcoinEncodeParam) -> Result<usize, Error>;
   fn encode_uint256<W:WriteStream>(&mut self, v:&UInt256, w:&mut W, p:&BitcoinEncodeParam) -> Result<usize, Error>;
   fn encode_limited_string<W:WriteStream>(&mut self, v:&str, lim:u32, w:&mut W, p:&BitcoinEncodeParam) -> Result<usize, Error>;
}

pub trait BitcoinEncodee {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&E::P) -> Result<usize, Error>;
}   

impl <E:BitcoinEncoder, W:WriteStream> Serializer<E,W> {
   #[inline(always)]
   pub fn serialize_bitcoin<A:BitcoinEncodee>(&mut self, obj:&A, p:&E::P) -> Result<usize, Error> {
      self.flat_map(|e,w| { obj.encode(e, w, p) })
   }
}


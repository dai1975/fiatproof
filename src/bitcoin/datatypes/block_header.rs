use super::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockHeader {
   pub version: i32,
   pub hash_prev_block: UInt256,
   pub hash_merkle_root: UInt256,
   pub time: u32,
   pub bits: u32,
   pub nonce: u32,
}

impl ::std::fmt::Display for BlockHeader {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "BlockHeader(version={}, prev={}, merkle={}, time={}, bits={}, nonce={})",
             self.version, self.hash_prev_block, self.hash_merkle_root, self.time, self.bits, self.nonce)
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for BlockHeader {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(ws, self.version));
      r += try!(self.hash_prev_block.encode(&(), e, ws));
      r += try!(self.hash_merkle_root.encode(&(), e, ws));
      r += try!(e.encode_u32le(ws, self.time));
      r += try!(e.encode_u32le(ws, self.bits));
      r += try!(e.encode_u32le(ws, self.nonce));
      Ok(r)
   }
}
impl BitcoinDecodee for BlockHeader {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(rs, &mut self.version));
      r += try!(self.hash_prev_block.decode(&(), d, rs));
      r += try!(self.hash_merkle_root.decode(&(), d, rs));
      r += try!(d.decode_u32le(rs, &mut self.time));
      r += try!(d.decode_u32le(rs, &mut self.bits));
      r += try!(d.decode_u32le(rs, &mut self.nonce));
      Ok(r)
   }
}

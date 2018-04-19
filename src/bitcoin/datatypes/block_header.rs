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

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for BlockHeader {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(self.hash_prev_block.encode(e));
      r += try!(self.hash_merkle_root.encode(e));
      r += try!(e.encode_u32le(self.time));
      r += try!(e.encode_u32le(self.bits));
      r += try!(e.encode_u32le(self.nonce));
      Ok(r)
   }
}
impl BitcoinDecodee for BlockHeader {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(self.hash_prev_block.decode(d));
      r += try!(self.hash_merkle_root.decode(d));
      r += try!(d.decode_u32le(&mut self.time));
      r += try!(d.decode_u32le(&mut self.bits));
      r += try!(d.decode_u32le(&mut self.nonce));
      Ok(r)
   }
}

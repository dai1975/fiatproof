use std;
use ::bitcoin::datatypes::MerkleBlock;

#[derive(Debug,Default,Clone)]
pub struct MerkleBlockMessage {
   pub block : MerkleBlock,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for MerkleBlockMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x00];
}

impl std::fmt::Display for MerkleBlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(block={})", self.block)
   }
}


use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for MerkleBlockMessage {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.encode(&(), e, ws));
      Ok(r)
   }
}
impl BitcoinDecodee for MerkleBlockMessage {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.decode(&(), d, rs));
      Ok(r)
   }
}

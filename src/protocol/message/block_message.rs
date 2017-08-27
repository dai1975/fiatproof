use std;
use ::Block;

#[derive(Debug,Default,Clone)]
pub struct BlockMessage {
   pub block: Block,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for BlockMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for BlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block({})", self.block)
   }
}


use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for BlockMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.encode(e));
      Ok(r)
   }
}
impl BitcoinDecodee for BlockMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.decode(d));
      Ok(r)
   }
}

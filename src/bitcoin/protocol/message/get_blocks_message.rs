use std;
use ::bitcoin::datatypes::{UInt256, BlockLocator};

#[derive(Debug,Default)]
pub struct GetBlocksMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetBlocksMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for GetBlocksMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetBlocks(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for GetBlocksMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.encode(&(), e, ws));
      r += try!(self.hash_stop.encode(&(), e, ws));
      Ok(r)
   }
}
impl BitcoinDecodee for GetBlocksMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.decode(&(), d, rs));
      r += try!(self.hash_stop.decode(&(), d, rs));
      Ok(r)
   }
}

use std;
use ::bitcoin::datatypes::{UInt256, BlockLocator};

#[derive(Debug,Default)]
pub struct GetHeadersMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetHeadersMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00];
}

impl GetHeadersMessage {
   pub fn new(hash: &UInt256) -> GetHeadersMessage {
      GetHeadersMessage {
         locator   : BlockLocator::default(),
         hash_stop : hash.clone(),
      }
   }
}

impl std::fmt::Display for GetHeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetHeaders(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for GetHeadersMessage {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.encode(&(), e, ws));
      r += try!(self.hash_stop.encode(&(), e, ws));
      Ok(r)
   }
}
impl BitcoinDecodee for GetHeadersMessage {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.decode(&(), d, rs));
      r += try!(self.hash_stop.decode(&(), d, rs));
      Ok(r)
   }
}

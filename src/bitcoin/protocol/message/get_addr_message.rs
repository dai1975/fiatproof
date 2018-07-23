use std;

#[derive(Debug,Default,Clone)]
pub struct GetAddrMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetAddrMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for GetAddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetAddr()")
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for GetAddrMessage {
   fn encode(&self, _e:&mut BitcoinEncoder) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDecodee for GetAddrMessage {
   fn decode(&mut self, _d:&mut BitcoinDecoder) -> ::Result<usize> {
      Ok(0usize)
   }
}

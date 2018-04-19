use std;

#[derive(Debug,Default,Clone)]
pub struct VerAckMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for VerAckMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for VerAckMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "VerAck()")
   }
}


use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for VerAckMessage {
   fn encode(&self, _e:&mut BitcoinEncoder) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDecodee for VerAckMessage {
   fn decode(&mut self, _d:&mut BitcoinDecoder) -> ::Result<usize> {
      Ok(0usize)
   }
}

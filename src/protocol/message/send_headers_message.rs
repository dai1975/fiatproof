use std;

#[derive(Debug,Default,Clone)]
pub struct SendHeadersMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for SendHeadersMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x73, 0x65, 0x6e, 0x64, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00];
}

impl std::fmt::Display for SendHeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "SendHeaders()")
   }
}

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for SendHeadersMessage {
   fn encode(&self, _e:&mut BitcoinEncoder) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDecodee for SendHeadersMessage {
   fn decode(&mut self, _d:&mut BitcoinDecoder) -> ::Result<usize> {
      Ok(0usize)
   }
}

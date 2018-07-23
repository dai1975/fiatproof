use std;

#[derive(Debug,Default,Clone)]
pub struct FilterClearMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for FilterClearMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x00];
}

impl std::fmt::Display for FilterClearMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterClear()")
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for FilterClearMessage {
   fn encode(&self, _e:&mut BitcoinEncoder) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDecodee for FilterClearMessage {
   fn decode(&mut self, _d:&mut BitcoinDecoder) -> ::Result<usize> {
      Ok(0usize)
   }
}

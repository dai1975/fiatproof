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

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for SendHeadersMessage {
   type P = ();
   fn encode(&self, _p:&Self::P, _e:&BitcoinEncoder, _ws:&mut WriteStream) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDecodee for SendHeadersMessage {
   type P = ();
   fn decode(&mut self, _p:&Self::P, _d:&BitcoinDecoder, _rs:&mut ReadStream) -> ::Result<usize> {
      Ok(0usize)
   }
}

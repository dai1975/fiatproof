use std;

#[derive(Debug,Default,Clone)]
pub struct MemPoolMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for MemPoolMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x6d, 0x65, 0x6d, 0x70, 0x6f, 0x6f, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for MemPoolMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MemPool()")
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for MemPoolMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDecodee for MemPoolMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      Ok(0usize)
   }
}

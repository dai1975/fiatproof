use std;
use super::PingMessage;

#[derive(Debug,Default,Clone)]
pub struct PongMessage
{
   pub nonce: u64,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for PongMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x70, 0x6f, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for PongMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Pong(nonce={})", self.nonce)
   }
}

impl PongMessage {
   pub fn new(ping:&PingMessage) -> PongMessage {
      PongMessage{ nonce: ping.nonce }
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for PongMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < e.medium().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl BitcoinDecodee for PongMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < d.medium().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

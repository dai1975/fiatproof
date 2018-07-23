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

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for PongMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < e.medium().version() {
         r += try!(e.encode_u64le(ws, self.nonce));
      }
      Ok(r)
   }
}
impl BitcoinDecodee for PongMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < d.medium().version() {
         r += try!(d.decode_u64le(rs, &mut self.nonce));
      }
      Ok(r)
   }
}

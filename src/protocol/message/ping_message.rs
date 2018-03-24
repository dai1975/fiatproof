use std;
extern crate rand;

#[derive(Debug,Default,Clone)]
pub struct PingMessage
{
   pub nonce: u64,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for PingMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x70, 0x69, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for PingMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Ping(nonce={})", self.nonce)
   }
}

impl PingMessage {
   pub fn reset_nonce(&mut self) {
      use self::rand::Rng;
      let mut rng = rand::os::OsRng::new().unwrap(); // This rng is cryptographic level, is it too secure?
      self.nonce = rng.next_u64();
   }
}

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for PingMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < e.medium().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl BitcoinDecodee for PingMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < d.medium().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

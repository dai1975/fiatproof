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
      use rand::Rng;
      let mut rng = rand::rngs::OsRng::new().unwrap(); // This rng is cryptographic level, is it too secure?
      self.nonce = rng.gen::<u64>();
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for PingMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < e.medium().version() {
         r += e.serialize_u64le(ws, self.nonce)?;
      }
      Ok(r)
   }
}
impl BitcoinDeserializee for PingMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < d.medium().version() {
         r += d.deserialize_u64le(rs, &mut self.nonce)?;
      }
      Ok(r)
   }
}

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

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for PongMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < e.medium().version() {
         r += try!(e.serialize_u64le(ws, self.nonce));
      }
      Ok(r)
   }
}
impl BitcoinDeserializee for PongMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::BIP0031_VERSION;
      if BIP0031_VERSION < d.medium().version() {
         r += try!(d.deserialize_u64le(rs, &mut self.nonce));
      }
      Ok(r)
   }
}

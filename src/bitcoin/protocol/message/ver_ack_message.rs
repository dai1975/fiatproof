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


use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for VerAckMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, _e:&BitcoinSerializer, _ws:&mut WriteStream) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDeserializee for VerAckMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, _d:&BitcoinDeserializer, _rs:&mut ReadStream) -> ::Result<usize> {
      Ok(0usize)
   }
}

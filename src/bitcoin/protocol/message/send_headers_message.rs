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

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for SendHeadersMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, _e:&BitcoinSerializer, _ws:&mut W) -> crate::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDeserializee for SendHeadersMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, _d:&BitcoinDeserializer, _rs:&mut R) -> crate::Result<usize> {
      Ok(0usize)
   }
}

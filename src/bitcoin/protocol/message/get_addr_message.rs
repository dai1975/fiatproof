use std;

#[derive(Debug,Default,Clone)]
pub struct GetAddrMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetAddrMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for GetAddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetAddr()")
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for GetAddrMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, _e:&BitcoinSerializer, _ws:&mut WriteStream) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDeserializee for GetAddrMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, _d:&BitcoinDeserializer, _rs:&mut ReadStream) -> ::Result<usize> {
      Ok(0usize)
   }
}

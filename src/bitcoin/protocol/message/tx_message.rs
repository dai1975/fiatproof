use std;
use crate::bitcoin::datatypes::Tx;

#[derive(Debug,Default)]
pub struct TxMessage {
   pub tx: Tx,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for TxMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x74, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for TxMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx({})", self.tx)
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for TxMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.tx.serialize(&(), e, ws)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for TxMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.tx.deserialize(&(), d, rs)?;
      Ok(r)
   }
}

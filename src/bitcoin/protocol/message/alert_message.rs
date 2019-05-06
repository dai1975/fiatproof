use std;

#[derive(Debug,Clone,Default)]
pub struct AlertMessage {
   pub msg: Vec<u8>,
   pub sig: Vec<u8>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for AlertMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x61, 0x6c, 0x65, 0x72, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for AlertMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Alert(msg={:?}, sig={})", self.msg, self.sig.len())
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for AlertMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_var_octets(ws, &self.msg[..], std::usize::MAX)?;
      r += e.serialize_var_octets(ws, &self.sig[..], std::usize::MAX)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for AlertMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_var_octets(rs, &mut self.msg, std::usize::MAX)?;
      r += d.deserialize_var_octets(rs, &mut self.sig, std::usize::MAX)?;
      Ok(r)
   }
}

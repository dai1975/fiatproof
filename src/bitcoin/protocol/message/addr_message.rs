use std;
use super::super::{ NetworkAddress };

#[derive(Debug,Default,Clone)]
pub struct AddrMessage {
   pub addrs : Vec<NetworkAddress>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for AddrMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}


use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for AddrMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_ADDR_SIZE;
      r += e.serialize_var_array(&true, ws, &self.addrs, MAX_ADDR_SIZE)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for AddrMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_ADDR_SIZE;
      r += d.deserialize_var_array(&true, rs, &mut self.addrs, MAX_ADDR_SIZE)?;
      Ok(r)
   }
}

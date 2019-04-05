use std;
use ::bitcoin::datatypes::BlockHeader;

#[derive(Debug,Default,Clone)]
pub struct HeadersMessage {
   pub headers: Vec< BlockHeader >,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for HeadersMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for HeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Headers(len={})", self.headers.len())
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for HeadersMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::std::usize::MAX;
      r += e.serialize_var_array(&(), ws, &self.headers[..], MAX)?;
      r += e.serialize_var_int(ws, 0u64)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for HeadersMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::std::usize::MAX;
      r += d.deserialize_var_array(&(), rs, &mut self.headers, MAX)?;
      {
         let mut x:u64 = 0;
         r += d.deserialize_var_int(rs, &mut x)?;
         if x != 0 { raise_serialize_error!(format!("HeadersMessage seems to have block body: len={}", x)) }
      }
      
      Ok(r)
   }
}

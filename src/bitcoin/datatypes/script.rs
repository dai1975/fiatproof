#[derive(Debug,Default,Clone)]
pub struct Script {
   pub bytecode: Vec<u8>,
}

impl Script {
   pub fn new<T:Into<Vec<u8>>>(v:T) -> Self {
      Script { bytecode: v.into() }
   }
   pub fn new_null() -> Self {
      Script { bytecode: vec![] }
   }
   
   pub fn set_null(&mut self) {
      self.bytecode.clear();
   }

   pub fn bytecode(&self) -> &Vec<u8> {
      &self.bytecode
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for Script {
   type P = bool; //add size prefix
   fn serialize(&self, p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      if *p {
         e.serialize_var_octets(ws, &self.bytecode[..], ::std::usize::MAX)
      } else {
         e.serialize_octets(ws, &self.bytecode[..])
      }
   }
}
impl BitcoinDeserializee for Script {
   type P = bool; //add size prefix
   fn deserialize(&mut self, p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      if *p {
         d.deserialize_var_octets(rs, &mut self.bytecode, ::std::usize::MAX)
      } else {
         d.deserialize_octets(rs, &mut self.bytecode)
      }
   }
}

impl ::std::fmt::Display for Script {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match ::ui::BitcoinSerializer::serialize(self, &false).map(|b| ::utils::b2h(b)) {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(_) => f.write_fmt(format_args!("err")),
      }
   }
}



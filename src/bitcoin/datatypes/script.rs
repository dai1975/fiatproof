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

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for Script {
   type P = bool; //add size prefix
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      if *p {
         e.encode_var_octets(ws, &self.bytecode[..], ::std::usize::MAX)
      } else {
         e.encode_octets(ws, &self.bytecode[..])
      }
   }
}
impl BitcoinDecodee for Script {
   type P = bool; //add size prefix
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      if *p {
         d.decode_var_octets(rs, &mut self.bytecode, ::std::usize::MAX)
      } else {
         d.decode_octets(rs, &mut self.bytecode)
      }
   }
}

impl ::std::fmt::Display for Script {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      let b = ::ui::BitcoinSerializer::serialize(self, &false);
      let h = ::utils::b2h(b);
      f.write_fmt(format_args!("{}", h))
   }
}



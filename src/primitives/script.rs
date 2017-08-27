#[derive(Debug,Default,Clone)]
pub struct Script {
   pub bytecode: Vec<u8>,
}

impl Script {
   pub fn new<T:Into<Vec<u8>>>(v:T) -> Script {
      Script { bytecode: v.into() }
   }

   pub fn bytecode(&self) -> &Vec<u8> {
      &self.bytecode
   }
}

use ::serialize::{FromOctets, ToOctets};
impl Script {
   pub fn parse_hex(s:&str) -> ::Result<Self> {
      Self::from_hex_string(s, "unsized")
   }
   pub fn format_hex(&self) -> ::Result<String> {
      self.to_hex_string("unsized")
   }
}

impl ::std::fmt::Display for Script {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self.format_hex() {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(e) => f.write_fmt(format_args!("{:?}", e)),
      }
   }
}


use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for Script {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      e.encode_var_octets(&self.bytecode[..], ::std::usize::MAX)
   }
}
impl BitcoinDecodee for Script {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      d.decode_var_octets(&mut self.bytecode, ::std::usize::MAX)
   }
}


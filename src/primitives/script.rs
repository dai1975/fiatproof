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

impl ::std::fmt::Display for Script {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      use ::serialize::ToOctets;
      match self.to_hex_string("") {
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


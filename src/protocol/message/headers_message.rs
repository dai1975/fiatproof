use std;
use ::BlockHeader;

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

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for HeadersMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::std::usize::MAX;
      r += try!(e.encode_var_array(&self.headers[..], MAX));
      r += try!(e.encode_var_int(0u64));
      Ok(r)
   }
}
impl BitcoinDecodee for HeadersMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::std::usize::MAX;
      r += try!(d.decode_var_array(&mut self.headers, MAX));
      {
         let mut x:u64 = 0;
         r += try!(d.decode_var_int(&mut x));
         if x != 0 { encode_error!(format!("HeadersMessage seems to have block body: len={}", x)) }
      }
      
      Ok(r)
   }
}

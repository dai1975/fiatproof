use std;
use super::super::Inv;

#[derive(Debug,Default,Clone)]
pub struct NotFoundMessage {
   pub invs : Vec<Inv>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for NotFoundMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for NotFoundMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "NotFound(len={}, 0={}", self.invs.len(), self.invs[0])
   }
}


use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for NotFoundMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_var_array(&(), ws, &self.invs[..], ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for NotFoundMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_var_array(&(), rs, &mut self.invs, ::std::usize::MAX));
      Ok(r)
   }
}

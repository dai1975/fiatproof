use std;
use super::super::{ Inv };

#[derive(Debug,Default,Clone)]
pub struct InvMessage {
   pub invs : Vec<Inv>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for InvMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x69, 0x6e, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for InvMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "Inv(len={})", self.invs.len()),
         1 => write!(f, "Inv(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "Inv(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for InvMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_INV_SIZE;
      r += try!(e.encode_var_array(&(), ws, &self.invs[..], MAX_INV_SIZE));
      Ok(r)
   }
}
impl BitcoinDecodee for InvMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_INV_SIZE;
      r += try!(d.decode_var_array(&(), rs, &mut self.invs, MAX_INV_SIZE));
      Ok(r)
   }
}

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

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for AlertMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_var_octets(ws, &self.msg[..], ::std::usize::MAX));
      r += try!(e.encode_var_octets(ws, &self.sig[..], ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for AlertMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_var_octets(rs, &mut self.msg, ::std::usize::MAX));
      r += try!(d.decode_var_octets(rs, &mut self.sig, ::std::usize::MAX));
      Ok(r)
   }
}

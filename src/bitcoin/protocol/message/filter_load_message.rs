use std;

#[derive(Debug,Default,Clone)]
pub struct FilterLoadMessage {
   pub data: Vec<u8>,
   pub hash_funcs: u32,
   pub tweak: u32,
   pub flags: u8,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for FilterLoadMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x00, 0x00];
}

impl std::fmt::Display for FilterLoadMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterLoad(data={:?},funcs={},tweak={},flags={})", self.data, self.hash_funcs, self.tweak, self.flags)
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for FilterLoadMessage {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_octets(ws, &self.data[..]));
      r += try!(e.encode_u32le(ws, self.hash_funcs));
      r += try!(e.encode_u32le(ws, self.tweak));
      r += try!(e.encode_u8(ws, self.flags));
      Ok(r)
   }
}
impl BitcoinDecodee for FilterLoadMessage {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_octets(rs, &mut self.data));
      r += try!(d.decode_u32le(rs, &mut self.hash_funcs));
      r += try!(d.decode_u32le(rs, &mut self.tweak));
      r += try!(d.decode_u8(rs, &mut self.flags));
      Ok(r)
   }
}

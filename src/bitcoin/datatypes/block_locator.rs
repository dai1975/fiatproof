use super::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockLocator {
   pub haves: Vec<UInt256>,
}

impl ::std::fmt::Display for BlockLocator {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "BlockLocator(len={})", self.haves.len())
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for BlockLocator {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      if !e.medium().is_hash() {
         let v:i32 = e.medium().version();
         r += try!(e.encode_i32le(ws, v));
      }
      r += try!(e.encode_var_array(&(), ws, &self.haves, ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for BlockLocator {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      if !d.medium().is_hash() {
         let mut v:i32 = 0;
         r += try!(d.decode_i32le(rs, &mut v));
      }
      r += try!(d.decode_var_array(&(), rs, &mut self.haves, ::std::usize::MAX));
      Ok(r)
   }
}


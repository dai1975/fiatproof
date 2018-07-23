use super::{BlockHeader, Tx};

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub txs: Vec<Tx>,
   pub checked: bool,
}

impl ::std::fmt::Display for Block {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "Block(header={}, tx={})", self.header, self.txs.len())
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for Block {
   type P = ();
   fn encode(&self, p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.encode(&(), e, ws));
      r += try!(e.encode_var_array(&(), ws, &self.txs, ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for Block {
   type P = ();
   fn decode(&mut self, p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.decode(&(), d, rs));
      r += try!(d.decode_var_array(&(), rs, &mut self.txs, ::std::usize::MAX));
      Ok(r)
   }
}

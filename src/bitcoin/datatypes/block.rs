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

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for Block {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.encode(e));
      r += try!(e.encode_var_array(&self.txs, ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for Block {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.decode(d));
      r += try!(d.decode_var_array(&mut self.txs, ::std::usize::MAX));
      Ok(r)
   }
}

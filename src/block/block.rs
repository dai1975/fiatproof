use super::{BlockHeader};
use ::Transaction;

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub transactions: Vec<Transaction>,
   pub checked: bool,
}

impl ::std::fmt::Display for Block {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "Block(header={}, tx={})", self.header, self.transactions.len())
   }
}

use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for Block {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.encode(e, ()));
      r += try!(self.transactions.encode(e, (::std::usize::MAX, ())));
      Ok(r)
   }
}
impl Decodee for Block {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.decode(d, ()));
      r += try!(self.transactions.decode(d, (::std::usize::MAX, ())));
      Ok(r)
   }
}

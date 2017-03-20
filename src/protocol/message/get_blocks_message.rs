use std;
use ::{UInt256, BlockLocator};

#[derive(Debug,Default)]
pub struct GetBlocksMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

impl std::fmt::Display for GetBlocksMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetBlocks(locator={}, stop={})", self.locator, self.hash_stop)
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for GetBlocksMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.encode(e, ()));
      r += try!(self.hash_stop.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for GetBlocksMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.decode(d, ()));
      r += try!(self.hash_stop.decode(d, ()));
      Ok(r)
   }
}

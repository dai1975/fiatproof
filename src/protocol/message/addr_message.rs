use std;
use super::super::{ NetworkAddress };

#[derive(Debug,Default,Clone)]
pub struct AddrMessage {
   pub addrs : Vec<NetworkAddress>,
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for AddrMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_ADDR_SIZE;
      r += try!(self.addrs.encode(e, (MAX_ADDR_SIZE,true)));
      Ok(r)
   }
}
impl Decodee for AddrMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_ADDR_SIZE;
      r += try!(self.addrs.decode(d, (MAX_ADDR_SIZE,true)));
      Ok(r)
   }
}

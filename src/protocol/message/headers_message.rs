use std;
use ::BlockHeader;

#[derive(Debug,Default,Clone)]
pub struct HeadersMessage {
   pub headers: Vec< BlockHeader >,
}

impl std::fmt::Display for HeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Headers(len={})", self.headers.len())
   }
}

use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for HeadersMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.headers.encode(e, (::std::usize::MAX,())));
      r += try!(e.encode_varint(0u64));
      Ok(r)
   }
}
impl Decodee for HeadersMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.headers.decode(d, (::std::usize::MAX,())));
      {
         let mut x:u64 = 0;
         r += try!(d.decode_varint(&mut x));
         if x != 0 { encode_error!(format!("HeadersMessage seems to have block body: len={}", x)) }
      }
      
      Ok(r)
   }
}

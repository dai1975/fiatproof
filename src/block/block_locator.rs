use ::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockLocator {
   pub haves: Vec<UInt256>,
}

impl ::std::fmt::Display for BlockLocator {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "BlockLocator(len={})", self.haves.len())
   }
}

use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for BlockLocator {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      if !e.media().is_hash() {
         let v:i32 = e.media().version();
         r += try!(e.encode_i32le(v));
      }
      r += try!(self.haves.encode(e, (::std::usize::MAX, ())));
      Ok(r)
   }
}
impl Decodee for BlockLocator {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      if !d.media().is_hash() {
         let mut v:i32 = 0;
         r += try!(d.decode_i32le(&mut v));
      }
      r += try!(self.haves.decode(d, (::std::usize::MAX, ())));
      Ok(r)
   }
}


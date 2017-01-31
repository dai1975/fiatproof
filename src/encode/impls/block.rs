use ::std::borrow::Borrow;
use super::super::{EncodeStream, Encodee, DecodeStream, Decodee};

pub use ::structs::block_header::{BlockHeader};
pub use ::structs::block_locator::{BlockLocator};
pub use ::structs::block::{Block};

impl Encodee for BlockHeader {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(self.hash_prev_block.encode(e, ()));
      r += try!(self.hash_merkle_root.encode(e, ()));
      r += try!(e.encode_u32le(self.time));
      r += try!(e.encode_u32le(self.bits));
      r += try!(e.encode_u32le(self.nonce));
      Ok(r)
   }
}
impl Decodee for BlockHeader {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(self.hash_prev_block.decode(d, ()));
      r += try!(self.hash_merkle_root.decode(d, ()));
      r += try!(d.decode_u32le(&mut self.time));
      r += try!(d.decode_u32le(&mut self.bits));
      r += try!(d.decode_u32le(&mut self.nonce));
      Ok(r)
   }
}

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


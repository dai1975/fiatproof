use ::std::borrow::Borrow;
use ::{Error};
use super::super::{Encoder, Encodee, Decoder, Decodee};

pub use ::structs::block_header::{BlockHeader};
pub use ::structs::block_locator::{BlockLocator};
pub use ::structs::block::{Block};

impl <E:Encoder> Encodee<E,()> for BlockHeader {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(e.encode_uint256(&self.hash_prev_block));
      r += try!(e.encode_uint256(&self.hash_merkle_root));
      r += try!(e.encode_u32le(self.time));
      r += try!(e.encode_u32le(self.bits));
      r += try!(e.encode_u32le(self.nonce));
      Ok(r)
   }
}
impl <D:Decoder> Decodee<D,()> for BlockHeader {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(d.decode_uint256(&mut self.hash_prev_block));
      r += try!(d.decode_uint256(&mut self.hash_merkle_root));
      r += try!(d.decode_u32le(&mut self.time));
      r += try!(d.decode_u32le(&mut self.bits));
      r += try!(d.decode_u32le(&mut self.nonce));
      Ok(r)
   }
}

impl <E:Encoder> Encodee<E,()> for BlockLocator {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      if !e.param().is_gethash() {
         let v:i32 = e.param().version();
         r += try!(e.encode_i32le(v));
      }
      r += try!(self.haves.encode((::std::usize::MAX, ()), e));
      Ok(r)
   }
}
impl <D:Decoder> Decodee<D,()> for BlockLocator {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      if !d.param().is_gethash() {
         let mut v:i32 = 0;
         r += try!(d.decode_i32le(&mut v));
      }
      r += try!(self.haves.decode((::std::usize::MAX, ()), d));
      Ok(r)
   }
}

impl <E:Encoder> Encodee<E,()> for Block {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.header.encode((), e));
      r += try!(self.transactions.encode((::std::usize::MAX, ()), e));
      Ok(r)
   }
}
impl <D:Decoder> Decodee<D,()> for Block {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.header.decode((), d));
      r += try!(self.transactions.decode((::std::usize::MAX, ()), d));
      Ok(r)
   }
}


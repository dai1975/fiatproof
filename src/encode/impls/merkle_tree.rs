use ::std::borrow::Borrow;
extern crate bit_vec;
use ::{Error};
use super::super::{Encoder, Encodee, Decoder, Decodee};

pub use ::structs::partial_merkle_tree::{ PartialMerkleTree };
pub use ::structs::merkle_block::{ MerkleBlock };

macro_rules! reverse_u8 {
   ($exp:expr) => {{
      let x:u8 = $exp;
      let x:u8 = ((x & 0x55) << 1) | ((x & 0xAA) >> 1);
      let x:u8 = ((x & 0x33) << 2) | ((x & 0xCC) >> 2);
      let x:u8 = (x << 4) | (x >> 4);
      x
   }}
}

impl <E:Encoder> Encodee<E,()> for PartialMerkleTree {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.n_transactions));
      {
         let mut bytes:Vec<u8> = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8!(*byte);
         }
         r += try!(e.encode_sequence_u8(&bytes[..]));
      }
      r += try!(self.hashes.encode((::std::usize::MAX, ()), e));
      Ok(r)
   }
}
impl <D:Decoder> Decodee<D,()> for PartialMerkleTree {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_u32le(&mut self.n_transactions));
      {
         let mut bytes:Vec<u8> = Vec::new();
         r += try!(d.decode_sequence_u8(&mut bytes));

         for byte in bytes.iter_mut() {
            *byte = reverse_u8!(*byte);
         }
         self.bits = bit_vec::BitVec::from_bytes(bytes.as_slice());
      }
      r += try!(self.hashes.decode((::std::usize::MAX, ()), d));
      Ok(r)
   }
}

impl <E:Encoder> Encodee<E,()> for MerkleBlock {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.header.encode((), e));
      r += try!(self.txn.encode((), e));
      Ok(r)
   }
}
impl <D:Decoder> Decodee<D,()> for MerkleBlock {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.header.decode((), d));
      r += try!(self.txn.decode((), d));
      Ok(r)
   }
}


extern crate bit_vec;
use ::UInt256;

#[derive(Debug,Default,Clone)]
pub struct PartialMerkleTree {
   pub n_transactions: u32,
   pub bits: bit_vec::BitVec,
   pub hashes: Vec<UInt256>,
}

impl ::std::fmt::Display for PartialMerkleTree {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "PartialMerkleTree(n={}, bits={:?}, hash={:?})", self.n_transactions, self.bits, self.hashes)
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};

macro_rules! reverse_u8 {
   ($exp:expr) => {{
      let x:u8 = $exp;
      let x:u8 = ((x & 0x55) << 1) | ((x & 0xAA) >> 1);
      let x:u8 = ((x & 0x33) << 2) | ((x & 0xCC) >> 2);
      let x:u8 = (x << 4) | (x >> 4);
      x
   }}
}

impl Encodee for PartialMerkleTree {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.n_transactions));
      {
         let mut bytes:Vec<u8> = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8!(*byte);
         }
         r += try!(e.encode_sequence_u8(&bytes[..]));
      }
      r += try!(self.hashes.encode(e, (::std::usize::MAX, ())));
      Ok(r)
   }
}
impl Decodee for PartialMerkleTree {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
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
      r += try!(self.hashes.decode(d, (::std::usize::MAX, ())));
      Ok(r)
   }
}

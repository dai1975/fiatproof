extern crate bit_vec;
use super::UInt256;

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


use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};

macro_rules! reverse_u8 {
   ($exp:expr) => {{
      let x:u8 = $exp;
      let x:u8 = ((x & 0x55) << 1) | ((x & 0xAA) >> 1);
      let x:u8 = ((x & 0x33) << 2) | ((x & 0xCC) >> 2);
      let x:u8 = (x << 4) | (x >> 4);
      x
   }}
}

impl BitcoinEncodee for PartialMerkleTree {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(ws, self.n_transactions));
      {
         let mut bytes:Vec<u8> = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8!(*byte);
         }
         r += try!(e.encode_var_octets(ws, &bytes, ::std::usize::MAX));
      }
      r += try!(e.encode_var_array(&(), ws, &self.hashes, ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for PartialMerkleTree {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_u32le(rs, &mut self.n_transactions));
      {
         let mut bytes:Vec<u8> = Vec::new();
         r += try!(d.decode_var_octets(rs, &mut bytes, ::std::usize::MAX));

         for byte in bytes.iter_mut() {
            *byte = reverse_u8!(*byte);
         }
         self.bits = bit_vec::BitVec::from_bytes(bytes.as_slice());
      }
      r += try!(d.decode_var_array(&(), rs, &mut self.hashes, ::std::usize::MAX));
      Ok(r)
   }
}

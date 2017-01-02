use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinSerializer, WriteStream};

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

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for PartialMerkleTree {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.n_transactions));
      {
         let mut bytes:Vec<u8> = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8!(*byte);
         }
         r += try!(e.encode_sequence_u8(&bytes[..]));
      }
      r += try!(e.encode_sequence(&self.hashes));
      Ok(r)
   }
}

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for MerkleBlock {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.header));
      r += try!(e.encode(&self.txn));
      Ok(r)
   }
}


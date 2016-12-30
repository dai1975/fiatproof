use ::{Error};
use super::super::super::{Encoder, WriteStream};
use super::super::{BitcoinEncoder, BitcoinEncodee};

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

impl BitcoinEncodee for PartialMerkleTree {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.n_transactions, w, ep));
      {
         let mut bytes:Vec<u8> = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8!(*byte);
         }
         r += try!(e.encode_sequence_u8(&bytes[..], w, ep));
      }
      r += try!(e.encode_sequence(&self.hashes, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for MerkleBlock {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.header, w, ep));
      r += try!(e.encode(&self.txn, w, ep));
      Ok(r)
   }
}


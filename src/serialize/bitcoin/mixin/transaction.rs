use ::{Error};
use super::super::super::{Encoder, WriteStream};
use super::super::{BitcoinEncoder, BitcoinEncodee};

pub use ::structs::transaction::{ OutPoint, TxIn, TxOut, Transaction };

impl BitcoinEncodee for OutPoint {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_uint256(&self.txid, w, ep));
      r += try!(e.encode_u32le(self.n, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for TxIn {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.prevout, w, ep));
      r += try!(e.encode(&self.script_sig, w, ep));
      r += try!(e.encode_u32le(self.sequence, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for TxOut {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i64le(self.value, w, ep));
      r += try!(e.encode(&self.script_pubkey, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for Transaction {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version, w, ep));
      r += try!(e.encode_sequence(&self.ins, w, ep));
      r += try!(e.encode_sequence(&self.outs, w, ep));
      r += try!(e.encode_u32le(self.locktime, w, ep));
      Ok(r)
   }
}
   
//PartialMerkleTree, MerkleBlock,
//BlockHeader, BlockLocator, Block


use ::{Error};
use super::super::super::{Encoder, WriteStream};
use super::super::{BitcoinEncoder, BitcoinEncodee};

pub use ::structs::block_header::{BlockHeader};
pub use ::structs::block_locator::{BlockLocator};
pub use ::structs::block::{Block};

impl BitcoinEncodee for BlockHeader {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version, w, ep));
      r += try!(e.encode_uint256(&self.hash_prev_block, w, ep));
      r += try!(e.encode_uint256(&self.hash_merkle_root, w, ep));
      r += try!(e.encode_u32le(self.time, w, ep));
      r += try!(e.encode_u32le(self.bits, w, ep));
      r += try!(e.encode_u32le(self.nonce, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for BlockLocator {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      if ep.is_gethash() {
         r += try!(e.encode_i32le(ep.version, w, ep));
      }
      r += try!(e.encode_sequence(&self.haves, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for Block {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.header, w, ep));
      r += try!(e.encode_sequence(&self.transactions, w, ep));
      Ok(r)
   }
}


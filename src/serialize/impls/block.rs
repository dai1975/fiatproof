use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinSerializer, WriteStream};

pub use ::structs::block_header::{BlockHeader};
pub use ::structs::block_locator::{BlockLocator};
pub use ::structs::block::{Block};

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for BlockHeader {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
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

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for BlockLocator {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      if !e.param().is_gethash() {
         let version = e.param().version();
         r += try!(e.encode_i32le(version));
      }
      r += try!(e.encode_sequence(&self.haves));
      Ok(r)
   }
}

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for Block {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.header));
      r += try!(e.encode_sequence(&self.transactions));
      Ok(r)
   }
}


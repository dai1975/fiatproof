use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinSerializer, WriteStream};
pub use ::script::script::{ Script };

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for Script {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.bytecode[..]));
      Ok(r)
   }
}


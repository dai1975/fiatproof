use ::std::borrow::Borrow;
use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinDecoder, BitcoinDecodee};
pub use ::script::script::{ Script };

impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for Script {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.bytecode[..]));
      Ok(r)
   }
}

impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for Script {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.bytecode));
      Ok(r)
   }
}


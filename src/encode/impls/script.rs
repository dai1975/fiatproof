use ::std::borrow::Borrow;
use ::{Error};
use super::super::{Encoder, Encodee, Decoder, Decodee};
pub use ::script::{ Script, Statement, ScriptNum, opcode };

impl <E:Encoder> Encodee<E,()> for Script {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(self.bytecode().as_slice()));
      Ok(r)
   }
}

impl <D:Decoder> Decodee<D,()> for Script {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      let mut v:Vec<u8> = Vec::new();
      r += try!(d.decode_sequence_u8(&mut v));
      *self = try!(Script::compile(v));
      Ok(r)
   }
}


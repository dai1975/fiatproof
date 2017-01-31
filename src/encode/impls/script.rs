use ::std::borrow::Borrow;
use super::super::{EncodeStream, Encodee, DecodeStream, Decodee};
pub use ::script::{ Script, Statement, ScriptNum, opcode };

impl Encodee for Script {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(self.bytecode().as_slice()));
      Ok(r)
   }
}

impl Decodee for Script {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut v:Vec<u8> = Vec::new();
      r += try!(d.decode_sequence_u8(&mut v));
      *self = try!(Script::compile(v));
      Ok(r)
   }
}


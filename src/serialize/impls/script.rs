use ::std::borrow::Borrow;
use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinDecoder, BitcoinDecodee};
pub use ::script::{ Script, Statement, ScriptNum, opcode };

/*
fn encode_value<E:BitcoinEncoder>(val:i64, e:&mut E) -> Result<usize, Error> {
   let mut r:usize = 0;
   match val {
      0      => r += try!(e.encode_u8(opcode::OP_0)),
      -1     => r += try!(e.encode_u8(opcode::OP_1NEGATE)),
      1...16 => r += try!(e.encode_u8((opcode::OP_1 + (val as u8) - 1) as u8)),
      _ => {
         let mut tmp = [0u8;9];
         let len = ScriptNum(val).encode(&mut tmp);
         r += try!(e.encode_u8(opcode::OP_0x01 + (len as u8) - 1));
         r += try!(e.encode_array_u8(&tmp[..len]));
      }
   }
   Ok(r)
}
fn encode_bytes<E:BitcoinEncoder>(val:&[u8], e:&mut E) -> Result<usize, Error> {
   let mut r:usize = 0;
   let len = val.len();
   if len == 0 {
      r += try!(e.encode_u8(opcode::OP_0));
   } else if len == 1 {
      match val[0] {
         0x00 | 0x80 => r += try!(e.encode_u8(opcode::OP_0)),
         0x81        => r += try!(e.encode_u8(opcode::OP_1NEGATE)),
         x @ 1...16  => r += try!(e.encode_u8(opcode::OP_1 + (x as u8) - 1)),
         x @ _ => {
            let tmp:[u8;2] = [ opcode::OP_0x01, x as u8 ];
            r += try!(e.encode_array_u8(&tmp));
         }
      }
   } else if len <= 75 {
      r += try!(e.encode_u8(opcode::OP_0x01 + (len as u8) - 1));
      r += try!(e.encode_array_u8(val));
   } else if len <= 0xFF {
      r += try!(e.encode_u8(opcode::OP_PUSHDATA1));
      r += try!(e.encode_u8(len as u8));
      r += try!(e.encode_array_u8(val));
   } else if len <= 0xFFFF {
      r += try!(e.encode_u8(opcode::OP_PUSHDATA2));
      r += try!(e.encode_u16le(len as u16));
      r += try!(e.encode_array_u8(val));
   } else if len <= 0xFFFFFFFF {
      r += try!(e.encode_u8(opcode::OP_PUSHDATA4));
      r += try!(e.encode_u32le(len as u32));
      r += try!(e.encode_array_u8(val));
   } else {
      serialize_error!(format!("data is too long: {}", val.len()));
   }
   Ok(r)
}

impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for Statement {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      match self {
         &Statement::Value(v)     => encode_value(v, e),
         &Statement::Bytes(ref v) => encode_bytes(v.as_ref(), e),
         &Statement::Op(v)        => e.encode_u8(v),
      }
   }
}
*/
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for Script {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(self.bytecode().as_slice()));
      Ok(r)
   }
}

impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for Script {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      let mut v:Vec<u8> = Vec::new();
      r += try!(d.decode_sequence_u8(&mut v));
      *self = try!(Script::compile(v));
      Ok(r)
   }
}


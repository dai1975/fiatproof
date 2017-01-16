use ::std::borrow::Borrow;
use super::opcode::*;
use super::{Statement, ScriptNum};
use ::encode::{Encoder, Encodee, EncodeStream};

pub struct Compiler<'a>(pub &'a Vec<Statement>);

impl <'a> Compiler<'a> {
   pub fn compile(&mut self) -> ::Result<Vec<u8>> {
      use ::std::io::Cursor;
      let mut ser = EncodeStream::new_with(Cursor::new(Vec::<u8>::with_capacity(1024)));
      for s in self.0 {
         try!(s.encode(&(), &mut ser));
      }
      Ok(ser.into_inner().into_inner())
   }
}

impl <'a,E:Encoder> Encodee<E,()> for Statement {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> ::Result<usize> {
      let mut r:usize = 0;
      match self {
         &Statement::Op(op) => {
            r += try!(e.encode_u8(op));
         },
         &Statement::Value(val) => {
            if val == 0 {
               r += try!(e.encode_u8(OP_0));
            } else if val == -1 {
               r += try!(e.encode_u8(OP_1NEGATE));
            } else if val <= 16  {
               r += try!(e.encode_u8(OP_1 + ((val-1) as u8)));
            } else {
               use ::encode::FixedEncodeStream;
               let mut tmp = FixedEncodeStream::new(9);
               let size = try!(ScriptNum(val).encode(&(), &mut tmp));
               if size <= 0x4b {
                  r += try!(e.encode_u8( OP_0x01 + ((size-1) as u8) ));
               } else {
                  r += try!(e.encode_u8(OP_PUSHDATA1));
                  r += try!(e.encode_u8(size as u8));
               }
               r += try!(e.encode_array_u8(&tmp.as_slice()[..size]));
            }
         },
         &Statement::Bytes(ref bytes) => {
            let len = bytes.len();
            if len == 0 {
               r += try!(e.encode_u8(OP_0));
            } else if len == 1 {
               match bytes[0] {
                  0x00 | 0x80     => r += try!(e.encode_u8(OP_0)),
                  0x81            => r += try!(e.encode_u8(OP_1NEGATE)),
                  v @ 0x01...0x16 => r += try!(e.encode_u8(OP_1 + (v-1) as u8)),
                  v @ _ => {
                     r += try!(e.encode_u8(OP_0x01));
                     r += try!(e.encode_u8(v));
                  }
               }
            }  else if len <= 0x4b {
               r += try!(e.encode_u8( OP_0x01 + ((len-1) as u8) ));
               r += try!(e.encode_array_u8(&bytes));
            } else if len <= ::std::u8::MAX as usize {
               r += try!(e.encode_u8(OP_PUSHDATA1));
               r += try!(e.encode_u8(len as u8));
               r += try!(e.encode_array_u8(&bytes));
            } else if len <= ::std::u16::MAX as usize {
               r += try!(e.encode_u8(OP_PUSHDATA2));
               r += try!(e.encode_u16le(len as u16));
               r += try!(e.encode_array_u8(&bytes));
            } else if len <= ::std::u32::MAX as usize {
               r += try!(e.encode_u8(OP_PUSHDATA4));
               r += try!(e.encode_u32le(len as u32));
               r += try!(e.encode_array_u8(&bytes));
            } else {
               encode_error!(format!("data is too long: len={}", len));
            }
         },
      }
      Ok(r)
   }
}

#[test]
fn test_compile() {
   {
      let s = vec![Statement::Value(0)];
      let r = Compiler(&s).compile().unwrap();
      assert_eq!(r.as_slice(), &[OP_0])
   }
}

#[derive(Debug,Clone)]
pub enum Statement {
   Value(i64),
   Bytes(Box<[u8]>),
   Op(u8),
}

use super::{ScriptNum};
use super::opcode::*;
use ::std::borrow::Borrow;
use ::codec::{Encodee, EncodeStream, BitcoinEncodeStream, SliceWriteStream, Media};
impl Encodee for Statement {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
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
               let mut tmp = BitcoinEncodeStream::new(SliceWriteStream::new([0u8; 9]), Media::default().set_net());
               let size = try!(ScriptNum(val).encode(&mut tmp, ()));
               r += try!(e.encode_u8( OP_0x01 + ((size-1) as u8) ));
               r += try!(e.encode_array_u8(&tmp.w.get_ref()[..size]));
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
fn test_encode() {
   use ::codec::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
   let v = Statement::Value(0);
   assert_matches!(v.encode(&mut e, ()), Ok(1));
   assert_eq!(e.w.get_ref().as_slice(), &[OP_0]);
}

use ::std;
use super::{opcode, Statement};

#[derive(Debug,Default,Clone)]
pub struct Script {
   bytecode:   Vec<u8>,
   statements: Vec<Statement>,
}
impl Script {
   pub fn new() -> Self {
      Script { bytecode: Vec::new(), statements: Vec::new() }
   }
   pub fn compile(v: Vec<u8>) -> ::Result<Self> {
      super::Parser(&v).parse().map(
         |statements| Script { bytecode: v, statements: statements }
      )
   }
   pub fn bytecode(&self) -> &Vec<u8> {
      &self.bytecode
   }
   pub fn statements(&self) -> &Vec<Statement> {
      &self.statements
   }
}
impl std::fmt::Display for Script {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      for s in self.statements.iter() {
         let _ = match s {
            &Statement::Value(x)     => write!(f, "[0x{:X}] ", x),
            &Statement::Bytes(ref x) => write!(f, "[({})] ", x.len()),
            &Statement::Op(x)        => write!(f, "{} ", opcode::OPCODE_INFO[x as usize].name),
         };
      }
      write!(f, "")
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for Script {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      if e.media().is_dump() {
         r += try!(e.encode_array_u8(self.bytecode().as_slice()));
      } else {
         r += try!(e.encode_sequence_u8(self.bytecode().as_slice()));
      }
      Ok(r)
   }
}

impl Decodee for Script {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut v:Vec<u8> = Vec::new();
      if d.media().is_dump() {
         r += try!(d.decode_to_end(&mut v));
      } else {
         r += try!(d.decode_sequence_u8(&mut v));
      }
      *self = try!(Script::compile(v));
      Ok(r)
   }
}

impl_dump! { Script, () }


#[test]
fn test_decode() {
   use ::codec::WithBytes;
   let hex = "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab";
   let s = Script::with_hex(hex);
   assert_matches!(s, Ok(_));
   let s = s.unwrap();
   assert_eq!(s.statements.len(), 2);
   assert_matches!(s.statements[0], Statement::Bytes(_));
   if let Statement::Bytes(ref bytes) = s.statements[0] {
      assert_eq!(bytes.len(), 0x48);
      assert_eq!(&bytes[..3], &[0x30, 0x45, 0x02]);
   }
   assert_matches!(s.statements[1], Statement::Bytes(_));
   if let Statement::Bytes(ref bytes) = s.statements[1] {
      assert_eq!(bytes.len(), 0x41);
      assert_eq!(&bytes[..3], &[0x04, 0xc5, 0x4f]);
   }
   /*
   48
   3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01
   41
   04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab
    */
}

use super::{Statement};

pub struct Compiler<'a>(pub &'a Vec<Statement>);

impl <'a> Compiler<'a> {
   pub fn compile(&mut self) -> ::Result<Vec<u8>> {
      use ::codec::{Encodee, BitcoinEncodeStream, VecWriteStream, Media};
      let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
      for s in self.0 {
         try!(s.encode(&mut e, ()));
      }
      Ok(e.w.into_inner())
   }
}

#[test]
fn test_compile() {
   use super::opcode::*;
   {
      let s = vec![Statement::Value(0)];
      let r = Compiler(&s).compile().unwrap();
      assert_eq!(r.as_slice(), &[OP_0])
   }
}

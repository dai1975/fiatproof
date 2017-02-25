use super::{Parser};

#[derive(Debug,Clone)]
pub struct Script {
   bytecode:    Box<[u8]>,
}
impl Default for Script {
   fn default() -> Self {
      Script {
         bytecode: Box::default(),
      }
   }
}

impl Script {
   pub fn new<T:Into<Vec<u8>>>(bytecode:T) -> Script {
      let bytecode = bytecode.into().into_boxed_slice();
      Script { bytecode:bytecode }
   }

   pub fn bytecode(&self) -> &[u8] {
      &self.bytecode[..]
   }
   pub fn parse(&self) -> Parser {
      Parser::new(&self.bytecode[..])
   }
   /*
   pub fn parsed(&self) -> ::Result<&[Parsed]> {
      match self.parsed {
         Err(ref e) => Err(e.clone()),
         Ok(ref b)  => Ok(b.as_ref()),
      }
   }
   pub fn to_asm(&self) -> ::Result<Vec<Instruction>> {
      match self.parsed {
         Err(ref err)   => Err(err.clone()),
         Ok(ref parsed) => {
            let instructions = parsed.iter().map(|parsed| {
               super::instruction::make(self.bytecode.as_ref(), parsed)
            }).collect();
            Ok(instructions)
         }
      }
   }
*/
}
/*
impl <'a, T:Into<Vec<Instruction<'a>>>> ::std::convert::TryFrom<T> for Script {
   type Err = ::Error;
   fn try_from(asm:T) -> ::Result<Self> {
      use ::codec::{BitcoinEncodeStream, Encodee, VecWriteStream, Media};
      let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
      for inst in asm.into().iter() {
         try!(inst.encode(&mut e, ()));
      }
      Ok(Script::new(e.w.into_inner()))
   }
}
*/
impl ::std::fmt::Display for Script {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      let mut v:Vec<String> = Vec::new();
      for parsed in self.parse() {
         match parsed {
            Err(ref err) => {
               try!(write!(f, "{:?}", err));
               return Ok(()); //break in case of parse error
               
            },
            Ok(ref p) => {
               v.push(format!("{}", p));
            },
         }
      }
      v.join(" ").fmt(f)
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for Script {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      if e.media().is_dump() {
         r += try!(e.encode_array_u8(&self.bytecode[..]));
      } else {
         r += try!(e.encode_sequence_u8(&self.bytecode[..]));
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
      *self = Script::new(v);
      Ok(r)
   }
}

impl_dump! { Script, () }



#[test]
fn test_decode() {
   use ::codec::WithBytes;
   let hex = "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab";
   /*
   48
   3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01
   41
   04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab
    */

   let script = Script::with_hex(hex);
   assert_matches!(script, Ok(_));
   let script = script.unwrap();
   assert_eq!(format!("{}", script), "[72] [65]");
/*
   let asm = script.to_asm();
   assert_matches!(asm, Ok(_));
   let asm = asm.unwrap();

   assert_eq!(asm.len(), 2);
   use super::instruction::Instruction::*;
   assert_matches!(&asm[..], &[FIX(0x48,_), FIX(0x41,_)]);
   assert_matches!(&asm[..], &[FIX(_, a), _] if a.len() == 0x48);
   assert_matches!(&asm[..], &[FIX(_, &[0x30, 0x45, ..]), FIX(_, &[0x04, .., 0xab])]);
*/
}

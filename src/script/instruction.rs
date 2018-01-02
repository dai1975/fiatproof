use super::pushee::Pushee;
use super::opcode::*;

pub enum Instruction<'a> {
   Push(Pushee<'a>),
   Op(u8),
}

impl <'a> Instruction<'a> {
   pub fn data(&self) -> Option<&[u8]> {
      match self {
         &Instruction::Push(ref pushee) => pushee.raw_data(),
         _ => None,
      }
   }
   pub fn value(&self) -> Option<i64> {
      match self {
         &Instruction::Push(ref pushee) => pushee.raw_value(),
         _ => None,
      }
   }
   pub fn opcode(&self) -> Option<u8> {
      match self {
         &Instruction::Op(code) => Some(code),
         _ => None,
      }
   }
}

impl <'a> ::std::fmt::Display for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Push(ref x) => x.fmt(f),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
         _ => write!(f, "unimplemented"),
      }
   }
}
impl <'a> ::std::fmt::Debug for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Push(ref x) => x.fmt(f),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
         _ => write!(f, "unimplemented"),
      }
   }
}

#[test]
fn test_infoarray() {
   //assert_eq!(256, OPCODE_INFO.len());
}



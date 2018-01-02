use super::opcode::*;
use super::num::ScriptNum;
use std::borrow::{Cow};

pub enum Instruction<'a> {
   Data(Cow<'a, [u8]>),
   Value(i64),
   Op(u8),
}

impl <'a> Instruction<'a> {
   pub fn new_data(data: &'a[u8]) -> Self {
      Instruction::Data(Cow::from(data))
   }
   pub fn new_data_copy(data: &[u8]) -> Self {
      let v:Vec<u8> = data.into_iter().cloned().collect();
      Instruction::Data(Cow::from(v))
   }
   pub fn new_value<T:Into<i64>>(v: T) -> Self {
      let v:i64 = v.into();
      Instruction::Value(v)
   }

   pub fn data(&self) -> Option<&[u8]> {
      match self {
         &Instruction::Data(Cow::Borrowed(x)) => Some(x),
         &Instruction::Data(Cow::Owned(ref v)) => Some(v.as_slice()),
         _ => None,
      }
   }
   pub fn value(&self) -> Option<i64> {
      match self {
         &Instruction::Value(v) => Some(v),
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
         &Instruction::Data(ref data) => write!(f, "[{}]", data.len()),
         &Instruction::Value(ref v) => write!(f, "{}(0x{:x})", v, v),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
         _ => write!(f, "unimplemented"),
      }
   }
}
impl <'a> ::std::fmt::Debug for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Data(ref data) => write!(f, "Data[{}]", data.len()),
         &Instruction::Value(ref v) => write!(f, "Value({})", v),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
         _ => write!(f, "unimplemented"),
      }
   }
}


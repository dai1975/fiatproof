use super::opcode::*;
use std::borrow::{Cow};

pub enum Instruction<'a> {
   Data(u8, Cow<'a, [u8]>),
   Value(u8, i64),
   Op(u8),
}

impl <'a> Instruction<'a> {
   pub fn new_data(op:u8, data: &'a[u8]) -> Self {
      Instruction::Data(op, Cow::from(data))
   }
   pub fn new_data_copy(op:u8, data: &[u8]) -> Self {
      let v:Vec<u8> = data.into_iter().cloned().collect();
      Instruction::Data(op, Cow::from(v))
   }
   pub fn new_value<T:Into<i64>>(op:u8, v: T) -> Self {
      let v:i64 = v.into();
      Instruction::Value(op, v)
   }

   #[inline] pub fn data(&self) -> Option<&[u8]> {
      self.data_and_opcode().map(|(d,_)| d)
   }
   #[inline] fn data_and_opcode(&self) -> Option<(&[u8], u8)> {
      match self {
         &Instruction::Data(op, Cow::Borrowed(x)) =>  Some((x,op)),
         &Instruction::Data(op, Cow::Owned(ref v)) => Some((v.as_slice(),op)),
         _ => None,
      }
   }
   #[inline] pub fn value(&self) -> Option<i64> {
      match self {
         &Instruction::Value(_, v) => Some(v),
         _ => None,
      }
   }
   #[inline] pub fn opcode(&self) -> Option<u8> {
      match self {
         &Instruction::Op(code) => Some(code),
         _ => None,
      }
   }

   pub fn check_minimal_push(&self) -> bool {
      if let Some((d,op)) = self.data_and_opcode() {
         match d.len() {
            0 => { op == OP_0 },
            1 => {
               if 1 <= d[0] && d[0] <= 16 {
                  op == OP_1 + d[0] - 1
               } else if d[0] == 0x81 {
                  op == OP_1NEGATE
               } else {
                  true
               }
            },
            l if l <= 75 =>    { op == (l as u8) },
            l if l <= 255 =>   { op == OP_PUSHDATA1 },
            l if l <= 65535 => { op == OP_PUSHDATA2 },
            _ => true,
         }
      } else {
         true
      }
   }
}

impl <'a> ::std::fmt::Display for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Data(_, ref data) => write!(f, "[{}]", data.len()),
         &Instruction::Value(_, ref v) => write!(f, "{}(0x{:x})", v, v),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
      }
   }
}
impl <'a> ::std::fmt::Debug for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Data(_, ref data) => write!(f, "Data[{}]", data.len()),
         &Instruction::Value(_, ref v) => write!(f, "Value({})", v),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
      }
   }
}


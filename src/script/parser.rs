use super::opcode::*;
use super::Instruction;

/*
#[derive(Debug,Clone)]
pub struct Parsed<'a> {
   bytecode: &'a [u8],
   opcode_offset: usize,
   opcode_info: &'static OpCodeInfo,
   data_offset: usize,
   data_len: usize,
}
impl <'a> Parsed<'a> {
   pub fn opcode(&self) -> u8 {
      self.bytecode[self.opcode_offset]
   }
   pub fn opinfo(&self) -> &'static OpCodeInfo {
      self.opcode_info
   }
   pub fn offset(&self) -> usize {
      self.opcode_offset
   }
   pub fn datalen(&self) -> usize {
      self.data_len
   }
   pub fn data(&self) -> &[u8] {
      &self.bytecode[self.data_offset .. (self.data_offset + self.data_len)]
   }
}
impl <'a> ::std::fmt::Display for Parsed<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self.opcode_info.num_operands {
         0 => write!(f, "{}",   self.opcode_info.name),
         1 => write!(f, "[{}]", self.data_len),
         2 => write!(f, "[{}({})]", self.data_len, self.data_offset - self.opcode_offset),
         _ => write!(f, "<unexpected opcode={}>", self.opcode_info.code),
      }
   }
}
 */

pub struct Parser<'a> {
   bytecode: &'a [u8],
}

impl <'a> Parser<'a> {
   pub fn new<'x>(bytecode: &'x [u8]) -> Parser<'x> {
      Parser { bytecode:bytecode }
   }
   pub fn iter(&self) -> Parsed<'a> {
      Parsed { bytecode: self.bytecode, cursor: 0 }
   }
}
impl <'a> ::std::iter::IntoIterator for Parser<'a> {
   type IntoIter = Parsed<'a>;
   type Item = ::Result<Instruction<'a>>;
   fn into_iter(self) -> Self::IntoIter {
      Parsed { bytecode: self.bytecode, cursor: 0 }
   }
}

pub struct Parsed<'a> {
   bytecode: &'a [u8],
   cursor:   usize,
}

impl <'a> Parsed<'a> {
   fn parse_pushdata(&self) -> ::Result<(usize,usize)> {
      let code = self.bytecode[self.cursor];
      match code {
         OP_PUSHDATA1 => {
            if self.bytecode.len() <= self.cursor+1 {
               parse_script_error!(format!("cannot get length of PUSHDATA1 at {}", self.cursor));
            }
            let v = self.bytecode[self.cursor + 1];
            Ok((1, v as usize))
         },
         OP_PUSHDATA2 => {
            if self.bytecode.len() <= self.cursor+2 {
               parse_script_error!(format!("cannot get length of PUSHDATA2 at {}", self.cursor));
            }
            let v:u16 = (self.bytecode[self.cursor + 1] as u16)
               | (self.bytecode[self.cursor + 2] as u16) << 8;
            Ok((2, v as usize))
         },
         OP_PUSHDATA4 => {
            if self.bytecode.len() <= self.cursor+4 {
               parse_script_error!(format!("cannot get length of PUSHDATA4 at {}", self.cursor));
            }
            let v:u32 = (self.bytecode[self.cursor+1] as u32)
               | (self.bytecode[self.cursor+2] as u32) << 8
               | (self.bytecode[self.cursor+3] as u32) << 16
               | (self.bytecode[self.cursor+4] as u32) << 24;
            Ok((4, v as usize))
         },
         _ => {
            Ok((0, code as usize))
         }
      }
   }

   fn next0(&self) -> ::Result<(usize,Instruction)> { //0 なら終端。Instruction はダミー。
      if self.bytecode.len() <= self.cursor {
         return Ok((0,Instruction::Nop));
      }
      let code = self.bytecode[self.cursor];
      let info = OPCODE_INFO[code as usize];
      //println!("    next. code[{}]={:x}={}...", cursor0, code, OPCODE_INFO[code as usize].name);
      match code {
         OP_PUSHDATAFIX_01 ... OP_PUSHDATA4 => {
            let (offset, datalen) = try!(self.parse_pushdata());
            let from = self.cursor + 1 + offset;
            let to   = from + datalen;
            if 0 < datalen && self.bytecode.len() < to {
               parse_script_error!(format!("cannot get data[{}] of {} at {}+{}", datalen, info.name, self.cursor, 1+offset));
            }
            Ok((1+offset+datalen, Instruction::PushData { data: &self.bytecode[from..to] }))
         },
         OP_0 => {
            Ok((1, Instruction::PushValue { value:0 }))
         },
         OP_1 ... OP_16 => {
            Ok((1, Instruction::PushValue { value: (code-OP_1+1) as u64 }))
         },
         _ => {
            Ok((1, Instruction::Nop))
         }
      }
   }
}

impl <'a> ::std::iter::Iterator for Parsed<'a> {
   type Item = ::Result<Instruction<'a>>;
   fn next(&mut self) -> Option<Self::Item> {
      let r = self.next0();
      match r {
         Err(e)     => Some(Err(e)),
         Ok((0, _)) => None,
         Ok((delta, inst)) => {
            self.cursor += delta;
            Some(Ok(inst))
         },
      }
   }
}


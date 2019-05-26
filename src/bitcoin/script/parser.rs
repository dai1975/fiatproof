use super::super::datatypes;
use super::opcode::*;
use super::Instruction;

pub struct Parser;

#[derive(Debug)]
pub struct Parsed<'a> {
   pub offset:      usize,
   pub size:        usize,
   pub opcode:      u8,
   pub instruction: Instruction<'a>,
}
pub struct Iter<'a> {
   pub bytecode: &'a [u8],
   pub cursor:   usize,
}

impl Parser {
   pub fn iter<'x>(bytecode: &'x [u8]) -> Iter<'x> {
      Iter { bytecode: bytecode, cursor: 0 }
   }
   pub fn parse<'x>(bytecode: &'x [u8]) -> crate::Result<Vec<Parsed<'x>>> {
      let mut v = Vec::new();
      for r in Parser::iter(bytecode) {
         if let Err(e) = r {
            return Err(e);
         }
         v.push(r.unwrap());
      }
      Ok(v)
   }
   pub fn parse_raw<'x>(bytecode: &'x [u8]) -> crate::Result<Vec<Instruction<'x>>> {
      let mut v = Vec::new();
      for r in Parser::iter(bytecode) {
         if let Err(e) = r {
            return Err(e);
         }
         v.push(r.unwrap().instruction);
      }
      Ok(v)
   }
   pub fn find_and_delete(bytecode: &[u8], target: &[u8]) -> (Vec<u8>, usize) {
      if target.len() == 0 {
         return (bytecode.iter().cloned().collect(), 0);
      }
      let mut num_found:usize = 0usize;
      let mut ret_vec:Vec<u8> = Vec::with_capacity(bytecode.len());
      let mut pc0 = 0usize;
      let mut pc = 0usize;
      loop {
         ret_vec.extend_from_slice(&bytecode[pc0..pc]);
         loop {
            let tmp = &bytecode[pc..];
            if tmp.len() < target.len() || &tmp[0..target.len()] != target {
               break;
            }
            num_found += 1;
            pc += target.len();
         }
         pc0 = pc;
         let r = match Parser::iter(&bytecode[pc..]).next() {
            None => None,
            Some(Err(_)) => None,
            Some(Ok(parsed)) => Some(parsed.size),
         };
         if r.is_none() { break; }
         pc += r.unwrap();
      }
      ret_vec.extend_from_slice(&bytecode[pc0..]);
      (ret_vec, num_found)
   }
   pub fn is_push_only(bytecode: &[u8]) -> bool {
      Parser::iter(bytecode).all(|r| {
         r.is_ok() && r.unwrap().opcode < OP_16
      })
   }
   pub fn parse_pay_to_script_hash(bytecode: &[u8]) -> Option<&[u8]> {
      datatypes::script::parse_pay_to_script_hash(bytecode)
   }
   pub fn parse_witness_script(bytecode: &[u8], enable_p2sh:bool) -> Option<(u8,&[u8])> {
      datatypes::script::parse_witness_script(bytecode, enable_p2sh)
   }
}

impl <'a> Iter<'a> {
   fn parse_pushdata(&self) -> crate::Result<(usize,usize)> {
      let code = self.bytecode[self.cursor];
      let info = OPCODE_INFO[code as usize];
      let (offset, datalen) = match code {
         OP_PUSHDATA1 => {
            if self.bytecode.len() <= self.cursor+1 {
               raise_parse_script_error!(format!("cannot get length of PUSHDATA1 at {}", self.cursor));
            }
            let v = self.bytecode[self.cursor + 1];
            (1, v as usize)
         },
         OP_PUSHDATA2 => {
            if self.bytecode.len() <= self.cursor+2 {
               raise_parse_script_error!(format!("cannot get length of PUSHDATA2 at {}", self.cursor));
            }
            let v:u16 = (self.bytecode[self.cursor + 1] as u16)
               | (self.bytecode[self.cursor + 2] as u16) << 8;
            (2, v as usize)
         },
         OP_PUSHDATA4 => {
            if self.bytecode.len() <= self.cursor+4 {
               raise_parse_script_error!(format!("cannot get length of PUSHDATA4 at {}", self.cursor));
            }
            let v:u32 = (self.bytecode[self.cursor+1] as u32)
               | (self.bytecode[self.cursor+2] as u32) << 8
               | (self.bytecode[self.cursor+3] as u32) << 16
               | (self.bytecode[self.cursor+4] as u32) << 24;
            (4, v as usize)
         },
         _ => {
            (0, code as usize)
         }
      };
      let from = self.cursor + 1 + offset;
      let to   = from + datalen;
      if 0 < datalen && self.bytecode.len() < to {
         raise_parse_script_error!(format!("cannot get data[{}] of {} at {}+{}", datalen, info.name, self.cursor, 1+offset));
      }
      Ok((from, to))
   }
}

impl <'a> std::iter::Iterator for Iter<'a> {
   type Item = crate::Result<Parsed<'a>>;

   fn next(&mut self) -> Option<crate::Result<Parsed<'a>>> {
      if self.bytecode.len() <= self.cursor {
         return None
      }
      let cursor0 = self.cursor;
      let code = self.bytecode[self.cursor];
      //let info = OPCODE_INFO[code as usize];
      //println!("    next. code[{}]={:x}={}...", cursor0, code, OPCODE_INFO[code as usize].name);
      let inst = match code {
         OP_PUSHDATAFIX_01 ... OP_PUSHDATA4 => {
            match self.parse_pushdata() {
               Err(e) => return Some(Err(e)),
               Ok((from, to)) => {
                  self.cursor = to;
                  Instruction::new_data(code, &self.bytecode[from..to])
               },
            }
         },
         OP_0 => {
            self.cursor += 1;
            Instruction::new_value(code, 0)
         },
         OP_1 ... OP_16 => {
            self.cursor += 1;
            Instruction::new_value(code, code-OP_1+1)
         },
         _ => {
            self.cursor += 1;
            Instruction::Op(code)
         }
      };
      let size = self.cursor - cursor0;
      Some(Ok(Parsed{offset:cursor0, size:size, opcode:code, instruction:inst}))
   }
}

/*
  next を分割したいのだが。
  たとえば
    fn next0(&self) -> Instruction<'a>
  というサブ関数に分離し、next(&mut self) から
    let r = self.next0()
  と呼ぶ形になる。
  しかし next 中の self は trait で指定された通り(&mut self)なのでライフタイムを指定できない。
  next0 の返すライフタイム('a) と不整合が起きる。
*/


#[test]
fn test_deserialize() {
   use crate::utils::h2b;
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                       "41", "04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab")).unwrap();
   // 0x48=72, 0x41=65, 0x48+0x41=137
   
   use super::{Instruction as I};
   use super::parser::Parser;
   let mut parsed = Parser::iter(bytecode.as_ref());

   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(_)));
      let parsed = n.unwrap().unwrap();
      assert_eq!(parsed.offset, 0);
      assert_eq!(parsed.opcode, OP_PUSHDATAFIX_48);
      assert_matches!(parsed.instruction, I::Data(_,_));
      let data = parsed.instruction.data().unwrap();
      assert_eq!(data.len(), 0x48);
      assert_eq!(data, &bytecode[1..(1+0x48)]);
      assert_eq!(data[0..4], [0x30, 0x45, 0x02, 0x21]);
   }
   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(_)));
      let parsed = n.unwrap().unwrap();
      assert_eq!(parsed.offset, 0x49);
      assert_eq!(parsed.opcode, OP_PUSHDATAFIX_41);
      assert_matches!(parsed.instruction, I::Data(_,_));
      let data = parsed.instruction.data().unwrap();
      assert_eq!(data.len(), 0x41);
      assert_eq!(data, &bytecode[0x4a..(0x4a+0x41)]);
      assert_eq!(data[0..4], [0x04, 0xc5, 0x4f, 0x8e]);
   }
}

#[test]
fn test_deserialize_failed() {
   use crate::utils::h2b;
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                       "4c", "FF", "")).unwrap();
   // 0x48=72, 0x41=65, 0x48+0x41=137
   
   use super::parser::Parser;
   use super::{Instruction as I};
   let mut parsed = Parser::iter(bytecode.as_ref());

   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(_)));
      let parsed = n.unwrap().unwrap();
      assert_eq!(parsed.offset, 0);
      assert_eq!(parsed.opcode, OP_PUSHDATAFIX_48);
      assert_matches!(parsed.instruction, I::Data(_,_));
      let data = parsed.instruction.data().unwrap();
      assert_eq!(data.len(), 0x48);
      assert_eq!(data, &bytecode[1..(1+0x48)]);
      assert_eq!(data[0..4], [0x30, 0x45, 0x02, 0x21]);
   }
   {
      let n = parsed.next();
      assert_matches!(n, Some(Err(_)));
   }
}

#[test]
fn test_parse() {
   use crate::utils::{h2b, FmtVec};
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                              "41", "04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab")).unwrap();

   let instructions = Parser::parse_raw(bytecode.as_ref()).unwrap();
   assert_eq!("[72] [65]", format!("{}", FmtVec(instructions)));
}
   

use super::opcode::*;
use super::Instruction;

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
   pub fn parse<'x>(bytecode: &'x [u8]) -> Parsed<'x> {
      Parsed { bytecode: bytecode, cursor: 0 }
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
      let info = OPCODE_INFO[code as usize];
      let (offset, datalen) = match code {
         OP_PUSHDATA1 => {
            if self.bytecode.len() <= self.cursor+1 {
               parse_script_error!(format!("cannot get length of PUSHDATA1 at {}", self.cursor));
            }
            let v = self.bytecode[self.cursor + 1];
            (1, v as usize)
         },
         OP_PUSHDATA2 => {
            if self.bytecode.len() <= self.cursor+2 {
               parse_script_error!(format!("cannot get length of PUSHDATA2 at {}", self.cursor));
            }
            let v:u16 = (self.bytecode[self.cursor + 1] as u16)
               | (self.bytecode[self.cursor + 2] as u16) << 8;
            (2, v as usize)
         },
         OP_PUSHDATA4 => {
            if self.bytecode.len() <= self.cursor+4 {
               parse_script_error!(format!("cannot get length of PUSHDATA4 at {}", self.cursor));
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
         parse_script_error!(format!("cannot get data[{}] of {} at {}+{}", datalen, info.name, self.cursor, 1+offset));
      }
      Ok((from, to))
   }
}

impl <'a> ::std::iter::Iterator for Parsed<'a> {
   type Item = ::Result<Instruction<'a>>;

   fn next(&mut self) -> Option<::Result<Instruction<'a>>> {
      if self.bytecode.len() <= self.cursor {
         return None
      }
      let code = self.bytecode[self.cursor];
      let info = OPCODE_INFO[code as usize];
      //println!("    next. code[{}]={:x}={}...", cursor0, code, OPCODE_INFO[code as usize].name);
      let inst = match code {
         OP_PUSHDATAFIX_01 ... OP_PUSHDATA4 => {
            match self.parse_pushdata() {
               Err(e) => return Some(Err(e)),
               Ok((from, to)) => {
                  self.cursor = to;
                  Instruction::PushData(&self.bytecode[from..to])
               },
            }
         },
         OP_0 => {
            self.cursor += 1;
            Instruction::PushValue(0)
         },
         OP_1 ... OP_16 => {
            self.cursor += 1;
            Instruction::PushValue((code-OP_1+1) as u64)
         },
         _ => {
            self.cursor += 1;
            Instruction::Nop
         }
      };
      Some(Ok(inst))
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
fn test_decode() {
   use ::utils::h2b;
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                       "41", "04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab")).unwrap();
   // 0x48=72, 0x41=65, 0x48+0x41=137
   
   use super::{Parser, Instruction as I};
   let parser = Parser::new(bytecode.as_slice());
   let mut parsed = parser.iter();

   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(I::PushData(_))));
      if let Some(Ok(I::PushData(data))) = n {
         assert_eq!(data.len(), 0x48);
         assert_eq!(data, &bytecode[1..(1+0x48)]);
         assert_eq!(data[0..4], [0x30, 0x45, 0x02, 0x21]);
      }
   }
   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(I::PushData(_))));
      if let Some(Ok(I::PushData(data))) = n {
         assert_eq!(data.len(), 0x41);
         assert_eq!(data, &bytecode[0x4a..(0x4a+0x41)]);
         assert_eq!(data[0..4], [0x04, 0xc5, 0x4f, 0x8e]);
      }
   }
}

#[test]
fn test_format() {
   use ::utils::{h2b, FmtVec};
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                              "41", "04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab")).unwrap();
   
   let instructions:Vec<_> = Parser::parse(bytecode.as_slice()).map(|r|r.unwrap()).collect();
   assert_eq!("[72] [65]", format!("{}", FmtVec(instructions)));
}
   


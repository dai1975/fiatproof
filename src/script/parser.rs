use super::opcode::*;
use super::{Statement, ParseScriptError};

pub struct Parser<'a>(pub &'a Vec<u8>);

pub struct Iter<'a> {
   bytecode: &'a Vec<u8>,
   cursor:   usize,
}

impl <'a> Parser<'a> {
   pub fn iter(&self) -> Iter<'a> {
      Iter { bytecode: self.0, cursor: 0 }
   }
   pub fn parse(&self) -> ::Result<Vec<Statement>> {
      //use ::ToBytes; print!("bytecode={}", self.0.to_hex().unwrap());
      let mut r:Vec<Statement> = Vec::with_capacity(self.0.len());
      for e in self.iter() {
         match e {
            Ok(stmt) => r.push(stmt),
            Err(e)   => return Err(e),
         }
      }
      Ok(r)
   }
}

impl <'a> ::std::iter::Iterator for Iter<'a> {
   type Item = ::Result<Statement>;
   fn next(&mut self) -> Option<Self::Item> {
      match self.next0() { //::Result<Option<Statement>>
         Ok(None)     => None,
         Ok(Some(s))  => Some(Ok(s)),
         Err(err)     => {
            self.cursor = self.bytecode.len();
            Some(Err(err))
         }
      }
   }
}

impl <'a> Iter<'a> {
   fn next0(&mut self) -> ::Result<Option<Statement>> {
      let len = self.bytecode.len();
      if len <= self.cursor { return Ok(None) }

      let cursor0 = self.cursor;
      let code    = self.bytecode[self.cursor];
      let mut i   = cursor0 + 1;
      //println!("    next. code[{}]={:x}={}...", cursor0, code, OPCODE_INFO[code as usize].name);
      match code {
         OP_0 => {
            self.cursor = i;
            Ok(Some(Statement::Value(0)))
         },
         OP_1NEGATE => {
            self.cursor = i;
            Ok(Some(Statement::Value(-1)))
         },
         OP_1 ... OP_16 => {
            self.cursor = i;
            let v = (code - OP_1 + 1) as i64;
            Ok(Some(Statement::Value(v)))
         },
         OP_0x01 ... OP_PUSHDATA4 => {
            let datalen = match code {
               OP_PUSHDATA1 => {
                  if len <= i { try!(Err(ParseScriptError::new(format!("cannot get length of PUSHDATA1 at {}", cursor0)))); }
                  let v = self.bytecode[i];
                  i += 1;
                  v as usize
               },
               OP_PUSHDATA2 => {
                  if len <= i+1 { try!(Err(ParseScriptError::new(format!("cannot get length of PUSHDATA2 at {}", cursor0)))); }
                  let v:u16 = (self.bytecode[i] as u16) | (self.bytecode[i+1] as u16) << 8;
                  i += 2;
                  v as usize
               },
               OP_PUSHDATA4 => {
                  if len <= i+3 { try!(Err(ParseScriptError::new(format!("cannot get length of PUSHDATA4 at {}", cursor0)))); }
                  let v:u32 = (self.bytecode[i] as u32) | (self.bytecode[i+1] as u32) << 8
                     | (self.bytecode[i+2] as u32) << 16 | (self.bytecode[i+3] as u32) << 24;
                  i += 4;
                  v as usize
               },
               _ => {
                  code as usize
               }
            };
            if len <= i+datalen-1 { // | 0 | ... | cursor0 = OP | [datalen] | i=data[0] ... data[i+datalen-1] | ... | len-1 | EOS
               try!(Err(ParseScriptError::new(
                  format!("cannot get data[{}] of {} at {}", datalen, OPCODE_INFO[code as usize].name, cursor0)
               )));
            }
            let v = self.bytecode[i..(i+datalen)].to_vec().into_boxed_slice();
            self.cursor = i + datalen;
            Ok(Some(Statement::Bytes(v)))
         }
         _ => {
            self.cursor = i;
            Ok(Some(Statement::Op(code)))
         }
      }
   }
}


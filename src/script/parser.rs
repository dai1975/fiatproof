use super::opcode::*;

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

pub struct Parser<'a> {
   bytecode: &'a [u8],
}

pub struct Iter<'a> {
   bytecode: &'a [u8],
   cursor:   usize,
}

impl <'a> Parser<'a> {
   pub fn new<'x>(bytecode: &'x [u8]) -> Parser<'x> {
      Parser { bytecode:bytecode }
   }
   /*
   pub fn iter(&self) -> Iter<'a> {
      Iter { bytecode: self.0, cursor: 0 }
   }
   pub fn parse(&self) -> ::Result<Vec<Parsed>> {
      //use ::ToBytes; print!("bytecode={}", self.0.to_hex().unwrap());
      let mut r:Vec<Parsed> = Vec::with_capacity(self.0.len());
      for e in self.iter() {
         match e {
            Ok(parsed) => r.push(parsed),
            Err(e)   => return Err(e),
         }
      }
      Ok(r)
   }
*/
}

impl <'a> ::std::iter::IntoIterator for Parser<'a> {
   type Item = ::Result<Parsed<'a>>;
   type IntoIter = Iter<'a>;
   fn into_iter(self) -> Self::IntoIter {
      Iter { bytecode: self.bytecode, cursor: 0 }
   }
}

impl <'a> ::std::iter::Iterator for Iter<'a> {
   type Item = ::Result<Parsed<'a>>;
   fn next(&mut self) -> Option<Self::Item> {
      match self.next0() {
         Ok(None)     => None,
         Ok(Some(p))  => Some(Ok(p)),
         Err(err)     => {
            self.cursor = self.bytecode.len();
            Some(Err(err))
         }
      }
   }
}

impl <'a> Iter<'a> {
   fn next0(&mut self) -> ::Result<Option<Parsed<'a>>> {
      let len = self.bytecode.len();
      if len <= self.cursor { return Ok(None) }

      let cursor0 = self.cursor;
      let code    = self.bytecode[self.cursor];
      let info    = OPCODE_INFO[code as usize];
      let mut i   = cursor0 + 1;
      //println!("    next. code[{}]={:x}={}...", cursor0, code, OPCODE_INFO[code as usize].name);
      match code {
         OP_PUSHDATAFIX_01 ... OP_PUSHDATA4 => {
            let datalen = match code {
               OP_PUSHDATA1 => {
                  if len <= i { parse_script_error!(format!("cannot get length of PUSHDATA1 at {}", cursor0)); }
                  let v = self.bytecode[i];
                  i += 1;
                  v as usize
               },
               OP_PUSHDATA2 => {
                  if len <= i+1 { parse_script_error!(format!("cannot get length of PUSHDATA2 at {}", cursor0)); }
                  let v:u16 = (self.bytecode[i] as u16) | (self.bytecode[i+1] as u16) << 8;
                  i += 2;
                  v as usize
               },
               OP_PUSHDATA4 => {
                  if len <= i+3 { parse_script_error!(format!("cannot get length of PUSHDATA4 at {}", cursor0)); }
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
               parse_script_error!(format!("cannot get data[{}] of {} at {}", datalen, info.name, cursor0));
            }
            self.cursor = i + datalen;
            Ok(Some(Parsed { bytecode:self.bytecode, opcode_offset:cursor0, opcode_info:info, data_offset:i, data_len:datalen }))
         },
         _ => {
            self.cursor = i;
            Ok(Some(Parsed { bytecode:self.bytecode, opcode_offset:cursor0, opcode_info:info, data_offset:0, data_len:0 }))
         }
      }
   }
}


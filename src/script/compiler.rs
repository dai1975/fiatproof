use combine::*;
use combine::char::*;

use super::instruction::Instruction;

pub fn lex<'a>(input: &str) -> ::Result<Vec<Instruction<'a>>> {
   enum Token {
      S(String),
      O(String),
   }
   let string_literal = between(
      token('\u{0027}'), token('\u{0027}'), many(satisfy(|c| c != '\u{0027}'))
   ).map(|s:String| Token::S(s));
   let other_literal = many1(
      satisfy(|c:char| c.is_digit(10) || c.is_uppercase())
   ).map(|s:String| Token::O(s));
   let literal = try(string_literal).or(try(other_literal));
   
   let mut p = spaces().with(
      sep_end_by::<Vec<_>,_,_>(literal, many1::<Vec<_>,_>(space())).skip(spaces())
   );
   let tokens = p.parse_stream(input).or_else(|e| {
      Err(::script::ParseScriptError::new(format!("{:?}", e)))
   })?;
   tokens.0.into_iter().fold(Ok(Vec::new()), |acc,t| {
      match (acc,t) {
         (Err(e),_) => Err(e),
         (Ok(mut v), Token::S(s)) => {
            v.push(Instruction::new_data_copy(s.as_bytes()));
            Ok(v)
         },
         (Ok(mut v), Token::O(s)) => {
            use super::opcode::NAME2CODE;
            if let Some(op) = NAME2CODE.get(format!("OP_{}", s).as_str()) {
               v.push(Instruction::Op(op.clone()));
               Ok(v)
            } else if 2 <= s.len() && &s[0..2] == "0x" {
               match i64::from_str_radix(&s[2..], 16) {
                  Ok(val) => {
                     v.push(Instruction::new_value(val));
                     Ok(v)
                  }
                  Err(e) =>  {
                     let msg = format!("malformed hexdigit \"{}\"", s);
                     Err(::Error::ParseScript(::script::ParseScriptError::new(msg)))
                  },
               }
            } else {
               match i64::from_str_radix(&s[..], 10) {
                  Ok(val) => {
                     v.push(Instruction::new_value(val));
                     Ok(v)
                  }
                  Err(e) =>  {
                     let msg = format!("unknown token \"{}\"", s);
                     Err(::Error::ParseScript(::script::ParseScriptError::new(msg)))
                  },
               }
            }
         },
      }
   })
}

pub fn compile_push_data(data:&[u8]) -> ::Result< Vec<u8> > {
   let mut ret = Vec::<u8>::new();
   use super::opcode::*;
   let op = get_opcode_for_pushdata(data)?;
   ret.push(op);
   let _ = match op {
      OP_0 | OP_1NEGATE | OP_1...OP_16 => (),
      OP_PUSHDATAFIX_01 ... OP_PUSHDATAFIX_4B => {
         ret.extend(data);
      },
      OP_PUSHDATA1 => {
         ret.push(data.len() as u8);
         ret.extend(data);
      },
      OP_PUSHDATA2 => {
         let len = (data.len() as u16).to_le();
         let buf: &[u8;2] = unsafe { ::std::mem::transmute(&len) };
         ret.extend(buf);
         ret.extend(data);
      },
      OP_PUSHDATA4 => {
         let len = (data.len() as u32).to_le();
         let buf: &[u8;4] = unsafe { ::std::mem::transmute(&len) };
         ret.extend(buf);
         ret.extend(data);
      },
      _ => { script_error!(format!("unexpected opcode {}", op)) }
   };
   Ok(ret)
}

pub fn compile_push_value(value:i64) -> ::Result< Vec<u8> > {
   let mut ret = Vec::<u8>::new();
   use super::opcode::*;
   if value == 0 {
      ret.push(OP_0);
   } else if value == -1 {
      ret.push(OP_1NEGATE);
   } else if 1 <= value && value <= 16 {
      ret.push(OP_1 + ((value-1) as u8));
   } else {
      let mut tmp = [0u8; 9];
      use super::num::ScriptNum;
      let n = ScriptNum::encode(value, &mut tmp);
      ret.push(OP_PUSHDATAFIX_01 + ((n-1) as u8));
      ret.extend(&tmp[0..n]);
   }
   Ok(ret)
}

pub fn compile(script: &str) -> ::Result<Vec<u8>> {
   let instructions = lex(script)?;
   instructions.into_iter().fold(Ok(Vec::<u8>::new()), |acc, inst| {
      match acc {
         Err(e) => Err(e),
         Ok(mut v) => {
            match inst {
               Instruction::Data(_)  => {
                  compile_push_data(inst.data().unwrap())
                     .map(|bytes| { v.extend(bytes); })
               },
               Instruction::Value(_) => {
                  compile_push_value(inst.value().unwrap())
                     .map(|bytes| { v.extend(bytes); })
               },
               Instruction::Op(_) => {
                  v.push(inst.opcode().unwrap());
                  Ok(())
               },
            }.map(|_| v)
         }
      }
   })
}

#[test]
fn test_lex() {
   use super::opcode::*;
   use super::instruction::Instruction;

   {
      let r = lex("10 100 11");
      assert_matches!(r, Ok(_));
      let r = r.unwrap();
      assert_eq!(r.len(), 3);
      assert_matches!(r[0], Instruction::Op(OP_10));
      assert_matches!(r[1], Instruction::Value(100));
      assert_matches!(r[2], Instruction::Op(OP_11));
   }

   {
      let r = lex("2DUP 3DUP");
      assert_matches!(r, Ok(_));
      let r = r.unwrap();
      assert_eq!(r.len(), 2);
      assert_matches!(r[0], Instruction::Op(OP_2DUP));
      assert_matches!(r[1], Instruction::Op(OP_3DUP));
   }

   {
      let r = lex("2DUP 3DUP 4DUP");
      assert_matches!(r, Err(_));
      use std::error::Error;
      assert_eq!(r.unwrap_err().description(), "unknown token \"4DUP\"");
   }
}


#[test]
fn test_opcode_compiler() {
   use super::opcode::*;
   let r = super::compile("CHECKSIG");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [OP_CHECKSIG]);

   let r = super::compile("0");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [OP_0]);
}

#[test]
fn test_string() {
   let src = "'Hatsune Miku'";
   let r = super::compile(&src);
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [12, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75]);
}

#[test]
fn test_strings() {
   let src = "'Hatsune Miku' 'Kagamine Rin'";
   let r = super::compile(&src);
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [12, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75, 12, 0x4B, 0x61, 0x67, 0x61, 0x6D, 0x69, 0x6E, 0x65, 0x20, 0x52, 0x69, 0x6E]);
}

#[test]
fn test_tailing_space() {
   let r = super::compile("'Hatsune Miku'   ");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [12, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75]);
}


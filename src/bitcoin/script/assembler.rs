use combine::*;
use combine::char::*;

#[derive(Debug)]
pub enum Token {
   String(String),
   Hex(Vec<u8>),
   Digit(i64),
   Op(u8),
}

pub fn lex(input: &str) -> crate::Result<Vec<Token>> {
   enum Tmp {
      S(String), O(String)
   };
   let string_literal = between(
      token('\u{0027}'), token('\u{0027}'), many(satisfy(|c| c != '\u{0027}'))
   ).map(|s:String| Tmp::S(s));
   let other_literal = many1(
      satisfy(|c:char| c=='-' || ('0'<=c && c<='9') || ('a'<=c && c<='z') || ('A'<=c && c<='Z'))
   ).map(|s:String| Tmp::O(s));
   let literal = try(string_literal).or(try(other_literal));
   
   let mut p = spaces().with(
      sep_end_by::<Vec<_>,_,_>(literal, many1::<Vec<_>,_>(space())).skip(spaces())
   );
   let tokens = p.parse_stream(input).or_else(|e| {
      Err(parse_script_error!(format!("{:?}", e)))
   })?;

   let mut ret = Vec::new();
   for tmp in tokens.0.into_iter() {
      match tmp {
         Tmp::S(s) => {
            ret.push(Token::String(s));
         },
         Tmp::O(s) => {
            use super::opcode::NAME2CODE;
            if 2 < s.len() && &s[0..2] == "0x" {
               let v = crate::utils::h2b(&s[2..])?;
               ret.push(Token::Hex(v.into_vec()));
            } else {
               if let Ok(v) = i64::from_str_radix(&s, 10) {
                  ret.push(Token::Digit(v));
               } else if let Some(op) = NAME2CODE.get(format!("OP_{}", s).as_str()) {
                  ret.push(Token::Op(*op));
               } else {
                  raise_script_error!(format!("unknown opcode `{}'", s));
               }
            }
         }
      }
   }
   Ok(ret)
}

pub fn assemble_push_data(data:&[u8]) -> crate::Result<Vec<u8>> {
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
         let buf: &[u8;2] = unsafe { std::mem::transmute(&len) };
         ret.extend(buf);
         ret.extend(data);
      },
      OP_PUSHDATA4 => {
         let len = (data.len() as u32).to_le();
         let buf: &[u8;4] = unsafe { std::mem::transmute(&len) };
         ret.extend(buf);
         ret.extend(data);
      },
      _ => { raise_script_error!(format!("unexpected opcode `{}'", op)) }
   };
   Ok(ret)
}

pub fn assemble_push_value(value:i64) -> crate::Result< Vec<u8> > {
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
      let n = ScriptNum::serialize(value, &mut tmp);
      ret.push(OP_PUSHDATAFIX_01 + ((n-1) as u8));
      ret.extend(&tmp[0..n]);
   }
   Ok(ret)
}

pub fn assemble(script: &str) -> crate::Result<Vec<u8>> {
   let tokens = lex(script)?;
   let mut ret = Vec::<u8>::new();
   for t in tokens.into_iter() {
      match t {
         Token::String(s) => {
            let v = assemble_push_data(s.as_bytes())?;
            ret.extend(v);
         },
         Token::Hex(v) => {
            ret.extend(v);
         },
         Token::Digit(v) => {
            let v = assemble_push_value(v)?;
            ret.extend(v);
         },
         Token::Op(op) => {
            ret.push(op);
         },
      }
   }
   Ok(ret)
}

#[test]
fn test_0_lex() {
   use super::opcode::*;

   {
      let r = lex("0x02 0x01 0x00");
      assert_matches!(r, Ok(_));
      let r = r.unwrap();
      assert_eq!(r.len(), 3);
      assert_matches!(r[0], Token::Hex(ref v) if v == &[0x02]);
      assert_matches!(r[1], Token::Hex(ref v) if v == &[0x01]);
      assert_matches!(r[2], Token::Hex(ref v) if v == &[0x00]);
   }

   {
      let r = lex("0x09 0x00000000 0x00000000 0x10");
      assert_matches!(r, Ok(_));
      let r = r.unwrap();
      assert_eq!(r.len(), 4);
      assert_matches!(r[0], Token::Hex(ref v) if v == &[0x09]);
      assert_matches!(r[1], Token::Hex(ref v) if v == &[0x00, 0x00, 0x00, 0x00]);
      assert_matches!(r[2], Token::Hex(ref v) if v == &[0x00, 0x00, 0x00, 0x00]);
      assert_matches!(r[3], Token::Hex(ref v) if v == &[0x10]);
   }
   {
      let r = lex("10 100 11");
      assert_matches!(r, Ok(_));
      let r = r.unwrap();
      assert_eq!(r.len(), 3);
      assert_matches!(r[0], Token::Digit(10));
      assert_matches!(r[1], Token::Digit(100));
      assert_matches!(r[2], Token::Digit(11));
   }

   {
      let r = lex("2DUP 3DUP");
      assert_matches!(r, Ok(_));
      let r = r.unwrap();
      assert_eq!(r.len(), 2);
      assert_matches!(r[0], Token::Op(OP_2DUP));
      assert_matches!(r[1], Token::Op(OP_3DUP));
   }

   {
      let r = lex("2DUP 3DUP 4DUP");
      assert_matches!(r, Err(_));
      use std::error::Error;
      assert_eq!(r.unwrap_err().description(), "unknown opcode `4DUP'");
   }
}


#[test]
fn test_1_assemble_opcode() {
   use super::opcode::*;
   let r = assemble("CHECKSIG");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [OP_CHECKSIG]);

   let r = assemble("0");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [OP_0]);
}


#[test]
fn test_1_assemble_hex() {
   use super::opcode::*;
   let r = assemble("0x02 0x01 0x00");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [OP_PUSHDATAFIX_02, 0x01, 0x00]);
}

#[test]
fn test_1_string() {
   let src = "'Hatsune Miku'";
   let r = assemble(&src);
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [12, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75]);
}

#[test]
fn test_1_strings() {
   let src = "'Hatsune Miku' 'Kagamine Rin'";
   let r = assemble(&src);
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [12, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75, 12, 0x4B, 0x61, 0x67, 0x61, 0x6D, 0x69, 0x6E, 0x65, 0x20, 0x52, 0x69, 0x6E]);
}

#[test]
fn test_tailing_space() {
   let r = assemble("'Hatsune Miku'   ");
   assert_matches!(r, Ok(_));
   let bytes = r.unwrap();
   assert_eq!(bytes, [12, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75]);
}


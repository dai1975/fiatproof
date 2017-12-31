use combine::*;
use combine::char::*;
use super::ParseScriptError;
   
use super::instruction::Instruction;
use super::pushee::Pushee;

fn instruction_literal<'a, I: 'static + Stream<Item=char>>() -> Box<Parser<Input=I, Output=Instruction<'a>>> {
   let string_literal = between(token('\u{0027}'), token('\u{0027}'),
                                many(satisfy(|c| c != '\u{0027}')))
      .map(|s:String| {
         Instruction::Push(Pushee::new_data_copy(s.as_bytes()))
      });
   let digit_literal  = many1(digit())
      .map(|s:String| {
         let v = s.parse::<i64>().unwrap();
         Instruction::Push(Pushee::new_value(v))
      });
   let hex_literal    = token('0').with(token('x')).with(
      many1(hex_digit()).map(|s:String| {
         let v = i64::from_str_radix(s.as_str(), 16).unwrap();
         Instruction::Push(Pushee::new_value(v))
      }));
   let literal = try(string_literal).or(try(digit_literal)).or(try(hex_literal));
   Box::new(literal)
}

pub fn compile_push_data(data:&[u8]) -> ::Result< Vec<u8> > {
   let mut ret = Vec::<u8>::new();
   use super::opcode::*;
   let op = get_opcode_for_pushdata(data)?;
   ret.push(op);
   let _ = match op {
      OP_0 | OP_1NEGATE | OP_1...OP_16 => (),
      OP_PUSHDATAFIX_01 ... OP_PUSHDATAFIX_48 => {
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
   let mut parser = sep_by::<Vec<_>,_,_>(instruction_literal(), spaces());
   let instructions = {
      //let r = parser.parse(State::new(script)).or_else(|e| //lifetime infer
      let r = parser.parse(script).or_else(|e|
         Err(::script::ParseScriptError::new(format!("{}", e)))
      )?;
      println!("len={}, rest={:?}", r.0.len(), r.1);
      r.0
   };
   instructions.into_iter().fold(Ok(Vec::<u8>::new()), |acc, item| {
      match (acc,item) {
         (Err(e), _) => Err(e),
         (Ok(mut v), Instruction::Push(Pushee::Data(cow))) => {
            compile_push_data(cow.as_ref()).and_then(|d| { v.extend(d); Ok(v) })
         },
         (Ok(mut v), Instruction::Push(Pushee::Value(val, _, _))) => {
            compile_push_value(val).and_then(|d| { v.extend(d); Ok(v) })
         },
         (Ok(mut v), Instruction::Op(op)) => {
            v.push(op); Ok(v)
         },
         (_, _) => script_error!("unexpected instruction")
      }
   })
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
fn test_sep_error() {
   let src = "'Hatsune Miku' ";
   let r = super::compile(&src);
   assert_matches!(r, Err(::Error::ParseScript(_)));
}

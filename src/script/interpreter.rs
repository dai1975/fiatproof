use super::ScriptVerifyFlags;
use ::Tx;
use super::stack::Stack;
use super::checksig::CheckSig;
use super::parser::{Parser, Parsed};
use super::opcode::*;

#[derive(Debug,Clone)]
pub struct Interpreter {
   stack: Stack,
}

pub struct Context<'a> {
   pub bytecode:   &'a [u8],
   pub checksig:   CheckSig<'a>,
   pub codesep:    usize,
   pub conditions: Vec<bool>,
   pub flags:      ScriptVerifyFlags,
}

impl Interpreter {
   pub fn new() -> Self {
      Interpreter { stack: Stack::new() }
   }
   pub fn new_with_stack(stack:Stack) -> Interpreter {
      Interpreter { stack: stack }
   }
   pub fn stack(&self) -> &Stack { &self.stack }

   pub fn eval<'a>(&mut self, bytecode:&'a [u8], tx:&Tx, in_idx:usize, flags:ScriptVerifyFlags) -> ::Result<()> {
      //println!("eval: {}", script);
      //let checker = signature::Checker::new(tx, in_idx);

      let mut ctx = Context {
         bytecode:   bytecode,
         checksig:   CheckSig::new(tx, in_idx),
         codesep:    0,
         conditions: Vec::new(),
         flags:      flags
      };
      let mut last_op = OP_0;
      let parseds = try!(Parser::parse(bytecode));
      for parsed in parseds.into_iter() {
         if last_op == OP_CODESEPARATOR {
            ctx.codesep = parsed.offset;
         }
         //let info = &OPCODE_INFO[code as usize];
         //println!("{:x}={}[{}]", code, info.name, follow.len());

         try!(self.step(&parsed, &mut ctx));
         if 1000 < self.stack.len() {
            script_error!("stack is too long");
         }
         last_op = parsed.opcode;
      }
      if 0 < ctx.conditions.len() {
         script_error!("if clauses are not closed");
      }
      Ok(())
   }
   
   fn step<'a>(&mut self, parsed:&Parsed<'a>, ctx:&'a mut Context) -> ::Result<()> {
      use super::instruction::Instruction as I;
      match parsed.instruction {
         I::Data(_) => {
            self.stack.push_data(parsed.instruction.data().unwrap());
         },
         I::Value(_) => {
            self.stack.push_value(parsed.instruction.value().unwrap());
         },
         I::Op(OP_0) => { self.stack.push_value(0); },
         I::Op(OP_1NEGATE) => { self.stack.push_value(-1); },
         I::Op(OP_2) => { self.stack.push_value(2); },
         I::Op(OP_3) => { self.stack.push_value(3); },
         I::Op(OP_4) => { self.stack.push_value(4); },
         I::Op(OP_5) => { self.stack.push_value(5); },
         I::Op(OP_6) => { self.stack.push_value(6); },
         I::Op(OP_7) => { self.stack.push_value(7); },
         I::Op(OP_8) => { self.stack.push_value(8); },
         I::Op(OP_9) => { self.stack.push_value(9); },
         I::Op(OP_10) => { self.stack.push_value(10); },
         I::Op(OP_11) => { self.stack.push_value(11); },
         I::Op(OP_12) => { self.stack.push_value(12); },
         I::Op(OP_13) => { self.stack.push_value(13); },
         I::Op(OP_14) => { self.stack.push_value(14); },
         I::Op(OP_15) => { self.stack.push_value(15); },
         I::Op(OP_16) => { self.stack.push_value(16); },
         
         I::Op(OP_IF) => { script_error!("not implemented yet"); },
         I::Op(OP_NOTIF) => { script_error!("not implemented yet"); },
         I::Op(OP_ELSE) => { script_error!("not implemented yet"); },
         I::Op(OP_ENDIF) => { script_error!("not implemented yet"); },
         I::Op(OP_RETURN) => { script_error!("not implemented yet"); },
         I::Op(OP_TOALTSTACK) => { script_error!("not implemented yet"); },
         I::Op(OP_FROMALTSTACK) => { script_error!("not implemented yet"); },
         I::Op(OP_2DROP) => { script_error!("not implemented yet"); },
         I::Op(OP_2DUP) => { script_error!("not implemented yet"); },
         I::Op(OP_3DUP) => { script_error!("not implemented yet"); },
         I::Op(OP_2OVER) => { script_error!("not implemented yet"); },
         I::Op(OP_2ROT) => { script_error!("not implemented yet"); },
         I::Op(OP_2SWAP) => { script_error!("not implemented yet"); },
         I::Op(OP_IFDUP) => { script_error!("not implemented yet"); },
         I::Op(OP_DEPTH) => {
            let v = self.stack.len() as i64;
            self.stack.push_value(v);
         },
         I::Op(OP_DROP) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_DUP) => {
            let _ = try!(self.stack.dup_at(-1));
         },
         I::Op(OP_NIP) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_OVER) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_PICK) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_ROLL) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_ROT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_SWAP) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_TUCK) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_CAT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_SUBSTR) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_LEFT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_RIGHT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_SIZE) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_INVERT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_AND) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_OR) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_XOR) => { raise_script_error!("not implemented yet"); },
         I::Op(code) if code == OP_EQUAL || code == OP_EQUALVERIFY => {
            let e1 = try!(self.stack.pop());
            let e2 = try!(self.stack.pop());
            let eq = e1 == e2;
            if code == OP_EQUALVERIFY {
               if !eq {
                  raise_script_error!("equalverify");
               }
            } else {
               self.stack.push_bool(eq);
            }
         },
         I::Op(OP_1ADD) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_1SUB) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_2MUL) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_2DIV) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_NEGATE) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_ABS) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_NOT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_0NOTEQUAL) => { raise_script_error!("not implemented yet"); },
         
         I::Op(op) if op == OP_ADD
            || op == OP_SUB
            || op == OP_BOOLAND
            || op == OP_BOOLOR
            || op == OP_NUMEQUAL
            || op == OP_NUMEQUALVERIFY
            || op == OP_NUMNOTEQUAL
            || op == OP_LESSTHAN
            || op == OP_GREATERTHAN
            || op == OP_LESSTHANOREQUAL
            || op == OP_GREATERTHANOREQUAL
            || op == OP_MIN
            || op == OP_MAX
            =>
         {
            println!("stack: {}", self.stack.len());
            if self.stack.len() < 2 {
               raise_script_interpret_error!(InvalidStackOperation);
            }
            let n2 = self.stack.at(-2)?.value(ctx.flags.is_require_minimal(), 4)?;
            let n1 = self.stack.at(-1)?.value(ctx.flags.is_require_minimal(), 4)?;
            enum Tmp { N(i64), B(bool) };
            let tmp = match op {
               OP_ADD                => Tmp::N(n1 + n2),
               OP_SUB                => Tmp::N(n1 - n2),
               OP_BOOLAND            => Tmp::B((n1 != 0) && (n2 != 0)),
               OP_BOOLOR             => Tmp::B((n1 != 0) || (n2 != 0)),
               OP_NUMEQUAL           => Tmp::B(n1 == n2),
               OP_NUMEQUALVERIFY     => Tmp::B(n1 == n2),
               OP_NUMNOTEQUAL        => Tmp::B(n1 != n2),
               OP_LESSTHAN           => Tmp::B(n1 < n2),
               OP_GREATERTHAN        => Tmp::B(n1 > n2),
               OP_LESSTHANOREQUAL    => Tmp::B(n1 <= n2),
               OP_GREATERTHANOREQUAL => Tmp::B(n1 >= n2),
               OP_MIN                => Tmp::N(if n1 < n2 { n1 } else { n2 }),
               OP_MAX                => Tmp::N(if n1 > n2 { n1 } else { n2 }),
               _ => { raise_script_error!("unexpected opcode"); Tmp::N(0) }
            };
            self.stack.pop()?;
            self.stack.pop()?;
            match tmp {
               Tmp::N(v) => self.stack.push_value(v),
               Tmp::B(v) => self.stack.push_bool(v),
            }
            if op == OP_NUMEQUALVERIFY {
               if self.stack.at(-1).unwrap().as_bool() {
                  self.stack.pop()?;
               } else {
                  raise_script_interpret_error!(NumEqualVerify);
               }
            }
         },            

         I::Op(OP_MUL) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_DIV) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_MOD) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_LSHIFT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_RSHIFT) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_WITHIN) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_RIPEMD160) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_SHA1) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_SHA256) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_HASH160) => {
            if self.stack.len() < 1 {
               raise_script_interpret_error!(InvalidStackOperation);
            }
            use ::crypto::{Hash160, Hasher};
            let entry = try!(self.stack.pop());
            let hash = Hash160::hash(entry.data());
            self.stack.push_data(hash.as_ref());
         },
         I::Op(OP_HASH256) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_CODESEPARATOR) => {
         },
         I::Op(op) if op == OP_CHECKSIG || op == OP_CHECKSIGVERIFY => {
            let pubkey   = try!(self.stack.pop());
            let signature= try!(self.stack.pop());
            let subscript = &ctx.bytecode[ctx.codesep..];
            let r = ctx.checksig.verify(subscript, pubkey.data(), signature.data(), ctx.flags).is_ok();
            if op == OP_CHECKSIGVERIFY {
               if !r { raise_script_error!("verify failed") }
            } else {
               self.stack.push_bool(r);
            }
         },
         I::Op(OP_CHECKMULTISIG) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_CHECKMULTISIGVERIFY) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_CHECKLOCKTIMEVERIFY) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_CHECKSEQUENCEVERIFY) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_SMALLINTEGER) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_PUBKEYS) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_PUBKEYHASH) => { raise_script_error!("not implemented yet"); },
         I::Op(OP_PUBKEY) => { raise_script_error!("not implemented yet"); },
         I::Op(code) => {
            let info = &OPCODE_INFO[code as usize];
            raise_script_error!(format!("  invalid op {}(0x{:x})", info.name, code));
         },
      }
      Ok(())
   }
}

pub fn verify(sigscr:&[u8], pkscr:&[u8], tx:&Tx, in_idx:usize, flags:ScriptVerifyFlags) -> ::Result<()> {
   let mut itpr = Interpreter::new();
   let _ = itpr.eval(sigscr, tx, in_idx, flags)?;
   let _ = itpr.eval(pkscr, tx, in_idx, flags)?;
   let _ = itpr.stack().at(-1).map_err(|_| {
      script_interpret_error!(EvalFalse)
   }).and_then(|e| {
      if !e.as_bool() {
         Err(script_interpret_error!(EvalFalse))
      } else {
         Ok(())
      }
   })?;
   Ok(())
}
   

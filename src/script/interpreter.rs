use ::Tx;
use super::ScriptError;
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
   pub flags:      u32,
}

impl Interpreter {
   pub fn new() -> Self {
      Interpreter { stack: Stack::new() }
   }
   pub fn new_with_stack(stack:Stack) -> Interpreter {
      Interpreter { stack: stack }
   }
   pub fn stack(&self) -> &Stack { &self.stack }

   pub fn eval<'a>(&mut self, bytecode:&'a [u8], tx:&Tx, in_idx:usize, flags:u32) -> ::Result<()> {
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
         I::Op(OP_DROP) => { script_error!("not implemented yet"); },
         I::Op(OP_DUP) => {
            let _ = try!(self.stack.dup_at(-1));
         },
         I::Op(OP_NIP) => { script_error!("not implemented yet"); },
         I::Op(OP_OVER) => { script_error!("not implemented yet"); },
         I::Op(OP_PICK) => { script_error!("not implemented yet"); },
         I::Op(OP_ROLL) => { script_error!("not implemented yet"); },
         I::Op(OP_ROT) => { script_error!("not implemented yet"); },
         I::Op(OP_SWAP) => { script_error!("not implemented yet"); },
         I::Op(OP_TUCK) => { script_error!("not implemented yet"); },
         I::Op(OP_CAT) => { script_error!("not implemented yet"); },
         I::Op(OP_SUBSTR) => { script_error!("not implemented yet"); },
         I::Op(OP_LEFT) => { script_error!("not implemented yet"); },
         I::Op(OP_RIGHT) => { script_error!("not implemented yet"); },
         I::Op(OP_SIZE) => { script_error!("not implemented yet"); },
         I::Op(OP_INVERT) => { script_error!("not implemented yet"); },
         I::Op(OP_AND) => { script_error!("not implemented yet"); },
         I::Op(OP_OR) => { script_error!("not implemented yet"); },
         I::Op(OP_XOR) => { script_error!("not implemented yet"); },
         I::Op(code) if code == OP_EQUAL || code == OP_EQUALVERIFY => {
            let e1 = try!(self.stack.pop());
            let e2 = try!(self.stack.pop());
            let eq = e1 == e2;
            if code == OP_EQUALVERIFY {
               if !eq {
                  script_error!("equalverify");
               }
            } else {
               self.stack.push_bool(eq);
            }
         },
         I::Op(OP_1ADD) => { script_error!("not implemented yet"); },
         I::Op(OP_1SUB) => { script_error!("not implemented yet"); },
         I::Op(OP_2MUL) => { script_error!("not implemented yet"); },
         I::Op(OP_2DIV) => { script_error!("not implemented yet"); },
         I::Op(OP_NEGATE) => { script_error!("not implemented yet"); },
         I::Op(OP_ABS) => { script_error!("not implemented yet"); },
         I::Op(OP_NOT) => { script_error!("not implemented yet"); },
         I::Op(OP_0NOTEQUAL) => { script_error!("not implemented yet"); },
         I::Op(OP_ADD) => { script_error!("not implemented yet"); },
         I::Op(OP_SUB) => { script_error!("not implemented yet"); },
         I::Op(OP_MUL) => { script_error!("not implemented yet"); },
         I::Op(OP_DIV) => { script_error!("not implemented yet"); },
         I::Op(OP_MOD) => { script_error!("not implemented yet"); },
         I::Op(OP_LSHIFT) => { script_error!("not implemented yet"); },
         I::Op(OP_RSHIFT) => { script_error!("not implemented yet"); },
         I::Op(OP_BOOLAND) => { script_error!("not implemented yet"); },
         I::Op(OP_BOOLOR) => { script_error!("not implemented yet"); },
         I::Op(OP_NUMEQUAL) => { script_error!("not implemented yet"); },
         I::Op(OP_NUMEQUALVERIFY) => { script_error!("not implemented yet"); },
         I::Op(OP_NUMNOTEQUAL) => { script_error!("not implemented yet"); },
         I::Op(OP_LESSTHAN) => { script_error!("not implemented yet"); },
         I::Op(OP_GREATERTHAN) => { script_error!("not implemented yet"); },
         I::Op(OP_LESSTHANOREQUAL) => { script_error!("not implemented yet"); },
         I::Op(OP_GREATERTHANOREQUAL) => { script_error!("not implemented yet"); },
         I::Op(OP_MIN) => { script_error!("not implemented yet"); },
         I::Op(OP_MAX) => { script_error!("not implemented yet"); },
         I::Op(OP_WITHIN) => { script_error!("not implemented yet"); },
         I::Op(OP_RIPEMD160) => { script_error!("not implemented yet"); },
         I::Op(OP_SHA1) => { script_error!("not implemented yet"); },
         I::Op(OP_SHA256) => { script_error!("not implemented yet"); },
         I::Op(OP_HASH160) => {
            use ::crypto::{Hash160, Hasher};
            let entry = try!(self.stack.pop());
            let hash = Hash160::hash(entry.data());
            self.stack.push_data(hash.as_ref());
         },
         I::Op(OP_HASH256) => { script_error!("not implemented yet"); },
         I::Op(OP_CODESEPARATOR) => {
         },
         I::Op(op) if op == OP_CHECKSIG || op == OP_CHECKSIGVERIFY => {
            let pubkey   = try!(self.stack.pop());
            let signature= try!(self.stack.pop());
            let subscript = &ctx.bytecode[ctx.codesep..];
            let r = ctx.checksig.verify(subscript, pubkey.data(), signature.data(), ctx.flags).is_ok();
            if op == OP_CHECKSIGVERIFY {
               if !r { script_error!("verify failed") }
            } else {
               self.stack.push_bool(r);
            }
         },
         I::Op(OP_CHECKMULTISIG) => { script_error!("not implemented yet"); },
         I::Op(OP_CHECKMULTISIGVERIFY) => { script_error!("not implemented yet"); },
         I::Op(OP_CHECKLOCKTIMEVERIFY) => { script_error!("not implemented yet"); },
         I::Op(OP_CHECKSEQUENCEVERIFY) => { script_error!("not implemented yet"); },
         I::Op(OP_SMALLINTEGER) => { script_error!("not implemented yet"); },
         I::Op(OP_PUBKEYS) => { script_error!("not implemented yet"); },
         I::Op(OP_PUBKEYHASH) => { script_error!("not implemented yet"); },
         I::Op(OP_PUBKEY) => { script_error!("not implemented yet"); },
         I::Op(code) => {
            let info = &OPCODE_INFO[code as usize];
            println!("  invalid op {}(0x{:x})", info.name, code);
         },
      }
      Ok(())
   }
}

pub fn verify(sigscr:&[u8], pkscr:&[u8], tx:&Tx, in_idx:usize, flags:u32) -> ::Result<()> {
   let mut itpr = Interpreter::new();
   let _ = itpr.eval(sigscr, tx, in_idx, flags)?;
   let _ = itpr.eval(pkscr, tx, in_idx, flags)?;
   let _ = itpr.stack().at(-1).or_else(|_| {
      Err(ScriptError::new("result in false stack"))
   }).and_then(|e| {
      if !e.as_bool() {
         Err(ScriptError::new("result in false stack"))
      } else {
         Ok(())
      }
   })?;
   Ok(())
}
   

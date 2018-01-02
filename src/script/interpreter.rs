use ::Tx;
use super::pushee::Pushee;
use super::stack::Stack;
use super::checksig::CheckSig;
use super::instruction::Instruction;
use super::parser::{Parser, Parsed};
use super::opcode::*;

#[derive(Debug,Clone)]
pub struct Interpreter<'a> {
   stack: Stack<'a>,
}

pub struct Context<'a> {
   pub bytecode:   &'a [u8],
   pub checksig:   CheckSig<'a>,
   pub codesep:    usize,
   pub conditions: Vec<bool>,
   pub flags:      u32,
}

impl <'a> Interpreter<'a> {
   pub fn new() -> Self {
      Interpreter { stack: Stack::new() }
   }
   pub fn new_with_stack<'x>(stack:Stack<'x>) -> Interpreter<'x> {
      Interpreter { stack: stack }
   }
   pub fn stack(&self) -> &Stack { &self.stack }

   pub fn eval(&mut self, bytecode:&'a [u8], tx:&Tx, in_idx:usize, flags:u32) -> ::Result<bool> {
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
      Ok(ctx.conditions.len() == 0)
   }
   
   fn step(&mut self, parsed:&Parsed<'a>, ctx:&mut Context) -> ::Result<()> {
      match parsed.instruction {
         Instruction::Push(ref pushee) => {
            self.stack.push(pushee.clone());
            Ok(())
         },
         Instruction::Op(OP_DUP) => {
            let _ = try!(self.stack.dup_at(-1));
            Ok(())
         },
         Instruction::Op(OP_HASH160) => {
            use ::crypto::{Hash160, Hasher};
            let pushee = try!(self.stack.pop());
            let hash = Hash160::hash(pushee.data());
            self.stack.push(Pushee::new_data_copy(hash.as_ref()));
            Ok(())
         },
         Instruction::Op(code) if code == OP_EQUAL || code == OP_EQUALVERIFY => {
            let e1 = try!(self.stack.pop());
            let e2 = try!(self.stack.pop());
            let eq = e1 == e2;
            if code == OP_EQUALVERIFY {
               if !eq {
                  script_error!("equalverify");
               }
            } else {
               self.stack.push_bool(eq)
            }
            Ok(())
         }
         Instruction::Op(OP_CODESEPARATOR) => {
            Ok(())
         },
         Instruction::Op(op) if op == OP_CHECKSIG || op == OP_CHECKSIGVERIFY => {
            let pubkey   = try!(self.stack.pop());
            let signature= try!(self.stack.pop());
            let subscript = &ctx.bytecode[ctx.codesep..];
            let r = ctx.checksig.verify(subscript, pubkey.data(), signature.data(), ctx.flags).is_ok();
            if op == OP_CHECKSIGVERIFY {
               if !r { script_error!("verify failed") }
            } else {
               self.stack.push_bool(r);
            }
            Ok(())
         },
         Instruction::Op(code) => {
            let info = &OPCODE_INFO[code as usize];
            println!("  unimplemented {}(0x{:x})", info.name, code);
            Ok(())
         },
      }
   }
}


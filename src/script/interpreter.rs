use ::Tx;
use super::stack::Stack;
use super::checksig::CheckSig;
use super::parser::{Parser, Parsed};
use super::opcode::*;
use super::apriori::*;

#[derive(Debug,Clone,Copy,Default)]
pub struct Flags {
   pub script_verify: super::flags::ScriptVerify,
   pub sig_version:   super::flags::SigVersion,
}

#[derive(Debug,Clone)]
pub struct Interpreter {
   stack: Stack,
}

pub struct Context<'a> {
   pub bytecode:   &'a [u8],
   pub checksig:   CheckSig<'a>,
   pub codesep:    usize,
   pub conditions: Vec<bool>,
   pub flags:      &'a Flags,
   pub op_count:   usize,
   pub altstack:   Stack,
}

impl Interpreter {
   pub fn new() -> Self {
      Interpreter { stack: Stack::new() }
   }
   pub fn new_with_stack(stack:Stack) -> Interpreter {
      Interpreter { stack: stack }
   }
   pub fn stack(&self) -> &Stack { &self.stack }

   pub fn eval<'a>(&mut self, bytecode:&'a [u8], tx:&Tx, in_idx:usize, flags:&Flags) -> ::Result<()> {
      //println!("eval: {}", script);
      //let checker = signature::Checker::new(tx, in_idx);
      if MAX_SCRIPT_SIZE < bytecode.len() {
         raise_script_interpret_error!(ScriptSize);
      }

      let mut ctx = Context {
         bytecode:   bytecode,
         checksig:   CheckSig::new(tx, in_idx),
         codesep:    0,
         conditions: Vec::<bool>::new(),
         flags:      flags,
         op_count:   0,
         altstack:   Stack::new(),
      };
      let mut last_op = OP_0;
      let parseds = Parser::parse(bytecode)
         .map_err(|_| script_interpret_error!(BadOpcode))?;
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
         raise_script_interpret_error!(UnbalancedConditional);
      }
      Ok(())
   }
   
   fn step<'a>(&mut self, parsed:&Parsed<'a>, ctx:&'a mut Context) -> ::Result<()> {
      use super::instruction::Instruction as I;
      let is_exec = ctx.conditions.iter().all(|c| *c);
      match parsed.instruction {
         I::Op(op) if op == OP_CAT
            || op == OP_SUBSTR
            || op == OP_LEFT
            || op == OP_RIGHT
            || op == OP_INVERT
            || op == OP_AND
            || op == OP_OR
            || op == OP_XOR
            || op == OP_2MUL
            || op == OP_2DIV
            || op == OP_MUL
            || op == OP_DIV
            || op == OP_MOD
            || op == OP_LSHIFT
            || op == OP_RSHIFT
            =>
         {
            raise_script_interpret_error!(DisabledOpcode);
         },
         _ => (),
      }
      match parsed.instruction {
         I::Data(_, _) => {
            let data = parsed.instruction.data().unwrap();
            if MAX_SCRIPT_ELEMENT_SIZE < data.len() {
               raise_script_interpret_error!(PushSize);
            }
            if is_exec {
               if ctx.flags.script_verify.is_require_minimal()
                  && !parsed.instruction.check_minimal_push()
               {
                  raise_script_interpret_error!(MinimalData);
               }
               self.stack.push_data(data);
            }
         },
         I::Value(_, _) => {
            if is_exec {
               if ctx.flags.script_verify.is_require_minimal()
                  && !parsed.instruction.check_minimal_push()
               {
                  raise_script_interpret_error!(MinimalData);
               }
               self.stack.push_value(parsed.instruction.value().unwrap());
            }
         },
         I::Op(op) => {
            println!("op={}", OPCODE_INFO[op as usize].name);
            
            ctx.op_count += 1;
            if MAX_OPS_PER_SCRIPT < ctx.op_count {
               raise_script_interpret_error!(OpCount);
            }
            if is_exec || (OP_IF <= op && op <= OP_ENDIF) {
               match op {
                  OP_0 => { self.stack.push_value(0); },
                  OP_1NEGATE => { self.stack.push_value(-1); },
                  _ if OP_1 <= op && op <= OP_16 => {
                     self.stack.push_value((op - OP_1 + 1) as i64);
                  },
                  OP_NOP => (),
                  _ if op == OP_IF || op == OP_NOTIF => {
                     let f = if is_exec {
                        if self.stack.len() < 1 {
                           raise_script_interpret_error!(InvalidStackOperation);
                        }
                        let r = {
                           let e = self.stack.at(-1).unwrap();
                           if ctx.flags.sig_version.is_witness_v0() &&
                              ctx.flags.script_verify.is_minimal_if()
                           {
                              if !e.is_minimal_if() {
                                 raise_script_interpret_error!(MinimalIf);
                              }
                           }
                           if op == OP_IF { e.as_bool() } else { !e.as_bool() }
                        };
                        let _ = self.stack.pop()?;
                        r
                     } else {
                        false
                     };
                     ctx.conditions.push(f);
                  },
                  OP_ELSE => {
                     let len = ctx.conditions.len();
                     if len < 1 {
                        raise_script_interpret_error!(UnbalancedConditional);
                     }
                     ctx.conditions[len-1] = !ctx.conditions[len-1];
                  },
                  OP_ENDIF => {
                     if ctx.conditions.len() < 1 {
                        raise_script_interpret_error!(UnbalancedConditional);
                     }
                     ctx.conditions.pop();
                  },
                  OP_VERIFY => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     if !self.stack.at(-1).unwrap().as_bool() {
                        raise_script_interpret_error!(Verify);
                     }
                     self.stack.pop()?;
                  },
                  OP_RETURN => { raise_script_error!("not implemented yet"); },
                  OP_TOALTSTACK => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     ctx.altstack.push(self.stack.at(-1).unwrap().clone());
                     self.stack.pop()?;
                  },
                  OP_FROMALTSTACK => {
                     if ctx.altstack.len() < 1 {
                        raise_script_interpret_error!(InvalidAltstackOperation);
                     }
                     self.stack.push(ctx.altstack.at(-1).unwrap().clone());
                     ctx.altstack.pop()?;
                  },
                  OP_IFDUP => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let pushee = {
                        let e = self.stack.at(-1).unwrap();
                        if e.as_bool() {
                           Some(e.clone())
                        } else {
                           None
                        }
                     };
                     if let Some(e) = pushee {
                        self.stack.push(e);
                     }
                  },
                  OP_DEPTH => {
                     let v = self.stack.len() as i64;
                     self.stack.push_value(v);
                  },
                  OP_DROP => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.pop()?;
                  },
                  OP_2DROP => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.pop()?;
                     self.stack.pop()?;
                  },
                  OP_DUP => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let _ = self.stack.dup_at(-1)?;
                  },
                  OP_2DUP => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let _ = self.stack.dup_at(-2)?;
                     let _ = self.stack.dup_at(-2)?;
                  },
                  OP_3DUP => {
                     if self.stack.len() < 3 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let _ = self.stack.dup_at(-3)?;
                     let _ = self.stack.dup_at(-3)?;
                     let _ = self.stack.dup_at(-3)?;
                  },
                  OP_NIP => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let _ = self.stack.remove_at(-2)?;
                  },
                  OP_OVER => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.dup_at(-2)?;
                  },
                  OP_2OVER => {
                     if self.stack.len() < 4 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.dup_at(-4)?;
                     self.stack.dup_at(-4)?;
                  },
                  _ if op == OP_PICK || op == OP_ROLL => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n = self.stack.pop()?.as_i32(ctx.flags.script_verify.is_require_minimal(), 4)? as usize;
                     if n < 0 || self.stack.len() <= (n as usize) {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n = (self.stack.len() - n - 1) as isize;
                     if op == OP_ROLL {
                        let e = self.stack.remove_at(n)?;
                        self.stack.push(e);
                     } else { //OP_PICK
                        self.stack.dup_at(n)?;
                     }
                  },
                  OP_ROT => {
                     if self.stack.len() < 3 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.swap(-3, -2)?;
                     self.stack.swap(-2, -1)?;
                  },
                  OP_2ROT => {
                     if self.stack.len() < 6 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.swap(-6, -4)?;
                     self.stack.swap(-5, -3)?;
                     self.stack.swap(-4, -2)?;
                     self.stack.swap(-3, -1)?;
                  },
                  OP_SWAP => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.swap(-2, -1)?;
                  },
                  OP_2SWAP => {
                     if self.stack.len() < 4 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     self.stack.swap(-4, -2)?;
                     self.stack.swap(-3, -1)?;
                  },
                  OP_TUCK => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let e = { self.stack.at(-1)?.clone() };
                     self.stack.insert_at(-2, e);
                  },
            
                  OP_SIZE => { raise_script_error!("not implemented yet"); },
                  _ if op == OP_EQUAL || op == OP_EQUALVERIFY => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let e1 = try!(self.stack.pop());
                     let e2 = try!(self.stack.pop());
                     let eq = e1 == e2;
                     if op == OP_EQUALVERIFY {
                        if !eq {
                           raise_script_error!("equalverify");
                        }
                     } else {
                        self.stack.push_bool(eq);
                     }
                  },
                  OP_1ADD => { raise_script_error!("not implemented yet"); },
                  OP_1SUB => { raise_script_error!("not implemented yet"); },
                  OP_NEGATE => { raise_script_error!("not implemented yet"); },
                  OP_ABS => { raise_script_error!("not implemented yet"); },
                  OP_NOT => { raise_script_error!("not implemented yet"); },
                  OP_0NOTEQUAL => { raise_script_error!("not implemented yet"); },
            
                  _ if op == OP_ADD
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
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n2 = self.stack.at(-2)?.value(ctx.flags.script_verify.is_require_minimal(), 4)?;
                     let n1 = self.stack.at(-1)?.value(ctx.flags.script_verify.is_require_minimal(), 4)?;
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
                  
                  OP_MOD => { raise_script_error!("not implemented yet"); },
                  OP_WITHIN => { raise_script_error!("not implemented yet"); },
                  _ if op == OP_RIPEMD160
                     || op == OP_SHA1
                     || op == OP_SHA256
                     || op == OP_HASH160
                     || op == OP_HASH256
                     =>
                  {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let hash = {
                        let e = self.stack.at(-1)?;
                        use ::crypto::{Ripemd160, Sha1, Sha256, Hash160, DHash256, Hasher};
                        match op {
                           OP_RIPEMD160 => Ripemd160::hash(e.data()),
                           OP_SHA1      => Sha1::hash(e.data()),
                           OP_SHA256    => Sha256::hash(e.data()),
                           OP_HASH160   => Hash160::hash(e.data()),
                           OP_HASH256   => DHash256::hash(e.data()),
                           _ => {
                              raise_script_error!("bad match");
                              Box::new([0u8;0])
                           },
                        }
                     };
                     self.stack.pop()?;
                     self.stack.push_data(hash.as_ref());
                  },
                  OP_HASH256 => { raise_script_error!("not implemented yet"); },
                  OP_CODESEPARATOR => {
                  },
                  _ if op == OP_CHECKSIG || op == OP_CHECKSIGVERIFY => {
                     let pubkey   = try!(self.stack.pop());
                     let signature= try!(self.stack.pop());
                     let subscript = &ctx.bytecode[ctx.codesep..];
                     let r = ctx.checksig.verify(subscript, pubkey.data(), signature.data(), ctx.flags.script_verify).is_ok();
                     if op == OP_CHECKSIGVERIFY {
                        if !r { raise_script_error!("verify failed") }
                     } else {
                        self.stack.push_bool(r);
                     }
                  },
                  OP_CHECKMULTISIG => { raise_script_error!("not implemented yet"); },
                  OP_CHECKMULTISIGVERIFY => { raise_script_error!("not implemented yet"); },
                  OP_CHECKLOCKTIMEVERIFY => { raise_script_error!("not implemented yet"); },
                  OP_CHECKSEQUENCEVERIFY => { raise_script_error!("not implemented yet"); },
                  OP_SMALLINTEGER => { raise_script_error!("not implemented yet"); },
                  OP_PUBKEYS => { raise_script_error!("not implemented yet"); },
                  OP_PUBKEYHASH => { raise_script_error!("not implemented yet"); },
                  OP_PUBKEY => { raise_script_error!("not implemented yet"); },
                  _ => {
                     let info = &OPCODE_INFO[op as usize];
                     raise_script_interpret_error!(BadOpcode);
                  },
               } //match op
            } //if is_exec
         } // I::Op
      } //match instruction
      if MAX_STACK_SIZE < self.stack.len() + ctx.altstack.len() {
         raise_script_interpret_error!(StackSize);
      }
      Ok(())
   }
}

pub fn verify(sigscr:&[u8], pkscr:&[u8], tx:&Tx, in_idx:usize, flags:&Flags) -> ::Result<()> {
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
   

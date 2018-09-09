extern crate crypto;

use ::bitcoin::datatypes::{Tx, TxIn};
use super::flags::Flags;
use super::stack::Stack;
use super::checker;
use super::parser::{Parser, Parsed};
use super::opcode::*;
use super::apriori::*;

#[derive(Debug,Clone)]
pub struct Interpreter {
   stack: Stack,
}

pub struct Context<'a> {
   pub bytecode:   &'a [u8],
   pub tx:         &'a Tx,
   pub txin_idx:   usize,
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
   pub fn pop_stack(&mut self) -> ::Result< super::stack::Entry > { self.stack.pop() }

   pub fn eval<'a>(&mut self, bytecode:&'a [u8], tx:&Tx, txin_idx:usize, flags:&Flags) -> ::Result<()> {
      //println!("eval: {}", script);
      //let checker = signature::Checker::new(tx, in_idx);
      if MAX_SCRIPT_SIZE < bytecode.len() {
         raise_script_interpret_error!(ScriptSize);
      }
      let mut ctx = Context {
         bytecode:   bytecode,
         tx:         tx,
         txin_idx:   txin_idx,
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
      let is_require_minimal = ctx.flags.script_verify.is_require_minimal();
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
               if is_require_minimal && !parsed.instruction.check_minimal_push()
               {
                  raise_script_interpret_error!(MinimalData);
               }
               self.stack.push_data(data);
            }
         },
         I::Value(_, _) => {
            if is_exec {
               if is_require_minimal && !parsed.instruction.check_minimal_push()
               {
                  raise_script_interpret_error!(MinimalData);
               }
               self.stack.push_value(parsed.instruction.value().unwrap());
            }
         },
         I::Op(op) => {
            let _info = &OPCODE_INFO[op as usize];
            //println!("op={}", _info.name);

            if OP_16 < op {
               ctx.op_count += 1;
               if MAX_OPS_PER_SCRIPT < ctx.op_count {
                  raise_script_interpret_error!(OpCount);
               }
            }
            if is_exec || (OP_IF <= op && op <= OP_ENDIF) {
               match op {
                  OP_0 => { self.stack.push_value(0); },
                  OP_1NEGATE => { self.stack.push_value(-1); },
                  _ if OP_1 <= op && op <= OP_16 => {
                     self.stack.push_value((op - OP_1 + 1) as i64);
                  },
                  OP_NOP => (),

                  _ if op == OP_NOP1 || op == OP_NOP4 || op == OP_NOP5 || op == OP_NOP6
                     || op == OP_NOP7 || op == OP_NOP8 || op == OP_NOP9 || op == OP_NOP10 =>
                  {
                     if ctx.flags.script_verify.is_discourage_upgradable_nops() {
                        raise_script_interpret_error!(DiscourageUpgradableNops);
                     }                        
                  },
                  
                  _ if op == OP_IF || op == OP_NOTIF => {
                     let f = if is_exec {
                        if self.stack.len() < 1 {
                           raise_script_interpret_error!(UnbalancedConditional);
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
                  OP_RETURN => {
                     raise_script_interpret_error!(OpReturn);
                  },
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
                     let n = self.stack.pop()?.as_i32(is_require_minimal, 4)?;
                     if n < 0 || self.stack.len() <= (n as usize) {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n = (self.stack.len() - (n as usize) - 1) as isize;
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
                     self.stack.insert_at(-2, e)?;
                  },
            
                  OP_SIZE => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let v = self.stack.at(-1)?.data().len();
                     self.stack.push_value(v as i64);
                  },
               
                  _ if op == OP_EQUAL || op == OP_EQUALVERIFY => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let e1 = try!(self.stack.pop());
                     let e2 = try!(self.stack.pop());
                     let eq = e1 == e2;
                     if op == OP_EQUALVERIFY {
                        if !eq {
                           raise_script_interpret_error!(EqualVerify);
                        }
                     } else {
                        self.stack.push_bool(eq);
                     }
                  },

                  _ if op == OP_1ADD
                     || op == OP_1SUB
                     || op == OP_NEGATE
                     || op == OP_ABS
                     || op == OP_NOT
                     || op == OP_0NOTEQUAL
                     =>
                  {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n = self.stack.at(-1)?.value(is_require_minimal, 4)?;
                     let v = match op {
                        OP_1ADD => n + 1,
                        OP_1SUB => n - 1,
                        OP_NEGATE => -n,
                        OP_ABS => if n < 0 { -n } else { n },
                        OP_NOT => if n == 0 { 1 } else { 0 },
                        OP_0NOTEQUAL => if n != 0 { 1 } else { 0 },
                        _ => { raise_script_error!("unexpected opcode"); 0 }
                     };
                     let _ = self.stack.pop()?;
                     self.stack.push_value(v);
                  },
                  
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
                     let n1 = self.stack.at(-2)?.value(is_require_minimal, 4)?;
                     let n2 = self.stack.at(-1)?.value(is_require_minimal, 4)?;
                     let tmp = match op {
                        OP_ADD                => n1 + n2,
                        OP_SUB                => n1 - n2,
                        OP_BOOLAND            => if (n1 != 0) && (n2 != 0) { 1 } else { 0 },
                        OP_BOOLOR             => if (n1 != 0) || (n2 != 0) { 1 } else { 0 },
                        OP_NUMEQUAL           => if n1 == n2 { 1 } else { 0 },
                        OP_NUMEQUALVERIFY     => if n1 == n2 { 1 } else { 0 },
                        OP_NUMNOTEQUAL        => if n1 != n2 { 1 } else { 0 },
                        OP_LESSTHAN           => if n1 < n2 { 1 } else { 0 },
                        OP_GREATERTHAN        => if n1 > n2 { 1 } else { 0 },
                        OP_LESSTHANOREQUAL    => if n1 <= n2 { 1 } else { 0 },
                        OP_GREATERTHANOREQUAL => if n1 >= n2 { 1 } else { 0 },
                        OP_MIN                => if n1 < n2 { n1 } else { n2 },
                        OP_MAX                => if n1 > n2 { n1 } else { n2 },
                        _ => { raise_script_error!("unexpected opcode"); 0 }
                     };
                     //println!("{}: {} {} -> {}", _info.name, n1, n2, tmp);
                     self.stack.pop()?;
                     self.stack.pop()?;
                     self.stack.push_value(tmp);
                     if op == OP_NUMEQUALVERIFY {
                        if self.stack.at(-1).unwrap().as_bool() {
                           self.stack.pop()?;
                        } else {
                           raise_script_interpret_error!(NumEqualVerify);
                        }
                     }
                  },            
                  
                  OP_WITHIN => {
                     if self.stack.len() < 3 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n1 = self.stack.at(-3)?.value(is_require_minimal, 4)?;
                     let n2 = self.stack.at(-2)?.value(is_require_minimal, 4)?;
                     let n3 = self.stack.at(-1)?.value(is_require_minimal, 4)?;
                     let b = (n2 <= n1) && (n1 < n3);
                     let _ = self.stack.pop()?;
                     let _ = self.stack.pop()?;
                     let _ = self.stack.pop()?;
                     if b {
                        self.stack.push_data(&[1u8, 1u8]);
                     } else {
                        self.stack.push_data(&[0u8]);
                     }
                  },
                  
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
                     let data = {
                        use ::crypto::{DigestExt, Ripemd160, Sha1, Sha256, Hash160, DHash256};
                        let e = self.stack.at(-1)?;
                        match op {
                           OP_RIPEMD160 => Ripemd160::_u8_to_box(e.data()),
                           OP_SHA1      => Sha1     ::_u8_to_box(e.data()),
                           OP_SHA256    => Sha256   ::_u8_to_box(e.data()),
                           OP_HASH160   => Hash160  ::_u8_to_box(e.data()),
                           OP_HASH256   => DHash256 ::_u8_to_box(e.data()),
                           _ => {
                              raise_script_error!("unexpected opcode");
                              Ripemd160::_u8_to_box(e.data()) //dummy
                           }
                        }
                     };
                     self.stack.pop()?;
                     self.stack.push_data(&data);
                  },
                  
                  OP_CODESEPARATOR => {
                  },
                  _ if op == OP_CHECKSIG || op == OP_CHECKSIGVERIFY => {
                     if self.stack.len() < 2 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let r = {
                        let sig = self.stack.at(-2)?;
                        let key = self.stack.at(-1)?;
                        let subscript = {
                           let tmp = &ctx.bytecode[ctx.codesep..];
                           if !ctx.flags.sig_version.is_base() {
                              tmp.to_vec()
                           } else {
                              Parser::find_and_delete(tmp, sig.data()).0
                           }
                        };

                        checker::check_signature_encoding(sig.data(), ctx.flags)?;
                        checker::check_pubkey_encoding(key.data(), ctx.flags)?;
                        let r = checker::chain_check_sign(ctx.tx, ctx.txin_idx, subscript.as_slice(), key.data(), sig.data());

                        if ctx.flags.script_verify.is_null_fail() && sig.data().len() != 0 {
                           match r {
                              Ok(false) => {
                                 raise_script_interpret_error!(SigNullFail, "verify failed");
                              },
                              Err(ref e) => {
                                 use std::error::Error;
                                 raise_script_interpret_error!(SigNullFail, e.description());
                              }
                              Ok(true) => (),
                           }
                        }
                        r.unwrap_or(false)
                     };
                     self.stack.pop()?;
                     self.stack.pop()?;
                     if op == OP_CHECKSIGVERIFY {
                        if !r { raise_script_interpret_error!(CheckSigVerify); }
                     } else {
                        self.stack.push_bool(r);
                     }
                  },

                  _ if op == OP_CHECKMULTISIG || op == OP_CHECKMULTISIGVERIFY => {
                     if self.stack.len() < 1 {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n_keys = self.stack.at(-1)?.as_i32(is_require_minimal, 4)?;
                     if n_keys < 0 || MAX_PUBKEYS_PER_MULTISIG < n_keys as usize {
                        raise_script_interpret_error!(PubkeyCount);
                     }
                     let n_keys = n_keys as usize;
                     ctx.op_count += n_keys;
                     if MAX_OPS_PER_SCRIPT < ctx.op_count {
                        raise_script_interpret_error!(OpCount);
                     }

                     if self.stack.len() < 2+n_keys {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }
                     let n_sigs = self.stack.at(-2-n_keys as isize)?.as_i32(is_require_minimal, 4)?;
                     if n_sigs < 0 || n_keys < n_sigs as usize {
                        raise_script_interpret_error!(SigCount);
                     }
                     let n_sigs = n_sigs as usize;
                     if self.stack.len() < 3+n_keys+n_sigs {
                        raise_script_interpret_error!(InvalidStackOperation);
                     }

                     let is_success = { //checksig
                        let len = self.stack.len();
                        let keys = &self.stack.as_slice()[(len-1-n_keys) .. (len-1)];
                        let sigs = &self.stack.as_slice()[(len-2-n_keys-n_sigs) .. (len-2-n_keys)];

                        let subscript = {
                           let tmp = &ctx.bytecode[ctx.codesep..];
                           if !ctx.flags.sig_version.is_base() {
                              tmp.to_vec()
                           } else {
                              sigs.iter().fold(tmp.to_vec(), |acc, sig| {
                                 Parser::find_and_delete(acc.as_slice(), sig.data()).0
                              })
                           }
                        };

                        let mut isig = sigs.len();
                        let mut ikey = keys.len();
                        while 0 < isig && isig <= ikey {
                           let sig = sigs[isig - 1].data();
                           let key = keys[ikey - 1].data();
                           //println!("checkmultisig: isig={}, ikey={}", isig, ikey);
                           checker::check_signature_encoding(sig, ctx.flags)?;
                           checker::check_pubkey_encoding(key, ctx.flags)?;
                           if checker::chain_check_sign(ctx.tx, ctx.txin_idx, subscript.as_slice(), key, sig).unwrap_or(false) {
                              //println!("  checkmultisig successeed: {}, {}", sig.len(), key.len());
                              isig -= 1;
                           }
                           ikey -= 1;
                        }
                        isig == 0
                     };

                     // clear stack
                     for _i in 0..(2+n_keys) { // not to check NULLFAIL
                        self.stack.pop()?;
                     }
                     for _i in 0..n_sigs {
                        if !is_success && ctx.flags.script_verify.is_null_fail() {
                           if self.stack.top()?.data().len() != 0 {
                              raise_script_interpret_error!(SigNullFail);
                           }
                        }
                        self.stack.pop()?;
                     }
                     if ctx.flags.script_verify.is_null_dummy() {
                        if self.stack.top()?.data().len() != 0 {
                           raise_script_interpret_error!(SigNullDummy);
                        }
                     }
                     self.stack.pop()?;

                     if op == OP_CHECKMULTISIGVERIFY {
                        if !is_success {
                           raise_script_interpret_error!(CheckMultisigVerify);
                        }
                     } else {
                        self.stack.push_bool(is_success);
                     }
                  },
                  
                  _ if op == OP_CHECKLOCKTIMEVERIFY || op == OP_CHECKSEQUENCEVERIFY => {
                     if (op == OP_CHECKLOCKTIMEVERIFY
                         && !ctx.flags.script_verify.is_check_locktime_verify()
                     ) || (op == OP_CHECKSEQUENCEVERIFY
                           && !ctx.flags.script_verify.is_check_sequence_verify()
                     ) {
                        if ctx.flags.script_verify.is_discourage_upgradable_nops() {
                           raise_script_interpret_error!(DiscourageUpgradableNops);
                        }
                     } else {
                        if self.stack.len() < 1 {
                           raise_script_interpret_error!(InvalidStackOperation);
                        }
                        // not to pop stack
                        let n = self.stack.at(-1)?.value(is_require_minimal, 5)?;
                        if n < 0 {
                           raise_script_interpret_error!(NegativeLocktime);
                        }
                        if op == OP_CHECKLOCKTIMEVERIFY {
                           if !checker::chain_check_locktime(&ctx.tx, ctx.txin_idx, n as u64)? {
                              raise_script_interpret_error!(UnsatisfiedLocktime);
                           }
                        } else {
                           let mut tmp = TxIn::default();
                           tmp.sequence = n as u32;
                           if !tmp.is_locktime_enable() {
                              ; // pass
                           } else {
                              if !checker::chain_check_sequence(&ctx.tx, ctx.txin_idx, n as u32)? {
                                 raise_script_interpret_error!(UnsatisfiedLocktime);
                              }
                           }
                        }
                     }
                  },
                  // OP_SMALLINTEGER => { raise_script_error!("not implemented yet"); },
                  // OP_PUBKEYS => { raise_script_error!("not implemented yet"); },
                  // OP_PUBKEYHASH => { raise_script_error!("not implemented yet"); },
                  // OP_PUBKEY => { raise_script_error!("not implemented yet"); },
                  _ => {
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
   if flags.script_verify.is_sig_push_only() {
      if !Parser::is_push_only(sigscr) {
         raise_script_interpret_error!(SigPushOnly);
      }
   }
   
   let mut interpreter = Interpreter::new();
   let _ = interpreter.eval(sigscr, tx, in_idx, flags)?;
   
   let p2sh = match flags.script_verify.is_p2sh() {
      true => Some(interpreter.clone()),
      false => None,
   };
   
   let _ = interpreter.eval(pkscr, tx, in_idx, flags)?;
   if interpreter.stack().len() < 1 {
      raise_script_interpret_error!(EvalFalse);
   }
   if ! interpreter.stack().at(-1)?.as_bool() {
      raise_script_interpret_error!(EvalFalse);
   }

   // witness
   if flags.script_verify.is_witness() {
      raise_script_error!("witness is not implemented yet");
   }

   if p2sh.is_some() && Parser::is_pay_to_script_hash(pkscr) {
      if !Parser::is_push_only(sigscr) {
         raise_script_interpret_error!(SigPushOnly);
      }
      interpreter = p2sh.unwrap(); //re-bind
      assert!(0 < interpreter.stack().len());
      let pkscr2 = interpreter.pop_stack().unwrap();
      let _ = interpreter.eval(pkscr2.data(), tx, in_idx, flags)?;
      if interpreter.stack().len() < 1 {
         raise_script_interpret_error!(EvalFalse);
      }
      if ! interpreter.stack().at(-1)?.as_bool() {
         raise_script_interpret_error!(EvalFalse);
      }
      if flags.script_verify.is_witness() {
         raise_script_error!("witness is not implemented yet");
      }
   }

   if flags.script_verify.is_clean_stack() {
      //assert!(flags.script_verify.is_p2sh());
      //assert!(flags.script_verify.is_witness());
      if interpreter.stack().len() != 1 {
         raise_script_interpret_error!(CleanStack);
      }
   }

   if flags.script_verify.is_witness() {
      //assert!(flags.script_verify.is_p2sh());
      raise_script_error!("witness is not implemented yet");
   }
   
   Ok(())
}
   

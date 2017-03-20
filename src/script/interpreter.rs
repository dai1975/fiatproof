use super::{Script, ScriptError};
use super::checksig::CheckSig;
use super::parser::Parsed;
use ::Transaction;

#[derive(Debug,Clone)]
pub enum Entry {
   Num(i64),
   Bytes(Box<[u8]>),
}

impl Entry {
   fn get_limit(limit:usize) -> Result<::std::ops::Range<i64>, ScriptError> {
      let r = match limit {
         4 => ::std::ops::Range { start:-0x7FFFFFFF,   end: 0x7FFFFFFF+1 },
         5 => ::std::ops::Range { start:-0x7FFFFFFFFF, end: 0x7FFFFFFFFF+1 },
         _ => script_error!(format!("unexpected limit: {}", limit)),
      };
      Ok(r)
   }

   pub fn as_bytes<F,T>(&self, f:F) -> T where F: Fn(&[u8]) -> T {
      match self {
         &Entry::Bytes(ref b) => {
            f(b.as_ref())
         },
         &Entry::Num(0) => {
            f(&[])
         }
         &Entry::Num(n) => {
            let mut tmp:Vec<u8> = Vec::new();
            let (neg, mut abs) = if n < 0 { (true, -n) } else { (false, n) };

            while 0 < abs {
               tmp.push((abs & 0xFF) as u8);
               abs >>= 8;
            }
            let len = tmp.len();
            if (tmp[len-1] & 0x80) != 0 {
               tmp.push(if neg { 0x80 } else { 0 });
            } else if neg {
               tmp[len-1] |= 0x80;
            }
            f(tmp.as_slice())
         }
      }
   }
   
   pub fn as_number(&self, require_minimal:bool, limit:usize) -> Result<i64, ScriptError> {
      match self {
         &Entry::Num(n) => {
            let range = try!(Entry::get_limit(limit));
            if !range.contains(n) {
               script_error!(format!("ScriptNum: value is exceeds limit: {} for {}", n, limit))
            }
            Ok(n)
         }
         &Entry::Bytes(ref bytes) => {
            let len = bytes.len();
            if limit < len {
               script_error!(format!("ScriptNum: bytes is longer than limit: {} for {}", len, limit))
            }
            if len == 0 {
               return Ok(0)
            }
            if require_minimal {
               if bytes[len-1] & 0x7F == 0 {
                  if len == 1 || bytes[len-1] & 0x7F == 0 {
                     script_error!(format!("ScriptNum: not a minimal bytes"))
                  }
               }
            }
            let v = {
               let (v,i) = bytes.iter().take(len-1).fold((0i64,0u8), |(v,i), &b| {
                  (v | (b as i64) << i, i+8)
               });
               if bytes[len-1] & 0x80 == 0 {
                  v | ((bytes[len-1] as i64) << i)
               } else {
                  -(v | ((bytes[len-1] & 0x7F) as i64) << i)
               }
            };
            Ok(v)
         }
      }
   }

   pub fn as_bool(&self) -> bool {
      match *self {
         Entry::Num(n) => {
            n != 0
         }
         Entry::Bytes(ref bytes) => {
            if bytes.iter().take(bytes.len()-1).any(|&b|{ b != 0x00 }) {
               return true;
            }
            match bytes[bytes.len()-1] {
               0x00 | 0x80 => false,
               _ => true,
            }
         }
      }
   }
}

impl PartialEq for Entry {
   fn eq(&self, rhs:&Entry) -> bool {
      match (self, rhs) {
         (&Entry::Num(l), &Entry::Num(r)) => {
            l == r
         },
         (&Entry::Bytes(ref l), &Entry::Bytes(ref r)) => {
            l == r
         },
         (&Entry::Bytes(ref l), &Entry::Num(_)) => {
            rhs.as_bytes(|bytes| bytes == l.as_ref())
         },
         (&Entry::Num(_), &Entry::Bytes(ref r)) => {
            self.as_bytes(|bytes| bytes == r.as_ref())
         },
      }
   }
}
impl Eq for Entry { }

#[derive(Debug,Clone)]
pub struct Stack {
   stack: Vec<Entry>,
}

impl Stack {
   pub fn new() -> Stack {
      Stack {
         stack:  Vec::new(),
      }
   }

   pub fn clear(&mut self) { self.stack.clear(); }
   pub fn len(&self) -> usize { self.stack.len() }
   
   pub fn result(&self) -> bool {
      match self.stack.last() {
         None => false,
         Some(e) => e.as_bool(),
      }
   }

   pub fn push_num(&mut self, n:i64)  {
      self.stack.push(Entry::Num(n))
   }
   pub fn push_bytes(&mut self, bytes:&[u8])  {
      self.stack.push(Entry::Bytes(bytes.to_vec().into_boxed_slice()))
   }

   pub fn push_bool(&mut self, v:bool) {
      let v = if v { 1 } else { 0 };
      self.stack.push(Entry::Num(v))
   }

   pub fn pop(&mut self) -> Result<Entry, ScriptError> {
      if self.stack.len() < 1 { script_error!("few stacks"); }
      Ok(self.stack.pop().unwrap())
   }
   pub fn dup(&mut self) -> Result<(), ScriptError> {
      if self.stack.len() < 1 { script_error!("few stacks"); }
      let dup = self.stack.last().unwrap().clone();
      self.stack.push(dup);
      Ok(())
   }
}

#[derive(Debug,Clone)]
pub struct Interpreter {
   stack: Stack,
   altstack: Stack,
}
struct Runtime<'a> {
   pub bytecode:   &'a [u8],
   pub checksig:   CheckSig<'a>,
   pub codesep:    usize,
   pub conditions: Vec<bool>,
   pub flags:      u32,
}

impl Interpreter {
   pub fn new() -> Self {
      Interpreter { stack: Stack::new(), altstack: Stack::new() }
   }
   pub fn with_stack(stack:Stack) -> Self {
      Interpreter { stack: stack, altstack: Stack::new() }
   }
   pub fn stack(&self) -> &Stack { &self.stack }

   pub fn eval(&mut self, script:&Script, tx:&Transaction, in_idx:usize, flags:u32) -> ::Result<bool> {
      use super::opcode::*;
      //println!("eval: {}", script);
      //let checker = signature::Checker::new(tx, in_idx);

      self.altstack.clear();
      let mut rt = Runtime {
         bytecode:   script.bytecode(),
         checksig:   CheckSig::new(tx, in_idx),
         codesep:    0,
         conditions: Vec::new(),
         flags:      flags
      };
      let mut last_op = OP_0;
      for parsed in script.parse() {
         let parsed = try!(parsed);
         if last_op == OP_CODESEPARATOR {
            rt.codesep = parsed.offset();
         }
         //let info = &OPCODE_INFO[code as usize];
         //println!("{:x}={}[{}]", code, info.name, follow.len());

         try!(self.step(&parsed, &mut rt));

         //for (i,v) in self.stack.iter().enumerate() {
         //   println!("  [{}] {:x}", i, ByteBuf(&v[..]));
         //}
         if 1000 < self.stack.len() + self.altstack.len() {
            script_error!("stack is too long");
         }
         last_op = parsed.opcode();
      }
      Ok(rt.conditions.len() == 0)
   }
   
   fn step(&mut self, parsed:&Parsed, rt:&mut Runtime) -> ::Result<()> {
      use super::opcode::*;
      //let sp = serialize::SerializeParam::new_net();
      let op = parsed.opcode();
      match op {
         OP_0 => {
            self.stack.push_num(0);
            Ok(())
         },
         OP_PUSHDATAFIX_01 ... OP_PUSHDATA4 => {
            self.stack.push_bytes(parsed.data());
            Ok(())
         },
         OP_1NEGATE => {
            self.stack.push_num(-1);
            Ok(())
         },
         OP_1 ... OP_16 => {
            self.stack.push_num((op - OP_1 + 1) as i64);
            Ok(())
         },
         OP_DUP => {
            let _ = try!(self.stack.dup());
            Ok(())
         },
         OP_HASH160 => {
            let e = try!(self.stack.pop());
            use ::crypto::{Hash160, Hasher};
            let hash = e.as_bytes(|bytes| Hash160::hash(bytes));
            self.stack.push_bytes(hash.as_ref());
            Ok(())
         },
         OP_EQUAL | OP_EQUALVERIFY => {
            let e1 = try!(self.stack.pop());
            let e2 = try!(self.stack.pop());
            let eq = e1 == e2;
            if op == OP_EQUAL {
               self.stack.push_bool(eq)
            } else {
               if !eq {
                  script_error!("equalverify");
               }
            }
            Ok(())
         }
         OP_CODESEPARATOR => {
            Ok(())
         },
         OP_CHECKSIG | OP_CHECKSIGVERIFY => {
            let pubkey   = try!(self.stack.pop());
            let signature= try!(self.stack.pop());
            let subscript = &rt.bytecode[rt.codesep..];
            let r = signature.as_bytes(|sig| {
               pubkey.as_bytes(|pk| {
                  rt.checksig.verify(subscript, pk, sig, rt.flags).is_ok()
               })
            });
            if op == OP_CHECKSIG {
               self.stack.push_bool(r);
            } else {
               if !r { script_error!("verify failed") }
            }
            Ok(())
         },
         _ => {
            let info = &OPCODE_INFO[op as usize];
            println!("  unimplemented {}(0x{:x})", info.name, op);
            Ok(())
         },
      }
   }
}


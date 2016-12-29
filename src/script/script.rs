use std;
use ::display;

#[derive(Debug,Clone,Default)]
pub struct Script {
   pub bytecode: Vec<u8>,
}

impl Script {
   pub fn len(&self) -> usize { self.bytecode.len() }
}

impl std::fmt::Display for Script {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "script({:x})", display::ByteSlice(&self.bytecode[..]))
   }
}


use std;
use ::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockLocator {
   pub haves: Vec<UInt256>,
}

impl std::fmt::Display for BlockLocator {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "BlockLocator(len={})", self.haves.len())
   }
}


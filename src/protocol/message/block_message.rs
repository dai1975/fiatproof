use std;
use ::Block;

#[derive(Debug,Default,Clone)]
pub struct BlockMessage {
   pub block: Block,
}

impl std::fmt::Display for BlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block({})", self.block)
   }
}


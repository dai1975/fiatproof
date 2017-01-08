use std;
use ::MerkleBlock;

#[derive(Debug,Default,Clone)]
pub struct MerkleBlockMessage {
   pub block : MerkleBlock,
}

impl std::fmt::Display for MerkleBlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(block={})", self.block)
   }
}


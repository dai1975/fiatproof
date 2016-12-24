use std;
use super::{BlockHeader, PartialMerkleTree};

#[derive(Debug,Default,Clone)]
pub struct MerkleBlock {
   header: BlockHeader,
   txn:    PartialMerkleTree,
}

impl std::fmt::Display for MerkleBlock {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(header={}, txn={})", self.header, self.txn)
   }
}


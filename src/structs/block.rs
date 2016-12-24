use std;
use super::{BlockHeader, Transaction};

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub transactions: Vec<Transaction>,
   pub checked: bool,
}

impl std::fmt::Display for Block {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block(header={}, tx={})", self.header, self.transactions.len())
   }
}


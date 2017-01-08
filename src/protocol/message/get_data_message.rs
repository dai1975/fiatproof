use std;
use ::UInt256;
use super::super::{ Inv, InvType };

#[derive(Debug,Default,Clone)]
pub struct GetDataMessage {
   pub invs : Vec<Inv>,
}

impl GetDataMessage {
   pub fn new(invtype:InvType, hash: UInt256) -> Self {
      GetDataMessage {
         invs: vec![ Inv::new(invtype, hash) ]
      }
   }
   pub fn new_tx(hash: UInt256)           -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)        -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filter_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}

impl std::fmt::Display for GetDataMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "GetData(len={})", self.invs.len()),
         1 => write!(f, "GetData(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "GetData(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}


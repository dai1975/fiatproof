use std;
use super::super::{ Inv };

#[derive(Debug,Default,Clone)]
pub struct InvMessage {
   pub invs : Vec<Inv>,
}

impl std::fmt::Display for InvMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "Inv(len={})", self.invs.len()),
         1 => write!(f, "Inv(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "Inv(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}



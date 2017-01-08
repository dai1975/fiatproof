use std;
use ::Transaction;

#[derive(Debug,Default)]
pub struct TxMessage {
   pub tx: Transaction,
}

impl std::fmt::Display for TxMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx({})", self.tx)
   }
}

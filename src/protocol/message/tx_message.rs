use std;
use ::Transaction;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default)]
pub struct TxMessage {
   pub tx: Transaction,
}
impl Message for TxMessage {
   fn get_command(&self) -> Command { command::TX }
}

impl std::fmt::Display for TxMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx({})", self.tx)
   }
}

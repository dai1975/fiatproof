use std;
use ::Transaction;
use super::message::{ Message, Command };

#[derive(Debug,Default)]
pub struct TxMessage {
   pub tx: Transaction,
}
impl Message for TxMessage {
   const COMMAND: Command = Command { data: &[0x74, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for TxMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx({})", self.tx)
   }
}

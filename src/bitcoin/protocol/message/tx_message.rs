use std;
use ::bitcoin::datatypes::Tx;

#[derive(Debug,Default)]
pub struct TxMessage {
   pub tx: Tx,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for TxMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x74, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for TxMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx({})", self.tx)
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for TxMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.tx.encode(e));
      Ok(r)
   }
}
impl BitcoinDecodee for TxMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.tx.decode(d));
      Ok(r)
   }
}

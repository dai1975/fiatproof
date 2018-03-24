use std;
use ::UInt256;
use super::super::{ Inv, InvType };

#[derive(Debug,Default,Clone)]
pub struct GetDataMessage {
   pub invs : Vec<Inv>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetDataMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00];
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

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for GetDataMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::apriori::MAX_INV_SIZE;
      r += try!(e.encode_var_array(&self.invs[..], MAX_INV_SIZE));
      Ok(r)
   }
}
impl BitcoinDecodee for GetDataMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::apriori::MAX_INV_SIZE;
      r += try!(d.decode_var_array(&mut self.invs, MAX_INV_SIZE));
      Ok(r)
   }
}

use std;
use ::UInt256;
use ::script::{Script};

pub type Amount = i64;

#[allow(dead_code)] const COIN:Amount = 100000000;
#[allow(dead_code)] const CENT:Amount = 1000000;
#[allow(dead_code)] const MAX_MONEY:Amount = 21000000 * COIN;

#[derive(Debug,Default,Clone,Eq,PartialEq,PartialOrd,Ord)]
pub struct OutPoint {
   pub hash: UInt256,
   pub n:    u32,
}

const COINBASE_OUT_POINT:OutPoint = OutPoint { hash: ::uint256::ZERO, n: std::u32::MAX };

impl OutPoint {
   pub fn set_null(&mut self)    { *self  = COINBASE_OUT_POINT; }
   pub fn is_null(&self) -> bool { self == &COINBASE_OUT_POINT }
}

#[derive(Debug,Default,Clone)]
pub struct TxIn {
   pub prevout:    OutPoint,
   pub script_sig: Script,
   pub sequence:   u32,
}

#[derive(Debug,Default,Clone)]
pub struct TxOut {
   pub value:         Amount,
   pub script_pubkey: Script,
}

#[derive(Debug,Default,Clone)]
pub struct Transaction {
   pub version:  i32,
   pub ins:      Vec<TxIn>,
   pub outs:     Vec<TxOut>,
   pub locktime: u32,
}

impl TxOut {
   pub fn new() -> TxOut {
      TxOut { //eq to set_null
         value: -1,
         script_pubkey: Script{ bytecode: vec!() },
      }
   }
   pub fn set_null(&mut self) {
      self.value = -1;
      self.script_pubkey.bytecode.clear();
   }
}

impl Transaction {
   pub fn is_coin_base(&self) -> bool {
      self.ins.len() == 1 && self.ins[0].prevout.is_null()
   }
}


impl std::fmt::Display for OutPoint {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "OutPoint(hash={}, n={})", self.hash, self.n)
   }
}
impl std::fmt::Display for TxIn {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "TxIn(prevout={}, sig={}, seq={})", self.prevout, self.script_sig, self.sequence)
   }
}
impl std::fmt::Display for TxOut {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "TxOut(val={}, pubkey={})", self.value, self.script_pubkey)
   }
}
impl std::fmt::Display for Transaction {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx(ver={}, ins={}, outs={}, locktime={})", self.version, self.ins.len(), self.outs.len(), self.locktime)
   }
}


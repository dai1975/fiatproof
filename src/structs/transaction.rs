use std;
use ::UInt256;
use ::script::{Script};

pub type Amount = i64;

#[allow(dead_code)] const COIN:Amount = 100000000;
#[allow(dead_code)] const CENT:Amount = 1000000;
#[allow(dead_code)] const MAX_MONEY:Amount = 21000000 * COIN;

#[derive(Debug,Default,Clone,Eq,PartialEq,PartialOrd,Ord)]
pub struct OutPoint {
   pub txid: UInt256,
   pub n:    u32,
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

#[derive(Debug,PartialEq,Clone)]
pub enum LockTime {
   NoLock,
   Block(u32),
   Time(std::time::SystemTime),
}
impl Default for LockTime {
   fn default() -> Self { LockTime::NoLock }
}

#[derive(Debug,Default,Clone)]
pub struct Transaction {
   pub version:  i32,
   pub ins:      Vec<TxIn>,
   pub outs:     Vec<TxOut>,
   pub locktime: LockTime,
}

const COINBASE_OUT_POINT:OutPoint = OutPoint { txid: ::uint256::ZERO, n: std::u32::MAX };
impl OutPoint {
   pub fn new_null() -> Self { COINBASE_OUT_POINT }
   pub fn set_null(&mut self)    { *self  = COINBASE_OUT_POINT; }
   pub fn is_null(&self) -> bool { self == &COINBASE_OUT_POINT }
}

const SEQUENCE_FINAL:u32                 = 0xFFFFFFFFu32;
const SEQUENCE_LOCKTIME_DISABLE_FLAG:u32 = (1 << 31);
const SEQUENCE_LOCKTIME_TYPE_FLAG:u32    = (1 << 22);
const SEQUENCE_LOCKTIME_MASK:u32         = 0x0000FFFFu32;
const SEQUENCE_GRANULARITY:u32           = 9;
impl TxIn {
   pub fn new() -> Self {
      TxIn { //eq to set_null
         prevout:    OutPoint::new_null(),
         script_sig: Script{ bytecode: vec!() },
         sequence:   SEQUENCE_FINAL,
      }
   }
   pub fn is_locktime_enable(&self) -> bool {
      (self.sequence & SEQUENCE_LOCKTIME_DISABLE_FLAG) == 0
   }
   pub fn is_locktime_type(&self) -> bool {
      (self.sequence & SEQUENCE_LOCKTIME_TYPE_FLAG) != 0
   }

   // bitcoin-core では mask,shift した後に -1 して、-1 との max を取っているが、mask,shift 時点で負にならないよな?
   pub fn get_locktime_time(&self) -> Option<u64> {
      let v:u64 = ((self.sequence & SEQUENCE_LOCKTIME_MASK) as u64) << SEQUENCE_GRANULARITY;
      if v == 0 { None } else { Some(v-1) }
   }
   pub fn get_locktime_height(&self) -> Option<u32> {
      let v:u32 = self.sequence & SEQUENCE_LOCKTIME_MASK;
      if v == 0 { None } else { Some(v-1) }
   }
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

const TRANSACTION_CURRENT_VERSION:i32 = 1i32;

impl Transaction {
   pub fn new() -> Self {
      Transaction {
         version:  TRANSACTION_CURRENT_VERSION,
         ins:      Vec::<TxIn>::new(), 
         outs:     Vec::<TxOut>::new(), 
         locktime: LockTime::NoLock,
      }
   }
   pub fn is_coin_base(&self) -> bool {
      self.ins.len() == 1 && self.ins[0].prevout.is_null()
   }
}


impl std::fmt::Display for OutPoint {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "OutPoint(txid={}, n={})", self.txid, self.n)
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
      write!(f, "Tx(ver={}, ins={}, outs={}, locktime={:?})", self.version, self.ins.len(), self.outs.len(), self.locktime)
   }
}


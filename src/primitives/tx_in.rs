use super::{UInt256, Script};

#[derive(Debug,Default,Clone,Eq,PartialEq,PartialOrd,Ord)]
pub struct TxOutPoint {
   pub txid: UInt256,
   pub n:    u32,
}

#[derive(Debug,Default,Clone)]
pub struct TxIn {
   pub prevout:    TxOutPoint,
   pub script_sig: Script,
   pub sequence:   u32,
}

const COINBASE_OUT_POINT:TxOutPoint = TxOutPoint {
   txid: UInt256::new_zero(),
   n:    ::std::u32::MAX,
};
impl TxOutPoint {
   pub fn new_null()     -> Self { COINBASE_OUT_POINT }
   pub fn is_null(&self) -> bool { self == &COINBASE_OUT_POINT }
   pub fn set_null(&mut self)    { *self = COINBASE_OUT_POINT; }
}

// Sequence 型を作るべきか
const SEQUENCE_FINAL:u32                 = 0xFFFFFFFFu32;
const SEQUENCE_LOCKTIME_DISABLE_FLAG:u32 = (1 << 31);
const SEQUENCE_LOCKTIME_TYPE_FLAG:u32    = (1 << 22);
const SEQUENCE_LOCKTIME_MASK:u32         = 0x0000FFFFu32;
const SEQUENCE_GRANULARITY:u32           = 9;
impl TxIn {
   pub fn new() -> Self {
      TxIn { //eq to set_null
         prevout:    TxOutPoint::new_null(),
         script_sig: Script::default(),
         sequence:   SEQUENCE_FINAL,
      }
   }
   pub fn is_sequence_final(&self) -> bool {
      self.sequence == SEQUENCE_FINAL
   }
   pub fn is_locktime_enable(&self) -> bool {
      (self.sequence & SEQUENCE_LOCKTIME_DISABLE_FLAG) == 0
   }
   pub fn is_locktime_type(&self) -> bool {
      (self.sequence & SEQUENCE_LOCKTIME_TYPE_FLAG) != 0
   }
   pub fn compare_sequence_locktime(l:u32, r:u32) -> Option<bool> {
      let l_is_blocktime = (l & SEQUENCE_LOCKTIME_TYPE_FLAG) != 0;
      let r_is_blocktime = (r & SEQUENCE_LOCKTIME_TYPE_FLAG) != 0;
      if l_is_blocktime ^ r_is_blocktime {
         None
      } else {
         Some((l & SEQUENCE_LOCKTIME_MASK) > (r & SEQUENCE_LOCKTIME_MASK))
      }
   }

   pub fn get_locktime_time(&self) -> Option<u64> {
      let v:u64 = ((self.sequence & SEQUENCE_LOCKTIME_MASK) as u64) << SEQUENCE_GRANULARITY;
      if v == 0 { None } else { Some(v-1) }
   }
   pub fn get_locktime_height(&self) -> Option<u32> {
      let v:u32 = self.sequence & SEQUENCE_LOCKTIME_MASK;
      if v == 0 { None } else { Some(v-1) }
   }
}

impl ::std::fmt::Display for TxOutPoint {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "OutPoint(txid={}, n={})", self.txid, self.n)
   }
}
impl ::std::fmt::Display for TxIn {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "TxIn(prevout={}, sig={}, seq={})", self.prevout, self.script_sig, self.sequence)
   }
}


use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for TxOutPoint {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.txid.encode(e));
      r += try!(e.encode_u32le(self.n));
      Ok(r)
   }
}
impl BitcoinDecodee for TxOutPoint {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.txid.decode(d));
      r += try!(d.decode_u32le(&mut self.n));
      Ok(r)
   }
}

impl BitcoinEncodee for TxIn {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.prevout.encode(e));
      r += try!(self.script_sig.encode(e));
      r += try!(e.encode_u32le(self.sequence));
      Ok(r)
   }
}
impl BitcoinDecodee for TxIn {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.prevout.decode(d));
      r += try!(self.script_sig.decode(d));
      r += try!(d.decode_u32le(&mut self.sequence));
      Ok(r)
   }
}


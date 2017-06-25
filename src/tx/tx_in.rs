use ::UInt256;
use ::script::{Script};

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

const COINBASE_OUT_POINT:OutPoint = OutPoint { txid: ::uint256::ZERO, n: ::std::u32::MAX };
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

   pub fn get_locktime_time(&self) -> Option<u64> {
      let v:u64 = ((self.sequence & SEQUENCE_LOCKTIME_MASK) as u64) << SEQUENCE_GRANULARITY;
      if v == 0 { None } else { Some(v-1) }
   }
   pub fn get_locktime_height(&self) -> Option<u32> {
      let v:u32 = self.sequence & SEQUENCE_LOCKTIME_MASK;
      if v == 0 { None } else { Some(v-1) }
   }
}

impl ::std::fmt::Display for OutPoint {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "OutPoint(txid={}, n={})", self.txid, self.n)
   }
}
impl ::std::fmt::Display for TxIn {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "TxIn(prevout={}, sig={}, seq={})", self.prevout, self.script_sig, self.sequence)
   }
}


use ::std::borrow::Borrow;
use ::serialize::{Encoder, Encodee, Decoder, Decodee};
impl Encodee for OutPoint {
   fn encode(&self, e:&mut Encoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.txid.encode(e));
      r += try!(e.encode_u32le(self.n));
      Ok(r)
   }
}
impl Decodee for OutPoint {
   fn decode(&mut self, d:&mut Decoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.txid.decode(d, ()));
      r += try!(d.decode_u32le(&mut self.n));
      Ok(r)
   }
}

impl Encodee for TxIn {
   fn encode(&self, e:&mut Encoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.prevout.encode(e));
      {
         use ::serialize::bitcoin::to_bytes;
         let tmp = try!(to_bytes(&self.script_sig));
         r += try!(e.encode_sequence_u8(tmp, None));
      }
      r += try!(e.encode_u32le(self.sequence));
      Ok(r)
   }
}
impl Decodee for TxIn {
   fn decode(&mut self, d:&mut Decoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.prevout.decode(d));
      {
         use ::serialize::bitcoin::from_bytes;
         let mut tmp:Vec<u8> = Vec::new();
         r += try!(d.decode_sequence_u8(&mut tmp));
         self.script_sig = try!(from_bytes(&tmp));
      }
      r += try!(d.decode_u32le(&mut self.sequence));
      Ok(r)
   }
}

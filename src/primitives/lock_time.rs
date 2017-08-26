use ::std::time::SystemTime;

#[derive(Debug,PartialEq,Clone)]
pub enum LockTime {
   NoLock,
   Block(u32),
   Time(SystemTime),
}

impl Default for LockTime {
   fn default() -> Self { LockTime::NoLock }
}

impl LockTime {
   pub fn is_no_lock(&self) -> bool {
      match self {
         &LockTime::NoLock => true,
         _ => false,
      }
   }
   pub fn get_block(&self) -> Option<u32> {
      match self {
         &LockTime::Block(v) => Some(v),
         _ => None,
      }
   }
   pub fn get_time(&self) -> Option<SystemTime> {
      match self {
         &LockTime::Time(t) => Some(t),
         _ => None,
      }
   }
}


use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
const TRANSACTION_LOCKTIME_BORDER:u32  = 500000000u32;
impl BitcoinEncodee for LockTime {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      let locktime:u32 = match self {
         &LockTime::NoLock      => 0u32,
         &LockTime::Block(v)    => {
            if TRANSACTION_LOCKTIME_BORDER <= v { encode_error!("locktime is too big block number") }
            v
         }
         &LockTime::Time(t) => {
            use std::time::UNIX_EPOCH;
            let t = match t.duration_since(UNIX_EPOCH) {
               Ok(d)  => d.as_secs(),
               Err(_) => encode_error!("the timestamp is earler than epoch"),
            };
            if t < (TRANSACTION_LOCKTIME_BORDER as u64) { 
               encode_error!("the timestamp is earler than locktime border");
            }
            t as u32 //note: maximum u32 unixtime is 2106-02-07T06:28:15+00:00 (ignores leap time)
         }
      };
      r += try!(e.encode_u32le(locktime));
      Ok(r)
   }
}
impl BitcoinDecodee for LockTime {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut locktime:u32 = 0;
      r += try!(d.decode_u32le(&mut locktime));
      *self = if locktime == 0 {
         LockTime::NoLock
      } else if locktime < TRANSACTION_LOCKTIME_BORDER {
         LockTime::Block(locktime)
      } else {
         use std::time::{UNIX_EPOCH, Duration};
         LockTime::Time(UNIX_EPOCH + Duration::from_secs(locktime as u64))
      };
      Ok(r)
   }
}

use std::time::SystemTime;

const TRANSACTION_LOCKTIME_BORDER:u32  = 500000000u32;

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
   pub fn new_by_u64(v:u64) -> Self {
      if v == 0 {
         LockTime::NoLock
      } else if v < (TRANSACTION_LOCKTIME_BORDER as u64) {
         LockTime::Block(v as u32)
      } else {
         use std::time::{UNIX_EPOCH, Duration};
         LockTime::Time(UNIX_EPOCH + Duration::from_secs(v))
      }
   }
}


use crate::iostream::{ WriteStream, ReadStream };
use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for LockTime {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      let locktime:u32 = match self {
         &LockTime::NoLock      => 0u32,
         &LockTime::Block(v)    => {
            if TRANSACTION_LOCKTIME_BORDER <= v { raise_serialize_error!("locktime is too big block number") }
            v
         }
         &LockTime::Time(t) => {
            use std::time::UNIX_EPOCH;
            let t = match t.duration_since(UNIX_EPOCH) {
               Ok(d)  => d.as_secs(),
               Err(_) => raise_serialize_error!("the timestamp is earler than epoch"),
            };
            if t < (TRANSACTION_LOCKTIME_BORDER as u64) { 
               raise_serialize_error!("the timestamp is earler than locktime border");
            }
            t as u32 //note: maximum u32 unixtime is 2106-02-07T06:28:15+00:00 (ignores leap time)
         }
      };
      r += e.serialize_u32le(ws, locktime)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for LockTime {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      let mut locktime:u32 = 0;
      r += d.deserialize_u32le(rs, &mut locktime)?;
      *self = LockTime::new_by_u64(locktime as u64);
      Ok(r)
   }
}

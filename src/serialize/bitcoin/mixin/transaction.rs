use std;
use ::{Error};
use super::super::super::{Encoder, WriteStream, SerializeError};
use super::super::{BitcoinEncoder, BitcoinEncodee};

pub use ::structs::transaction::{ OutPoint, TxIn, TxOut, Transaction, LockTime };

macro_rules! serialize_error {
   ($msg:expr) => {
      try!( Err(SerializeError::new($msg.to_string())) )
   }
}

impl BitcoinEncodee for OutPoint {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_uint256(&self.txid, w, ep));
      r += try!(e.encode_u32le(self.n, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for TxIn {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.prevout, &(), w, ep));
      r += try!(e.encode(&self.script_sig, &(), w, ep));
      r += try!(e.encode_u32le(self.sequence, w, ep));
      Ok(r)
   }
}

impl BitcoinEncodee for TxOut {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i64le(self.value, w, ep));
      r += try!(e.encode(&self.script_pubkey, &(), w, ep));
      Ok(r)
   }
}

const TRANSACTION_LOCKTIME_BORDER:u32  = 500000000u32;
impl BitcoinEncodee for Transaction {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version, w, ep));
      r += try!(e.encode_sequence(&self.ins, &(), w, ep));
      r += try!(e.encode_sequence(&self.outs, &(), w, ep));
      let locktime:u32 = match self.locktime {
         LockTime::NoLock      => 0u32,
         LockTime::Block(v)    => {
            if TRANSACTION_LOCKTIME_BORDER <= v { serialize_error!("locktime is too big block number") }
            v
         }
         LockTime::Time(t) => {
            let t = match t.duration_since(std::time::UNIX_EPOCH) {
               Ok(d)  => d.as_secs(),
               Err(_) => serialize_error!("the timestamp is earler than epoch"),
            };
            if t < (TRANSACTION_LOCKTIME_BORDER as u64) { 
               serialize_error!("the timestamp is earler than locktime border");
            }
            t as u32
         }
      };
      r += try!(e.encode_u32le(locktime, w, ep));
      Ok(r)
   }
}

#[test]
fn test_transaction() {
   /*
   use ::protocol::{NetworkAddress, NODE_FULL};
   use ::serialize::{FixedBitcoinSerializer, BitcoinEncodeParam};
   use std::net::SocketAddr;
   use std::str::FromStr;

   let mut _tx = Transaction::new();
*/
}

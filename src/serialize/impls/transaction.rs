use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinSerializer, WriteStream, SerializeError};
pub use ::structs::transaction::{ OutPoint, TxIn, TxOut, Transaction, LockTime };

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for OutPoint {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.txid));
      r += try!(e.encode_u32le(self.n));
      Ok(r)
   }
}

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for TxIn {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.prevout));
      r += try!(e.encode(&self.script_sig));
      r += try!(e.encode_u32le(self.sequence));
      Ok(r)
   }
}

impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for TxOut {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i64le(self.value));
      r += try!(e.encode(&self.script_pubkey));
      Ok(r)
   }
}

const TRANSACTION_LOCKTIME_BORDER:u32  = 500000000u32;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for Transaction {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(e.encode_sequence(&self.ins));
      r += try!(e.encode_sequence(&self.outs));
      let locktime:u32 = match self.locktime {
         LockTime::NoLock      => 0u32,
         LockTime::Block(v)    => {
            if TRANSACTION_LOCKTIME_BORDER <= v { serialize_error!("locktime is too big block number") }
            v
         }
         LockTime::Time(t) => {
            use std::time::UNIX_EPOCH;
            let t = match t.duration_since(UNIX_EPOCH) {
               Ok(d)  => d.as_secs(),
               Err(_) => serialize_error!("the timestamp is earler than epoch"),
            };
            if t < (TRANSACTION_LOCKTIME_BORDER as u64) { 
               serialize_error!("the timestamp is earler than locktime border");
            }
            t as u32
         }
      };
      r += try!(e.encode_u32le(locktime));
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

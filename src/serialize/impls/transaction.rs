use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinDecoder, BitcoinDecodee, SerializeError};
use ::structs::transaction::{ OutPoint, TxIn, TxOut, Transaction, LockTime };

impl <E:BitcoinEncoder> BitcoinEncodee<E> for OutPoint {
   fn encode(&self, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.txid));
      r += try!(e.encode_u32le(self.n));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D> for OutPoint {
   fn decode(&mut self, e:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.decode(&mut self.txid));
      r += try!(e.decode_u32le(&mut self.n));
      Ok(r)
   }
}

impl <E:BitcoinEncoder> BitcoinEncodee<E> for TxIn {
   fn encode(&self, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.prevout));
      r += try!(e.encode(&self.script_sig));
      r += try!(e.encode_u32le(self.sequence));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D> for TxIn {
   fn decode(&mut self, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode(&mut self.prevout));
      r += try!(d.decode(&mut self.script_sig));
      r += try!(d.decode_u32le(&mut self.sequence));
      Ok(r)
   }
}

impl <E:BitcoinEncoder> BitcoinEncodee<E> for TxOut {
   fn encode(&self, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i64le(self.value));
      r += try!(e.encode(&self.script_pubkey));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D> for TxOut {
   fn decode(&mut self, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_i64le(&mut self.value));
      r += try!(d.decode(&mut self.script_pubkey));
      Ok(r)
   }
}

const TRANSACTION_LOCKTIME_BORDER:u32  = 500000000u32;
impl <E:BitcoinEncoder> BitcoinEncodee<E> for LockTime {
   fn encode(&self, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      let locktime:u32 = match self {
         &LockTime::NoLock      => 0u32,
         &LockTime::Block(v)    => {
            if TRANSACTION_LOCKTIME_BORDER <= v { serialize_error!("locktime is too big block number") }
            v
         }
         &LockTime::Time(t) => {
            use std::time::UNIX_EPOCH;
            let t = match t.duration_since(UNIX_EPOCH) {
               Ok(d)  => d.as_secs(),
               Err(_) => serialize_error!("the timestamp is earler than epoch"),
            };
            if t < (TRANSACTION_LOCKTIME_BORDER as u64) { 
               serialize_error!("the timestamp is earler than locktime border");
            }
            t as u32 //note: maximum u32 unixtime is 2106-02-07T06:28:15+00:00 (ignores leap time)
         }
      };
      r += try!(e.encode_u32le(locktime));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D> for LockTime {
   fn decode(&mut self, d:&mut D) -> Result<usize, Error> {
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

impl <E:BitcoinEncoder> BitcoinEncodee<E> for Transaction {
   fn encode(&self, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(e.encode_sequence(&self.ins));
      r += try!(e.encode_sequence(&self.outs));
      r += try!(e.encode(&self.locktime));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D> for Transaction {
   fn decode(&mut self, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(d.decode_sequence(&mut self.ins));
      r += try!(d.decode_sequence(&mut self.outs));
      r += try!(d.decode(&mut self.locktime));
      Ok(r)
   }
}

use std;
use ::{ToBytes, FromBytes, ToHash};
use super::super::{BitcoinSerializer, BitcoinDeserializer};
impl_to_bytes_for_encodee!{Transaction, 1000}
impl_to_hash_for_encodee!{Transaction, 1000}
impl_from_bytes_for_decodee!{Transaction}

#[test]
fn test_decode_transaction() {
   use ::{WithHex, WithBytes, ToHex};
   let bytes = Vec::<u8>::with_hex("0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000").unwrap();

   let tx = Transaction::with_bytes(bytes);
   assert_matches!(tx, Ok(_));
   let tx = tx.unwrap();
   assert_eq!(tx.version, 1);
   assert_eq!(tx.ins.len(), 2);
   assert_eq!(tx.outs.len(), 2);
   assert_eq!(tx.locktime.is_no_lock(), true);

   assert_eq!(tx.ins[0].prevout.txid.to_hex().unwrap(), "4d6da9420d472b6b52c36eee132d87448bf160d8839a58afdd2add6f6adfc8d8");
   assert_eq!(tx.ins[0].prevout.n, 0);
   assert_eq!(tx.ins[0].is_sequence_final(), true);
   assert_eq!(tx.ins[0].script_sig.bytecode.to_hex().unwrap(), "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab");

   assert_eq!(tx.ins[1].prevout.txid.to_hex().unwrap(), "b220a19ba645c021f0cf501435fd6c3b4db960325d0834612612a5684ffab32a");
   assert_eq!(tx.ins[1].prevout.n, 0);
   assert_eq!(tx.ins[1].is_sequence_final(), true);
   assert_eq!(tx.ins[1].script_sig.bytecode.to_hex().unwrap(), "4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226");

   assert_eq!(tx.outs[0].value, 39000000);
   assert_eq!(tx.outs[0].script_pubkey.bytecode.to_hex().unwrap(), "76a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788ac");
   assert_eq!(tx.outs[1].value, 155000000);
   assert_eq!(tx.outs[1].script_pubkey.bytecode.to_hex().unwrap(), "76a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac");
}

#[test]
fn test_encode_transaction() {
   use ::script::Script;
   use ::{WithHex, ToHash, ToHex};
   let mut tx = Transaction::new();
   tx.ins.push(TxIn {
      prevout: OutPoint { txid: ::UInt256::with_hex("4d6da9420d472b6b52c36eee132d87448bf160d8839a58afdd2add6f6adfc8d8").unwrap(), n:0 },
      script_sig: Script { bytecode: Vec::<u8>::with_hex("483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab").unwrap() },
      sequence: 0xFFFFFFFFu32,
   } );
   tx.ins.push(TxIn {
      prevout: OutPoint { txid: ::UInt256::with_hex("b220a19ba645c021f0cf501435fd6c3b4db960325d0834612612a5684ffab32a").unwrap(), n:0 },
      script_sig: Script { bytecode: Vec::<u8>::with_hex("4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226").unwrap() },
      sequence: 0xFFFFFFFFu32,
   } );
   tx.outs.push(TxOut {
      value: 39000000,
      script_pubkey: Script { bytecode: Vec::<u8>::with_hex("76a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788ac").unwrap() },
   });
   tx.outs.push(TxOut {
      value: 155000000,
      script_pubkey: Script { bytecode: Vec::<u8>::with_hex("76a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac").unwrap() },
   });

   assert_eq!(tx.to_hex().unwrap(), "0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000");
   
   assert_eq!(tx.to_dhash256_reverse_hex().unwrap(), "9021b49d445c719106c95d561b9c3fac7bcb3650db67684a9226cd7fa1e1c1a0");
}
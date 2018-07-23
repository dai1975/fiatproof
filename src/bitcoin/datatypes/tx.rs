use super::{UInt256, TxIn, TxOut, LockTime};

#[derive(Debug,Default,Clone)]
pub struct Tx {
   pub version:  i32,
   pub ins:      Vec<TxIn>,
   pub outs:     Vec<TxOut>,
   pub locktime: LockTime,
}

const TRANSACTION_CURRENT_VERSION:i32 = 1i32;

impl Tx {
   pub fn new_null() -> Self {
      Tx {
         version:  TRANSACTION_CURRENT_VERSION,
         ins:      Vec::<TxIn>::new(), 
         outs:     Vec::<TxOut>::new(), 
         locktime: LockTime::NoLock,
      }
   }
   pub fn is_coin_base(&self) -> bool {
      self.ins.len() == 1 && self.ins[0].prevout.is_null()
   }
   pub fn is_null(&self) -> bool {
      self.ins.len() == 0 && self.outs.len() == 0
   }
   pub fn get_hash(&self) -> UInt256 {
      use ::serialize::ToDigest;
      UInt256::new(&self.to_dhash256("hash").unwrap())
   }
}

impl ::std::fmt::Display for Tx {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "Tx(ver={}, ins={}, outs={}, locktime={:?})", self.version, self.ins.len(), self.outs.len(), self.locktime)
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for Tx {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(e.encode_var_array(self.ins.as_slice(), ::std::usize::MAX));
      r += try!(e.encode_var_array(self.outs.as_slice(), ::std::usize::MAX));
      r += try!(self.locktime.encode(e));
      Ok(r)
   }
}
impl BitcoinDecodee for Tx {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(d.decode_var_array(&mut self.ins, ::std::usize::MAX));
      r += try!(d.decode_var_array(&mut self.outs, ::std::usize::MAX));
      r += try!(self.locktime.decode(d));
      Ok(r)
   }
}

#[test]
fn test_decode_transaction() {
   use super::Tx;
   use ::serialize::{FromOctets, ToOctets};
   let hex = "0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000";

   let tx = Tx::from_hex_string(hex, "");
   assert_matches!(tx, Ok(_));
   let tx = tx.unwrap();
   assert_eq!(tx.version, 1);
   assert_eq!(tx.ins.len(), 2);
   assert_eq!(tx.outs.len(), 2);
   assert_eq!(tx.locktime.is_no_lock(), true);

   assert_eq!(tx.ins[0].prevout.txid.to_hex_string_rev("").unwrap(), "4d6da9420d472b6b52c36eee132d87448bf160d8839a58afdd2add6f6adfc8d8");
   assert_eq!(tx.ins[0].prevout.n, 0);
   assert_eq!(tx.ins[0].is_sequence_final(), true);
   assert_eq!(tx.ins[0].script_sig.to_hex_string("unsized").unwrap(), "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab");

   assert_eq!(tx.ins[1].prevout.txid.to_hex_string_rev("").unwrap(), "b220a19ba645c021f0cf501435fd6c3b4db960325d0834612612a5684ffab32a");
   assert_eq!(tx.ins[1].prevout.n, 0);
   assert_eq!(tx.ins[1].is_sequence_final(), true);
   assert_eq!(tx.ins[1].script_sig.to_hex_string("unsized").unwrap(), "4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226");

   assert_eq!(tx.outs[0].value, 39000000);
   assert_eq!(tx.outs[0].script_pubkey.to_hex_string("unsized").unwrap(), "76a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788ac");
   assert_eq!(tx.outs[1].value, 155000000);
   assert_eq!(tx.outs[1].script_pubkey.to_hex_string("unsized").unwrap(), "76a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac");
}

#[test]
fn test_encode_transaction() {
   use super::{UInt256, Script, TxIn, TxOutPoint};
   use ::serialize::{FromOctets, ToOctets, ToDigest};

   let mut tx = Tx::new_null();
   tx.ins.push(TxIn {
      prevout: TxOutPoint {
         txid: UInt256::from_hex_string_rev("4d6da9420d472b6b52c36eee132d87448bf160d8839a58afdd2add6f6adfc8d8", "").unwrap(),
         n:0
      },
      script_sig: Script::from_hex_string("483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab", "unsized").unwrap(),
      sequence: 0xFFFFFFFFu32,
   } );
   tx.ins.push(TxIn {
      prevout: TxOutPoint {
         txid: UInt256::from_hex_string_rev("b220a19ba645c021f0cf501435fd6c3b4db960325d0834612612a5684ffab32a", "").unwrap(),
         n:0
      },
      script_sig: Script::from_hex_string("4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226", "unsized").unwrap(),
      sequence: 0xFFFFFFFFu32,
   } );
   tx.outs.push(TxOut {
      value: 39000000,
      script_pubkey: Script::from_hex_string("76a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788ac", "unsized").unwrap(),
   });
   tx.outs.push(TxOut {
      value: 155000000,
      script_pubkey: Script::from_hex_string("76a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac", "unsized").unwrap(),
   });

   assert_eq!(tx.to_hex_string("").unwrap(), "0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000");
   
   assert_eq!(tx.to_dhash256_hex_string_rev("").unwrap(), "9021b49d445c719106c95d561b9c3fac7bcb3650db67684a9226cd7fa1e1c1a0");
}

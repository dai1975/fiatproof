use super::{Script};

pub type Amount = i64;

#[allow(dead_code)] const COIN:Amount = 100000000;
#[allow(dead_code)] const CENT:Amount = 1000000;
#[allow(dead_code)] const MAX_MONEY:Amount = 21000000 * COIN;

#[derive(Debug,Default,Clone)]
pub struct TxOut {
   pub value:         Amount,
   pub script_pubkey: Script,
}

impl TxOut {
   pub fn new_null() -> TxOut {
      TxOut {
         value: -1,
         script_pubkey: Script::new_null(),
      }
   }
   pub fn set_null(&mut self) {
      self.value = -1;
      self.script_pubkey.set_null();
   }
   pub fn is_null(&self) -> bool {
      self.value == -1
   }
}

impl ::std::fmt::Display for TxOut {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "TxOut(val={}, pubkey={})", self.value, self.script_pubkey)
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for TxOut {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i64le(self.value));
      r += try!(self.script_pubkey.encode(e));
      Ok(r)
   }
}
impl BitcoinDecodee for TxOut {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i64le(&mut self.value));
      r += try!(self.script_pubkey.decode(d));
      Ok(r)
   }
}

use ::script::{Script};

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
   pub fn new() -> TxOut {
      TxOut { //eq to set_null
         value: -1,
         script_pubkey: Script::new(),
      }
   }
   pub fn set_null(&mut self) {
      self.value = -1;
      self.script_pubkey = Script::new();
   }
}

impl ::std::fmt::Display for TxOut {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "TxOut(val={}, pubkey={})", self.value, self.script_pubkey)
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for TxOut {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i64le(self.value));
      r += try!(self.script_pubkey.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for TxOut {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i64le(&mut self.value));
      r += try!(self.script_pubkey.decode(d, ()));
      Ok(r)
   }
}

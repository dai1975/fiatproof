use std;
extern crate rand;

#[derive(Debug,Default,Clone)]
pub struct PingMessage
{
   pub nonce: u64,
}

impl std::fmt::Display for PingMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Ping(nonce={})", self.nonce)
   }
}

impl PingMessage {
   pub fn reset_nonce(&mut self) {
      use self::rand::Rng;
      let mut rng = rand::os::OsRng::new().unwrap(); // This rng is cryptographic level, is it too secure?
      self.nonce = rng.next_u64();
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for PingMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < e.media().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl Decodee for PingMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < d.media().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

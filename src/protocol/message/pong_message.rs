use std;
use super::PingMessage;

#[derive(Debug,Default,Clone)]
pub struct PongMessage
{
   pub nonce: u64,
}

impl std::fmt::Display for PongMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Pong(nonce={})", self.nonce)
   }
}

impl PongMessage {
   pub fn new(ping:&PingMessage) -> PongMessage {
      PongMessage{ nonce: ping.nonce }
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for PongMessage {
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
impl Decodee for PongMessage {
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

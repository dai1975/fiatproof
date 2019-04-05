use crate::crypto::digest::{Digest};
use crate::crypto::hmac::{Hmac, Mac, MacResult, helper};
use std::borrow::Borrow;

pub struct HmacUi<D:Digest> {
   pub hmac: Hmac<D>,
}

impl <D:Digest> Mac for HmacUi<D> {
   fn input(&mut self, data: &[u8]) { self.hmac.input(data) }
   fn reset(&mut self) { self.hmac.reset() }
   fn result(&mut self) -> MacResult { self.hmac.result() }
   fn raw_result(&mut self, output: &mut [u8]) { self.hmac.raw_result(output) }
   fn output_bytes(&self) -> usize { self.hmac.output_bytes() }
}

impl <D:Digest> HmacUi<D> {
   pub fn new(h:Hmac<D>) -> Self { Self { hmac:h } }

   pub fn input(&mut self, data: &[u8]) { self.hmac.input(data) }
   pub fn reset(&mut self) { self.hmac.reset() }
   pub fn raw_result(&mut self, output: &mut [u8]) { self.hmac.raw_result(output) }
   pub fn output_bytes(&self) -> usize { self.hmac.output_bytes() }
   
   pub fn input_hex<T:Borrow<str>>(&mut self, input: T) {
      helper::input_hex(&mut self.hmac, input)
   }
   pub fn input_hex_rev<T:Borrow<str>>(&mut self, input: T) {
      helper::input_hex_rev(&mut self.hmac, input)
   }

   pub fn result_u8(&mut self) -> Box<[u8]> {
      helper::result_u8(&mut self.hmac)
   }

   pub fn result_hex(&mut self) -> String {
      helper::result_hex(&mut self.hmac)
   }
   pub fn result_hex_rev(&mut self) -> String {
      helper::result_hex_rev(&mut self.hmac)
   }
   pub fn u8_to_u8<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> {
      helper::u8_to_u8(&mut self.hmac, input)
   }
   pub fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      helper::u8_to_hex(&mut self.hmac, input)
   }
   pub fn u8_to_hex_rev<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      helper::u8_to_hex_rev(&mut self.hmac, input)
   }
   pub fn hex_to_u8<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      helper::hex_to_u8(&mut self.hmac, input)
   }
   pub fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String {
      helper::hex_to_hex(&mut self.hmac, input)
   }
   pub fn hex_to_u8_rev<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      helper::hex_to_u8_rev(&mut self.hmac, input)
   }
   pub fn hex_to_hex_rev<T:Borrow<str>>(&mut self, input: T) -> String {
      helper::hex_to_hex_rev(&mut self.hmac, input)
   }
}

macro_rules! deffn {
   ($fname:ident, $t:path) => {
      pub fn $fname(key: &[u8]) -> HmacUi<$t> { HmacUi::new( Hmac::new(<$t>::new(), key)) }
   }
}

deffn! { create_hmac_sha256,    crate::crypto::digest::Sha256 }
deffn! { create_hmac_sha512,    crate::crypto::digest::Sha512 }


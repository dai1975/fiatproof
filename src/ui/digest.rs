use ::crypto::digest::{Digest, helper};
use ::std::borrow::Borrow;

pub struct Decorator<D:Digest> {
   pub digest: D,
}

impl <D:Digest> Digest for Decorator<D> {
   fn input(&mut self, input: &[u8])    { self.digest.input(input) }
   fn result(&mut self, out: &mut [u8]) { self.digest.result(out) }
   fn reset(&mut self)                  { self.digest.reset() }
   fn output_bits(&self) -> usize       { self.digest.output_bits() }
   fn output_bytes(&self) -> usize      { self.digest.output_bytes() }
   fn block_size(&self) -> usize        { self.digest.block_size() }
   
   fn input_str(&mut self, input: &str) { self.digest.input_str(input) }
   fn result_str(&mut self) -> String   { self.digest.result_str() }
}

impl <D:Digest> Decorator<D> {
   pub fn new(d:D) -> Self { Self { digest:d } }

   pub fn input_hex<T:Borrow<str>>(&mut self, input: T) {
      helper::input_hex(&mut self.digest, input)
   }
   pub fn input_hex_rev<T:Borrow<str>>(&mut self, input: T) {
      helper::input_hex_rev(&mut self.digest, input)
   }

   pub fn result_u8(&mut self) -> Box<[u8]> {
      helper::result_u8(&mut self.digest)
   }

   pub fn result_hex(&mut self) -> String {
      helper::result_hex(&mut self.digest)
   }
   pub fn result_hex_rev(&mut self) -> String {
      helper::result_hex_rev(&mut self.digest)
   }
   pub fn u8_to_u8<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> {
      helper::u8_to_u8(&mut self.digest, input)
   }
   pub fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      helper::u8_to_hex(&mut self.digest, input)
   }
   pub fn u8_to_hex_rev<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      helper::u8_to_hex_rev(&mut self.digest, input)
   }
   pub fn hex_to_u8<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      helper::hex_to_u8(&mut self.digest, input)
   }
   pub fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String {
      helper::hex_to_hex(&mut self.digest, input)
   }
   pub fn hex_to_u8_rev<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      helper::hex_to_u8_rev(&mut self.digest, input)
   }
   pub fn hex_to_hex_rev<T:Borrow<str>>(&mut self, input: T) -> String {
      helper::hex_to_hex_rev(&mut self.digest, input)
   }
}

macro_rules! deffn {
   ($fname:ident, $t:path) => {
      pub fn $fname() -> Decorator<$t> { Decorator::new( <$t>::new() ) }
   }
}

deffn! { create_sha1,      ::crypto::digest::Sha1 }
deffn! { create_sha256,    ::crypto::digest::Sha256 }
deffn! { create_ripemd160, ::crypto::digest::Ripemd160 }
deffn! { create_dhash256,  ::crypto::digest::DHash256 }
deffn! { create_hash160,   ::crypto::digest::Hash160 }


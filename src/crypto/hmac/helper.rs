use super::super::digest::Digest;
use super::{Hmac,Mac};
use ::std::borrow::{Borrow, BorrowMut};

pub trait Helpable {
   type D: Digest;
   fn create_digest() -> Self::D;
   fn create_hmac(key:&[u8]) ->  Hmac<Self::D> {
      Hmac::<Self::D>::new( Self::create_digest(), key )
   }
}

pub struct Helper<H:Helpable> {
   pub hmac: Hmac<H::D>,
}

impl <H:Helpable> Helper<H> {
   pub fn new(key: &[u8]) -> Self {
      Self { hmac: H::create_hmac(key) }
   }

   pub fn s_output_bytes()    -> usize { H::create_digest().output_bytes() }
   pub fn output_bytes(&self) -> usize { self.hmac.output_bytes() }
   
   pub fn reset(&mut self) {
      self.hmac.reset()
   }
   pub fn input(&mut self, input: &[u8]) {
      self.hmac.input(input)
   }
   pub fn result(&mut self, out: &mut [u8]) {
      self.hmac.raw_result(out)
   }
   
   pub fn input_hex<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b(input).unwrap().as_ref())
   }
   pub fn input_hex_rev<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b_rev(input).unwrap().as_ref())
   }
   pub fn result_u8(&mut self) -> Box<[u8]> {
      let len = self.hmac.output_bytes();
      let mut v = Vec::<u8>::with_capacity(len);
      unsafe { v.set_len(len); }
      self.result(v.as_mut_slice());
      v.into_boxed_slice()
   }
   pub fn result_hex(&mut self) -> String {
      ::utils::b2h(self.result_u8())
   }
   pub fn result_hex_rev(&mut self) -> String {
      ::utils::b2h_rev(self.result_u8())
   }

   pub fn u8_to_u8<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input(input.borrow());
      self.result_u8()
   }
   pub fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex()
   }
   pub fn u8_to_hex_rev<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex_rev()
   }
   pub fn hex_to_u8<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex(input.borrow());
      self.result_u8()
   }
   pub fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex(input.borrow());
      self.result_hex()
   }
   pub fn hex_to_u8_rev<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex_rev(input.borrow());
      self.result_u8()
   }
   pub fn hex_to_hex_rev<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex_rev(input.borrow());
      self.result_hex()
   }
}

macro_rules! def_helper {
   ($n:ident, $d:path) => {
      impl ::crypto::hmac::Helpable for $d {
         type D = $d;
         fn create_digest() -> Self::D {
            <$d>::new()
         }
      }
      pub type $n = ::crypto::hmac::helper::Helper<$d>;
   };
}
def_helper!(HmacSha512Helper, super::super::digest::Sha512);
def_helper!(HmacSha256Helper, super::super::digest::Sha256);


#[test]
fn test_hmac_sha512() {
   let key:&[u8]    = b"Kagamine Rin";
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "5b274c80deabf563b1e84176acc0dbf944f9d883293b98f004eeadfdfd5856af65da1d332628795766ebd034f37b94327bd10b92edad735014ddd094e1c504bd";
   
   assert_eq!(64, super::HmacSha512Helper::s_output_bytes());
   assert_eq!(expect, super::HmacSha512Helper::new(key).u8_to_hex(input));
}

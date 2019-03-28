use super::super::digest::Digest;
use super::{Hmac,Mac};
use ::std::borrow::{Borrow, BorrowMut};

pub fn input_hex<D: Digest, T:Borrow<str>>(h: &mut Hmac<D>, input: T) {
   h.input(::utils::h2b(input).unwrap().as_ref())
}
pub fn input_hex_rev<D: Digest, T:Borrow<str>>(h: &mut Hmac<D>, input: T) {
   h.input(::utils::h2b_rev(input).unwrap().as_ref())
}
pub fn result_u8<D: Digest>(h: &mut Hmac<D>) -> Box<[u8]> {
   let len = h.output_bytes();
   let mut v = Vec::<u8>::with_capacity(len);
   unsafe { v.set_len(len); }
   h.raw_result(v.as_mut_slice());
   v.into_boxed_slice()
}
pub fn result_hex<D: Digest>(h: &mut Hmac<D>) -> String {
   ::utils::b2h(result_u8(h))
}
pub fn result_hex_rev<D: Digest>(h: &mut Hmac<D>) -> String {
   ::utils::b2h_rev(result_u8(h))
}

pub fn u8_to_u8<D: Digest, T:Borrow<[u8]>>(h: &mut Hmac<D>, input: T) -> Box<[u8]> {
   h.reset();
   h.input(input.borrow());
   result_u8(h)
}
pub fn u8_to_hex<D:Digest, T:Borrow<[u8]>>(h: &mut Hmac<D>, input: T) -> String {
   h.reset();
   h.input(input.borrow());
   result_hex(h)
}
pub fn u8_to_hex_rev<D:Digest, T:Borrow<[u8]>>(h: &mut Hmac<D>, input: T) -> String {
   h.reset();
   h.input(input.borrow());
   result_hex_rev(h)
}
pub fn hex_to_u8<D:Digest, T:Borrow<str>>(h: &mut Hmac<D>, input: T) -> Box<[u8]> {
   h.reset();
   input_hex(h, input.borrow());
   result_u8(h)
}
pub fn hex_to_hex<D:Digest, T:Borrow<str>>(h: &mut Hmac<D>, input: T) -> String {
   h.reset();
   input_hex(h, input.borrow());
   result_hex(h)
}
pub fn hex_to_u8_rev<D:Digest, T:Borrow<str>>(h: &mut Hmac<D>, input: T) -> Box<[u8]> {
   h.reset();
   input_hex_rev(h, input.borrow());
   result_u8(h)
}
pub fn hex_to_hex_rev<D: Digest, T:Borrow<str>>(h: &mut Hmac<D>, input: T) -> String {
   h.reset();
   input_hex_rev(h, input.borrow());
   result_hex(h)
}

#[test]
fn test_hmac_sha512() {
   let key:&[u8]    = b"Kagamine Rin";
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "5b274c80deabf563b1e84176acc0dbf944f9d883293b98f004eeadfdfd5856af65da1d332628795766ebd034f37b94327bd10b92edad735014ddd094e1c504bd";
   
   let mut hmac = ::crypto::hmac::Hmac::new(::crypto::digest::Sha512::new(), key);
   assert_eq!(expect, super::u8_to_hex(&mut hmac, input));
}

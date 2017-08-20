use utils::{ b2h, b2h_rev };

pub trait ToOctets<T> where T:?Sized {
   fn to_octets(&self) -> ::Result<Vec<u8>>;

   fn to_hex_string(&self) -> ::Result<String> {
      self.to_octets().map(|b| b2h(b.as_slice()))
   }
   fn to_octets_rev(&self) -> ::Result<Vec<u8>> {
      self.to_octets().map(|mut b| {
         b.reverse();
         b
      })
   }
   fn to_hex_string_rev(&self) -> ::Result<String> {
      self.to_octets_rev().map(|o| b2h(o.as_slice()))
   }
}

pub trait ToDigest<T> where T:?Sized {
   fn to_digest_input(&self) -> ::Result<Vec<u8>>;
   fn to_hash160(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, Hash160 as H};
      let b = try!(self.to_digest_input());
      Ok(H::hash(b) )
   }
   fn to_dhash256(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, DHash256 as H};
      let b = try!(self.to_digest_input());
      Ok(H::hash(b))
   }
   fn to_dhash256_hex(&self) -> ::Result<String> {
      self.to_dhash256().map(|b| { b2h(b.as_ref()) })
   }
   fn to_dhash256_rhex(&self) -> ::Result<String> {
      self.to_dhash256().map(|b| { b2h_rev(b.as_ref()) })
   }
   fn to_hash160_hex(&self) -> ::Result<String> {
      self.to_hash160().map(|b| { b2h(b.as_ref()) })
   }
   fn to_hash160_rhex(&self) -> ::Result<String> {
      self.to_hash160().map(|b| { b2h_rev(b.as_ref()) })
   }
}
impl <T,X> ToDigest<T> for X where T:?Sized, X:ToOctets<T> {
   fn to_digest_input(&self) -> ::Result<Vec<u8>> {
      self.to_octets()
   }
}


use ::handy::{ b2h, b2h_rev };

pub trait ToOctets<T> where T:?Sized {
   fn to_octets(&self, opt:&str) -> ::Result<Vec<u8>>;

   fn to_hex_string(&self, opt:&str) -> ::Result<String> {
      self.to_octets(opt).map(|b| b2h(b.as_slice()))
   }
   fn to_octets_rev(&self, opt:&str) -> ::Result<Vec<u8>> {
      self.to_octets(opt).map(|mut b| {
         b.reverse();
         b
      })
   }
   fn to_hex_string_rev(&self, opt:&str) -> ::Result<String> {
      self.to_octets_rev(opt).map(|o| b2h(o.as_slice()))
   }
}

pub trait ToDigest<T> where T:?Sized {
   fn to_digest_input(&self, opt:&str) -> ::Result<Vec<u8>>;
   fn to_hash160(&self, opt:&str) -> ::Result<Box<[u8]>> {
      use ::crypto::{Digest, Hash160 as H};
      let b = try!(self.to_digest_input(opt));
      Ok(H::digest_box(&b) )
   }
   fn to_dhash256(&self, opt:&str) -> ::Result<Box<[u8]>> {
      use ::crypto::{Digest, DHash256 as H};
      let b = try!(self.to_digest_input(opt));
      Ok(H::digest_box(&b))
   }
   fn to_dhash256_hex_string(&self, opt:&str) -> ::Result<String> {
      self.to_dhash256(opt).map(|b| { b2h(b.as_ref()) })
   }
   fn to_dhash256_hex_string_rev(&self, opt:&str) -> ::Result<String> {
      self.to_dhash256(opt).map(|b| { b2h_rev(b.as_ref()) })
   }
   fn to_hash160_hex_string(&self, opt:&str) -> ::Result<String> {
      self.to_hash160(opt).map(|b| { b2h(b.as_ref()) })
   }
   fn to_hash160_hex_string_rev(&self, opt:&str) -> ::Result<String> {
      self.to_hash160(opt).map(|b| { b2h_rev(b.as_ref()) })
   }
}
impl <T,X> ToDigest<T> for X where T:?Sized, X:ToOctets<T> {
   fn to_digest_input(&self, opt:&str) -> ::Result<Vec<u8>> {
      self.to_octets(opt)
   }
}


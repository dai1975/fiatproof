use utils::{ b2h, b2h_rev };

pub trait ToBytes<T> where T:?Sized {
   fn to_bytes(&self) -> ::Result<Vec<u8>>;

   fn to_hex(&self) -> ::Result<String> {
      self.to_bytes().map(|bytes| b2h(bytes.as_slice()))
   }
   fn to_rbytes(&self) -> ::Result<Vec<u8>> {
      self.to_bytes().map(|mut bytes| {
         bytes.reverse();
         bytes
      })
   }
   fn to_rhex(&self) -> ::Result<String> {
      self.to_rbytes().map(|bytes| b2h(bytes.as_slice()))
   }
}

pub trait ToDigest<T> where T:?Sized {
   fn to_digest_input(&self) -> ::Result<Vec<u8>>;
   fn to_hash160(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, Hash160 as H};
      let bytes = try!(self.to_digest_input());
      Ok(H::hash(bytes) )
   }
   fn to_dhash256(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, DHash256 as H};
      let bytes = try!(self.to_digest_input());
      Ok(H::hash(bytes))
   }
   fn to_dhash256_hex(&self) -> ::Result<String> {
      self.to_dhash256().map(|bytes| { b2h(bytes.as_ref()) })
   }
   fn to_dhash256_rhex(&self) -> ::Result<String> {
      self.to_dhash256().map(|bytes| { b2h_rev(bytes.as_ref()) })
   }
   fn to_hash160_hex(&self) -> ::Result<String> {
      self.to_hash160().map(|bytes| { b2h(bytes.as_ref()) })
   }
   fn to_hash160_rhex(&self) -> ::Result<String> {
      self.to_hash160().map(|bytes| { b2h_rev(bytes.as_ref()) })
   }
}
impl <T,X> ToDigest<T> for X where T:?Sized, X:ToBytes<T> {
   fn to_digest_input(&self) -> ::Result<Vec<u8>> {
      self.to_bytes()
   }
}


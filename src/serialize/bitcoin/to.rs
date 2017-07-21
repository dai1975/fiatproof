use utils::{ b2h };

pub trait ToBytes {
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

impl <T> ToBytes for T where T:AsRef<[u8]> {
   fn to_bytes(&self) -> ::Result<Vec<u8>> {
      Ok( Vec::<u8>::from(self.as_ref()) )
   }
}

pub trait ToDigest {
   fn to_digest_input(&self) -> ::Result<Vec<u8>>;
   fn to_dhash256(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, DHash256 as H};
      let bytes = try!(self.to_digest_input());
      Ok(H::hash(bytes))
   }
   fn to_hash160(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, Hash160 as H};
      let bytes = try!(self.to_digest_input());
      Ok(H::hash(bytes) )
   }
   fn to_dhash256_hex(&self) -> ::Result<String> {
      self.to_dhash256().and_then(|bytes| { bytes.to_hex() })
   }
   fn to_dhash256_rhex(&self) -> ::Result<String> {
      self.to_dhash256().and_then(|bytes| { bytes.to_rhex() })
   }
   fn to_hash160_hex(&self) -> ::Result<String> {
      self.to_hash160().and_then(|bytes| { bytes.to_hex() })
   }
   fn to_hash160_rhex(&self) -> ::Result<String> {
      self.to_hash160().and_then(|bytes| { bytes.to_rhex() })
   }
}
impl <T:ToBytes> ToDigest for T {
   fn to_digest_input(&self) -> ::Result<Vec<u8>> {
      self.to_bytes()
   }
}

impl ToBytes for super::Encodee {
   fn to_bytes(&self) -> ::Result<Vec<u8>> {
      let mut w = ::serialize::VecWriteStream::default();
      {
         use super::{Encoder, Medium};
         let mut e = Encoder::new(&mut w, &Medium::default());
         let _ = try!(self.encode(&mut e));
      }
      Ok(w.into_inner())
   }
}

use utils::{ b2h, b2h_rev };

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

pub trait ToDigest {
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
impl <T:ToBytes> ToDigest for T {
   fn to_digest_input(&self) -> ::Result<Vec<u8>> {
      self.to_bytes()
   }
}

impl <T> ToBytes for T where T:super::Encodee {
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

#[cfg(test)]
mod tests {
   use ::serialize::bitcoin::{Encoder, Encodee};

   struct Foo<'a> { s: &'a str }
   impl <'a> Encodee for Foo<'a> {
      fn encode(&self, e:&mut Encoder) -> ::Result<usize> {
         e.encode_array_u8(self.s.as_bytes())
      }
   }
   
   #[test]
   fn test_to_bytes() {
      use ::serialize::bitcoin::{ToBytes};
      let f = Foo{ s:"Hatsune Miku" };
      let v = f.to_bytes().unwrap();
      assert_eq!(&[0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75], &v[..]);
   }

   #[test]
   fn test_to_digest() {
      use ::serialize::bitcoin::{ToDigest};
      let f = Foo{ s:"Hatsune Miku" };
      let v = f.to_dhash256_hex().unwrap();
      assert_eq!("e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9", v.as_str());
      let v = f.to_hash160_hex().unwrap();
      assert_eq!("b7233a798e6ea977644ded49241c2b153a6617b9", v.as_str());
      let v = f.to_hash160_rhex().unwrap();
      assert_eq!("b917663a152b1c2449ed4d6477a96e8e793a23b7", v.as_str());
   }
}

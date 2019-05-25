use secp256k1::{Secp256k1, Signing, All};
use secp256k1::key::{PublicKey, SecretKey};

pub fn create_secret_key() -> SecretKey {
   let mut rng = rand::thread_rng();
   SecretKey::new(&mut rng)
}

/**
 * set self as scalar(self) + scalar(other) (mod n)
 * error if result is 0.
 */
pub fn add_mut(sk: &mut SecretKey, other:&SecretKey) -> crate::Result<()> {
   let _ = sk.add_assign(&other[..])?;
   Ok(())
}
   
pub fn to_public_key<T:Signing>(ctx: &Secp256k1<T>, sk:&SecretKey) -> PublicKey {
   PublicKey::from_secret_key(ctx, sk)
}

pub struct RawEncoder {
}
impl RawEncoder {
   pub fn new() -> Self {
      Self { }
   }

   pub fn s_encode(sk:&SecretKey) -> Box<[u8]> {
      let v:Vec<u8> = (&sk[..]).iter().cloned().collect();
      v.into_boxed_slice()
   }
   pub fn encode(&self, sk:&SecretKey) -> Box<[u8]> {
      Self::s_encode(sk)
   }
}

pub struct RawDecoder {
}
impl RawDecoder {
   pub fn new() -> Self {
      Self { }
   }

   /**
    * fail in case of
    *  - vch.len() != 32
    *  - n <= value(vch)
    *  - value(vch) == 0
    * see SecretKey::from_slice and secp256k1_ec_seckey_verify
    */
   pub fn s_decode(vch:&[u8]) -> crate::Result<SecretKey> {
      let skey = SecretKey::from_slice(vch)?;
      Ok(skey)
   }
   pub fn decode(&self, vch:&[u8]) -> crate::Result<SecretKey> {
      Self::s_decode(vch)
   }
}


pub struct Base58checkEncoder {
   is_compressed: bool,
   b58c: crate::utils::Base58check,
}
impl Base58checkEncoder {
   pub fn new(b58c: crate::utils::Base58check, is_compressed:bool) -> Self {
      Self {
         is_compressed:is_compressed,
         b58c: b58c,
      }
   }

   pub fn encode(&self, sk:&SecretKey) -> String {
      let bytes = RawEncoder::new().encode(sk);
      if self.is_compressed {
         let mut v = Vec::from(bytes.as_ref());
         v.push(1);
         self.b58c.encode(v.as_slice())
      } else {
         self.b58c.encode(bytes.as_ref())
      }
   }
}

pub struct Base58checkDecoder {
   b58c: crate::utils::Base58check,
}
impl Base58checkDecoder {
   pub fn new(b58c: crate::utils::Base58check) -> Self {
      Self {
         b58c: b58c,
      }
   }

   pub fn decode_base58check(&self, s: &str) -> crate::Result<(Box<[u8]>, bool)> {
      //check base58check and version bytes is match
      let bytes = self.b58c.decode(s)?; 
      //check 32bytes or 33bytes compression format
      let is_compressed = if bytes.len() == 32 {
         Ok(false)
      } else if bytes.len() == 33 && bytes[32] == 1 {
         Ok(true)
      } else {
         error_secp256k1_error!("malformed secret key base58check")
      }?;
      Ok((bytes, is_compressed))
   }
   
   pub fn decode(&self, s: &str) -> crate::Result<SecretKey> {
      let (bytes, _is_compressed) = self.decode_base58check(s)?;
      let dec = RawDecoder::new();
      dec.decode(&bytes[0..32])
   }
}


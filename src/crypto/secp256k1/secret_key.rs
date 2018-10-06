use super::{Secp256k1, SecretKey, PublicKey, Signing};

extern crate rand;
fn random_32_bytes<R: rand::Rng>(rng: &mut R) -> [u8; 32] {
    let mut ret = [0u8; 32];
    rng.fill_bytes(&mut ret);
    ret
}

pub fn create_secret_key<T>(ctx: &Secp256k1<T>) -> SecretKey {
   let mut rng = rand::thread_rng();
   let mut data = random_32_bytes(&mut rng);
   let sk = loop {
      if let Ok(sk) = SecretKey::from_slice(ctx, &data) {
         break sk;
      }
      data = random_32_bytes(&mut rng);
   };
   sk
}

/**
 * set self as scalar(self) + scalar(other) (mod n)
 * error if result is 0.
 */
pub fn add_mut<T>(ctx: &Secp256k1<T>, sk: &mut SecretKey, other:&SecretKey) -> ::Result<()> {
   let _ = sk.add_assign(ctx, &other)?;
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

   pub fn encode(&self, sk:&SecretKey) -> Box<[u8]> {
      let v:Vec<u8> = (&sk[..]).iter().cloned().collect();
      v.into_boxed_slice()
   }
}

pub struct RawDecoder {
   ctx: Secp256k1<super::secp256k1::All>,
}
impl RawDecoder {
   pub fn new() -> Self {
      Self { ctx: Secp256k1::new() }
   }

   /**
    * fail in case of
    *  - vch.len() != 32
    *  - n <= value(vch)
    *  - value(vch) == 0
    * see SecretKey::from_slice and secp256k1_ec_seckey_verify
    */
   pub fn s_decode<T>(ctx: &Secp256k1<T>, vch:&[u8]) -> ::Result<SecretKey> {
      let skey = SecretKey::from_slice(ctx, vch)?;
      Ok(skey)
   }
   pub fn decode(&self, vch:&[u8]) -> ::Result<SecretKey> {
      Self::s_decode(&self.ctx, vch)
   }
}


pub struct Base58checkEncoder {
   is_compressed: bool,
   b58c: ::utils::Base58check,
}
impl Base58checkEncoder {
   pub fn new(b58c: ::utils::Base58check, is_compressed:bool) -> Self {
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
   b58c: ::utils::Base58check,
}
impl Base58checkDecoder {
   pub fn new(b58c: ::utils::Base58check) -> Self {
      Self {
         b58c: b58c,
      }
   }

   pub fn decode_base58check(&self, s: &str) -> ::Result<(Box<[u8]>, bool)> {
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
   
   pub fn decode(&self, s: &str) -> ::Result<SecretKey> {
      let (bytes, _is_compressed) = self.decode_base58check(s)?;
      let dec = RawDecoder::new();
      dec.decode(&bytes[0..32])
   }
}


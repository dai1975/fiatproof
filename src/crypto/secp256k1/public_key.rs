use super::{Secp256k1, PublicKey, SecretKey, Message, Signature, Verification};
use ::std::error::Error;

pub fn add_secret_key<T:Verification>(ctx: &Secp256k1<T>, pk:&mut PublicKey, sk: &SecretKey) -> ::Result<()> {
   let _ = pk.add_exp_assign(&ctx, sk)?;
   Ok(())
}

pub fn verify<T:Verification>(ctx: &Secp256k1<T>, pk: &PublicKey, msg: &[u8], sig: &Signature) -> ::Result<()> {
   let message = Message::from_slice(msg).map_err(|e| {
      secp256k1_error!(e.description())
   })?;
   let _ = ctx.verify(&message, sig, pk).map_err(|e| {
      secp256k1_error!(e.description())
   })?;
   Ok(())
}

pub const SEC1_TAG_ODD:u8 = 0x02;
pub const SEC1_TAG_EVEN:u8 = 0x03;
pub const SEC1_TAG_UNCOMPRESSED:u8 = 0x04;
pub const SEC1_TAG_HYBRID_EVEN:u8 = 0x06;
pub const SEC1_TAG_HYBRID_ODD:u8 = 0x07;

pub struct Sec1Encoder {
   compress:bool,
}
impl Sec1Encoder {
   pub fn new(compress: bool) -> Self {
      Self { compress:compress }
   }

   pub fn s_encode(compress:bool, pk:&PublicKey) -> Box<[u8]> {
      match compress {
         true  => Box::new(pk.serialize()),
         false => Box::new(pk.serialize_uncompressed()),
      }
   }
   pub fn s_encode_to(compress:bool, pk:&PublicKey, out:&mut[u8]) {
      let _ = match compress {
         true  => {
            out.clone_from_slice(&pk.serialize());
         },
         false => {
            out.clone_from_slice(&pk.serialize_uncompressed());
         },
      };
   }
   
   pub fn encode(&self, pk:&PublicKey) -> Box<[u8]> {
      Self::s_encode(self.compress, pk)
   }
   pub fn encode_to(&self, pk:&PublicKey, out:&mut[u8]) {
      Self::s_encode_to(self.compress, pk, out)
   }
}

pub struct Sec1Decoder {
   ctx: Secp256k1<super::secp256k1::All>,
   compress: Option<bool>,
   hybrid:   bool,
}
impl Sec1Decoder {
   pub fn new(compress: Option<bool>, hybrid:bool) -> Self {
      Self {
         ctx: Secp256k1::new(),
         compress: compress,
         hybrid: hybrid
      }
   }

   pub fn check(&self, vch:&[u8]) -> ::Result<()> {
      Self::s_check(self.compress, self.hybrid, vch)
   }
   pub fn s_check(compress: Option<bool>, hybrid:bool, vch:&[u8]) -> ::Result<()> {
      let len = vch.len();
      if len == 0 {
         raise_secp256k1_error!("empty");
      }
      match vch[0] {
         2 | 3 if compress.unwrap_or(true) => {
            if len != 33 {
               raise_secp256k1_error!(format!("compressed but not 33 bytes length: {}", len));
            }
            Ok(())
         },
         4 if !compress.unwrap_or(false) => {
            if len != 65 {
               raise_secp256k1_error!(format!("compressed but not 65 bytes length: {}", len));
            }
            Ok(())
         },
         6 | 7 if !compress.unwrap_or(false) && hybrid => {
            if len != 65 {
               raise_secp256k1_error!(format!("compressed but not 65 bytes length: {}", len));
            }
            Ok(())
         },
         _ => {
            error_secp256k1_error!(format!("unexpected tag: {}", vch[0]))
         },
      }
   }
   
   pub fn s_decode<T>(ctx: &Secp256k1<T>, compress: Option<bool>, hybrid: bool, vch:&[u8]) -> ::Result<PublicKey> {
      if compress.is_some() || !hybrid {
         Self::s_check(compress, hybrid, vch)?;
      }
      let pk = PublicKey::from_slice(ctx, vch).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(pk)
   }
      
   pub fn decode(&self, vch:&[u8]) -> ::Result<PublicKey> {
      Self::s_decode(&self.ctx, self.compress, self.hybrid, vch)
   }
}

pub struct RawEncoder { }
impl RawEncoder {
   pub fn new() -> Self { Self {} }
   
   pub fn s_encode(pk:&PublicKey) -> [u8; 64] {
      let u65 = &pk.serialize_uncompressed();
      let mut out = [0u8; 64];
      out.copy_from_slice(&u65[1..]);
      out
   }
   pub fn s_encode_to(pk:&PublicKey, out:&mut [u8;64]) {
      let u65 = &pk.serialize_uncompressed();
      out.copy_from_slice(&u65[1..]);
   }

   pub fn encode(&self, pk:&PublicKey) -> [u8; 64] {
      Self::s_encode(pk)
   }
   pub fn encode_to(&self, pk:&PublicKey, out:&mut [u8;64]) {
      Self::s_encode_to(pk, out)
   }
}

pub struct RawDecoder {
   ctx: Secp256k1<super::secp256k1::All>,
}
impl RawDecoder {
   pub fn new() -> Self {
      Self { ctx: Secp256k1::new() }
   }

   pub fn s_decode<T>(ctx: &Secp256k1<T>, bytes:&[u8;64]) -> ::Result<PublicKey> {
      let mut vch = [0u8;65];
      vch[0] = SEC1_TAG_UNCOMPRESSED;
      (&mut vch[1..]).copy_from_slice(bytes);
      let pk = PublicKey::from_slice(ctx, &vch[..]).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(pk)
   }
   pub fn decode(&self, bytes:&[u8;64]) -> ::Result<PublicKey> {
      Self::s_decode(&self.ctx, bytes)
   }
}


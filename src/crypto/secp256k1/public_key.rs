use super::{Secp256k1, PublicKey, SecretKey};
use ::std::error::Error;

pub const SEC1_TAG_ODD:u8 = 0x02;
pub const SEC1_TAG_EVEN:u8 = 0x03;
pub const SEC1_TAG_UNCOMPRESSED:u8 = 0x04;
pub const SEC1_TAG_HYBRID_EVEN:u8 = 0x06;
pub const SEC1_TAG_HYBRID_ODD:u8 = 0x07;

pub struct Helper {
   ctx: Secp256k1<super::secp256k1::All>,
}

impl Helper {
   pub fn new() -> Self {
      Self { ctx: Secp256k1::new() }
   }
   pub fn add_secret_key(&self, pk: &mut PublicKey, sec:&SecretKey) -> ::Result<()> {
      let _ = pk.add_exp_assign(&self.ctx, sec)?;
      Ok(())
   }
}

pub struct Sec1Encoder {
   compress:bool,
}
impl Sec1Encoder {
   pub fn new(compress: bool) -> Self {
      Self { compress:compress }
   }

   pub fn encode(&self, pk:&PublicKey) -> Box<[u8]> {
      match self.compress {
         true  => Box::new(pk.serialize()),
         false => Box::new(pk.serialize_uncompressed()),
      }
   }
   pub fn encode_to(&self, pk:&PublicKey, out:&mut[u8]) {
      let _ = match self.compress {
         true  => {
            out.clone_from_slice(&pk.serialize());
         },
         false => {
            out.clone_from_slice(&pk.serialize_uncompressed());
         },
      };
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
      let len = vch.len();
      if len == 0 {
         raise_secp256k1_error!("empty");
      }
      match vch[0] {
         2 | 3 if self.compress.unwrap_or(true) => {
            if len != 33 {
               raise_secp256k1_error!(format!("compressed but not 33 bytes length: {}", len));
            }
            Ok(())
         },
         4 if !self.compress.unwrap_or(false) => {
            if len != 65 {
               raise_secp256k1_error!(format!("compressed but not 65 bytes length: {}", len));
            }
            Ok(())
         },
         6 | 7 if !self.compress.unwrap_or(false) && self.hybrid => {
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
   
   pub fn decode(&self, vch:&[u8]) -> ::Result<PublicKey> {
      if self.compress.is_some() || !self.hybrid {
         self.check(vch)?;
      }
      let pk = PublicKey::from_slice(&self.ctx, vch).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(pk)
   }
}

pub struct RawEncoder { }
impl RawEncoder {
   pub fn new() -> Self { Self {} }
   pub fn encode(&self, pk:&PublicKey) -> [u8; 64] {
      let u65 = &pk.serialize_uncompressed();
      let mut out = [0u8; 64];
      out.copy_from_slice(&u65[1..]);
      out
   }
}

pub struct RawDecoder {
   ctx: Secp256k1<super::secp256k1::All>,
}
impl RawDecoder {
   pub fn new() -> Self {
      Self { ctx: Secp256k1::new() }
   }

   pub fn decode(&self, bytes:&[u8;64]) -> ::Result<PublicKey> {
      let mut vch = [0u8;65];
      vch[0] = SEC1_TAG_UNCOMPRESSED;
      (&mut vch[1..]).copy_from_slice(bytes);
      let pk = PublicKey::from_slice(&self.ctx, &vch[..]).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(pk)
   }
}

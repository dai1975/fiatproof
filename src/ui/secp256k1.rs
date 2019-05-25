use num::bigint::BigUint;
use secp256k1::{Secp256k1, Signature, All};
use secp256k1::key::{PublicKey, SecretKey};
use crate::crypto::secp256k1::{signature, public_key, secret_key};

#[derive(Clone)]
pub struct SignatureUi {
   pub signature: Signature,
   pub ctx: Secp256k1<All>,
}

impl SignatureUi {
   pub fn into_signature(self) -> Signature { self.signature }
   
   pub fn new(sig: Signature) -> Self {
      Self { signature:sig, ctx: Secp256k1::new() }
   }
   pub fn get_raw(&self) -> (BigUint, BigUint) {
      signature::get_raw(&self.signature)
   }
   pub fn is_low_s(&self) -> bool {
      signature::is_low_s(&self.signature)
   }
   pub fn normalize_s(&mut self) -> bool {
      signature::normalize_s(&mut self.signature)
   }
   pub fn encode_der(&self) -> Box<[u8]> {
      signature::DerEncoder::s_encode(&self.signature)
   }
   
   pub fn s_check_strict(vch: &[u8]) -> crate::Result<()> {
      signature::DerDecoder::s_check_strict(vch)
   }
   pub fn s_decode_der(is_strict: bool, vch: &[u8]) -> crate::Result<SignatureUi> {
      let ctx = Secp256k1::new();
      let sig = signature::DerDecoder::s_decode(is_strict, vch)?;
      Ok(Self { signature:sig, ctx:ctx }) 
   }
   pub fn decode_der(&mut self, is_strict:bool, vch: &[u8]) -> crate::Result<()> {
      self.signature = signature::DerDecoder::s_decode(is_strict, vch)?;
      Ok(())
   }
}

#[derive(Clone)]
pub struct PublicKeyUi {
   pub public_key: PublicKey,
   pub ctx: Secp256k1<All>,
}

impl PublicKeyUi {
   pub fn into_public_key(self) -> PublicKey { self.public_key }
   pub fn new(pk: PublicKey) -> Self {
      Self { public_key:pk, ctx: Secp256k1::new() }
   }

   pub fn add_secret_key(&mut self, sk: &SecretKeyUi) -> crate::Result<()> {
      public_key::add_secret_key(&self.ctx, &mut self.public_key, &sk.secret_key)
   }
   pub fn verify(&self, msg: &[u8], sig: &SignatureUi) -> crate::Result<()> {
      public_key::verify(&self.ctx, &self.public_key, msg, &sig.signature)
   }
   
   pub fn encode_sec1(&self, compress: bool) -> Box<[u8]> {
      public_key::Sec1Encoder::s_encode(compress, &self.public_key)
   }
   pub fn encode_sec1_to(&self, compress: bool, out: &mut [u8]) {
      public_key::Sec1Encoder::s_encode_to(compress, &self.public_key, out)
   }
   pub fn check_sec1(&self, compress: Option<bool>, hybrid: bool, vch:&[u8]) -> crate::Result<()> {
      public_key::Sec1Decoder::s_check(compress, hybrid, vch)
   }
   pub fn s_check_sec1(compress: Option<bool>, hybrid: bool, vch:&[u8]) -> crate::Result<()> {
      public_key::Sec1Decoder::s_check(compress, hybrid, vch)
   }
   pub fn decode_sec1(&mut self, compress: Option<bool>, hybrid: bool, vch:&[u8]) -> crate::Result<()> {
      self.public_key = public_key::Sec1Decoder::s_decode(compress, hybrid, vch)?;
      Ok(())
   }
   pub fn s_decode_sec1(compress: Option<bool>, hybrid: bool, vch:&[u8]) -> crate::Result<Self> {
      let pk = public_key::Sec1Decoder::s_decode(compress, hybrid, vch)?;
      Ok(Self::new(pk))
   }

   pub fn encode_raw(&self) -> [u8; 64] {
      public_key::RawEncoder::s_encode(&self.public_key)
   }
   pub fn encode_raw_to(&self, out: &mut [u8;64]) {
      public_key::RawEncoder::s_encode_to(&self.public_key, out)
   }
   pub fn decode_raw(&mut self, bytes:&[u8;64]) -> crate::Result<()> {
      self.public_key = public_key::RawDecoder::s_decode(bytes)?;
      Ok(())
   }
   pub fn s_decode_raw(bytes:&[u8;64]) -> crate::Result<Self> {
      let pk = public_key::RawDecoder::s_decode(bytes)?;
      Ok(Self::new(pk))
   }
}



#[derive(Clone)]
pub struct SecretKeyUi {
   pub secret_key: SecretKey,
   pub ctx: Secp256k1<secp256k1::All>,
}

impl SecretKeyUi {
   pub fn into_secret_key(self) -> SecretKey { self.secret_key }
   pub fn new(sk: SecretKey) -> Self {
      Self { secret_key:sk, ctx: Secp256k1::new() }
   }
   pub fn new_random() -> Self {
      let ctx = Secp256k1::new();
      let sk = secret_key::create_secret_key();
      Self { secret_key:sk, ctx:ctx }
   }

   pub fn add(&mut self, other:&SecretKeyUi) -> crate::Result<()> {
      secret_key::add_mut(&mut self.secret_key, &other.secret_key)
   }
   pub fn add_raw(&mut self, other:&SecretKey) -> crate::Result<()> {
      secret_key::add_mut(&mut self.secret_key, other)
   }
   pub fn to_public_key(&self) -> PublicKeyUi {
      let pk = secret_key::to_public_key(&self.ctx, &self.secret_key);
      PublicKeyUi::new(pk)
   }

   pub fn encode_raw(&self) -> Box<[u8]> {
      secret_key::RawEncoder::new().encode(&self.secret_key)
   }
   pub fn decode_raw(&mut self, vch: &[u8]) -> crate::Result<()> {
      self.secret_key = secret_key::RawDecoder::s_decode(vch)?;
      Ok(())
   }
   pub fn s_decode_raw(vch: &[u8]) -> crate::Result<Self> {
      let sk = secret_key::RawDecoder::s_decode(vch)?;
      Ok(Self::new(sk))
   }
}


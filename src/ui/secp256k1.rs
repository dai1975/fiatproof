extern crate num;
use self::num::bigint::BigUint;
use ::crypto::secp256k1;

#[derive(Clone)]
pub struct SignatureUi {
   pub signature:  secp256k1::Signature,
   pub ctx: secp256k1::Secp256k1<secp256k1::All>,
}

impl SignatureUi {
   pub fn into_signature(self) -> secp256k1::Signature { self.signature }
   
   pub fn new(sig: secp256k1::Signature) -> Self {
      Self { signature:sig, ctx: secp256k1::Secp256k1::new() }
   }
   pub fn get_raw(&self) -> (BigUint, BigUint) {
      secp256k1::signature::get_raw(&self.ctx, &self.signature)
   }
   pub fn is_low_s(&self) -> bool {
      secp256k1::signature::is_low_s(&self.ctx, &self.signature)
   }
   pub fn normalize_s(&mut self) -> bool {
      secp256k1::signature::normalize_s(&self.ctx, &mut self.signature)
   }
   pub fn encode_der(&self) -> Box<[u8]> {
      secp256k1::signature::DerEncoder::s_encode(&self.ctx, &self.signature)
   }
   
   pub fn s_check_strict(vch: &[u8]) -> ::Result<()> {
      secp256k1::signature::DerDecoder::s_check_strict(vch)
   }
   pub fn s_decode_der(is_strict: bool, vch: &[u8]) -> ::Result<SignatureUi> {
      let ctx = secp256k1::Secp256k1::new();
      let sig = secp256k1::signature::DerDecoder::s_decode(&ctx, is_strict, vch)?;
      Ok(Self { signature:sig, ctx:ctx }) 
   }
   pub fn decode_der(&mut self, is_strict:bool, vch: &[u8]) -> ::Result<()> {
      self.signature = secp256k1::signature::DerDecoder::s_decode(&self.ctx, is_strict, vch)?;
      Ok(())
   }
}

#[derive(Clone)]
pub struct PublicKeyUi {
   pub public_key:  secp256k1::PublicKey,
   pub ctx: secp256k1::Secp256k1<secp256k1::All>,
}

impl PublicKeyUi {
   pub fn into_public_key(self) -> secp256k1::PublicKey { self.public_key }
   pub fn new(pk: secp256k1::PublicKey) -> Self {
      Self { public_key:pk, ctx: secp256k1::Secp256k1::new() }
   }

   pub fn add_secret_key(&mut self, sk: &SecretKeyUi) -> ::Result<()> {
      secp256k1::public_key::add_secret_key(&self.ctx, &mut self.public_key, &sk.secret_key)
   }
   pub fn verify(&self, msg: &[u8], sig: &SignatureUi) -> ::Result<()> {
      secp256k1::public_key::verify(&self.ctx, &self.public_key, msg, &sig.signature)
   }
   
   pub fn encode_sec1(&self, compress: bool) -> Box<[u8]> {
      secp256k1::Sec1Encoder::s_encode(compress, &self.public_key)
   }
   pub fn encode_sec1_to(&self, compress: bool, out: &mut [u8]) {
      secp256k1::Sec1Encoder::s_encode_to(compress, &self.public_key, out)
   }
   pub fn check_sec1(&self, compress: Option<bool>, hybrid: bool, vch:&[u8]) -> ::Result<()> {
      secp256k1::Sec1Decoder::s_check(compress, hybrid, vch)
   }
   pub fn s_check_sec1(compress: Option<bool>, hybrid: bool, vch:&[u8]) -> ::Result<()> {
      secp256k1::Sec1Decoder::s_check(compress, hybrid, vch)
   }
   pub fn decode_sec1(&mut self, compress: Option<bool>, hybrid: bool, vch:&[u8]) -> ::Result<()> {
      self.public_key = secp256k1::Sec1Decoder::s_decode(&self.ctx, compress, hybrid, vch)?;
      Ok(())
   }
   pub fn s_decode_sec1(compress: Option<bool>, hybrid: bool, vch:&[u8]) -> ::Result<Self> {
      let ctx = secp256k1::Secp256k1::new();
      let pk = secp256k1::public_key::Sec1Decoder::s_decode(&ctx, compress, hybrid, vch)?;
      Ok(Self::new(pk))
   }

   pub fn encode_raw(&self) -> [u8; 64] {
      secp256k1::public_key::RawEncoder::s_encode(&self.public_key)
   }
   pub fn encode_raw_to(&self, out: &mut [u8;64]) {
      secp256k1::public_key::RawEncoder::s_encode_to(&self.public_key, out)
   }
   pub fn decode_raw(&mut self, bytes:&[u8;64]) -> ::Result<()> {
      self.public_key = secp256k1::public_key::RawDecoder::s_decode(&self.ctx, bytes)?;
      Ok(())
   }
   pub fn s_decode_raw(bytes:&[u8;64]) -> ::Result<Self> {
      let ctx = secp256k1::Secp256k1::new();
      let pk = secp256k1::public_key::RawDecoder::s_decode(&ctx, bytes)?;
      Ok(Self::new(pk))
   }
}



#[derive(Clone)]
pub struct SecretKeyUi {
   pub secret_key:  secp256k1::SecretKey,
   pub ctx: secp256k1::Secp256k1<secp256k1::All>,
}

impl SecretKeyUi {
   pub fn into_secret_key(self) -> secp256k1::SecretKey { self.secret_key }
   pub fn new(sk: secp256k1::SecretKey) -> Self {
      Self { secret_key:sk, ctx: secp256k1::Secp256k1::new() }
   }
   pub fn new_random() -> Self {
      let ctx = secp256k1::Secp256k1::new();
      let sk = secp256k1::secret_key::create_secret_key(&ctx);
      Self { secret_key:sk, ctx:ctx }
   }

   pub fn add(&mut self, other:&SecretKeyUi) -> ::Result<()> {
      secp256k1::secret_key::add_mut(&self.ctx, &mut self.secret_key, &other.secret_key)
   }
   pub fn add_raw(&mut self, other:&secp256k1::SecretKey) -> ::Result<()> {
      secp256k1::secret_key::add_mut(&self.ctx, &mut self.secret_key, other)
   }
   pub fn to_public_key(&self) -> PublicKeyUi {
      let pk = secp256k1::secret_key::to_public_key(&self.ctx, &self.secret_key);
      PublicKeyUi::new(pk)
   }

   pub fn encode_raw(&self) -> Box<[u8]> {
      secp256k1::secret_key::RawEncoder::new().encode(&self.secret_key)
   }
   pub fn decode_raw(&mut self, vch: &[u8]) -> ::Result<()> {
      self.secret_key = secp256k1::secret_key::RawDecoder::s_decode(&self.ctx, vch)?;
      Ok(())
   }
   pub fn s_decode_raw(vch: &[u8]) -> ::Result<Self> {
      let ctx = secp256k1::Secp256k1::new();
      let sk = secp256k1::secret_key::RawDecoder::s_decode(&ctx, vch)?;
      Ok(Self::new(sk))
   }
}


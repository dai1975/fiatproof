extern crate secp256k1;

pub struct Signature(secp256k1::Signature);

impl Signature {
   pub fn inner(&self) -> &secp256k1::Signature { &self.0 }
}

pub fn parse(vch:&[u8]) -> ::Result<Signature> {
   let ctx = secp256k1::Secp256k1::new();
   let inner = secp256k1::Signature::from_der_lax(
      &ctx, vch
   ).map(|mut sig| {
      sig.normalize_s(&ctx);
      sig
   }).map_err(|e| {
      use ::std::error::Error;
      secp256k1_error!(e.description())
   })?;
   Ok(Signature(inner))
}

pub fn check_format(vch:&[u8]) -> ::Result<()> {
   let len = vch.len();
   if len < 9 { raise_secp256k1_error!(format!("sigenc: too short: {}", len)); }
   if len >73 { raise_secp256k1_error!(format!("sigenc: too long: {}", len)); }
   
   if vch[0] != 0x30 { raise_secp256k1_error!(format!("sigenc: [0] != 0x30: {:x}", vch[0])); }
   if (vch[1]+3) as usize != len { raise_secp256k1_error!(format!("sigenc: [1]={}, len={}", vch[1], len)); }
   
   let len_r = vch[3] as usize;
   if 5 + len_r >= len { raise_secp256k1_error!(format!("sigenc: len_r={}, len={}", len_r, len)); }

   let len_s = vch[5 + len_r] as usize;
   if len_r + len_s + 7 != len { raise_secp256k1_error!(format!("sigenc: len_r={}, len_s={}, len={}", len_r, len_s, len)); }

   if vch[2] != 0x02 { raise_secp256k1_error!(format!("sigenc: [2] != 0x02: {:x}", vch[2])); }
   if len_r == 0 { raise_secp256k1_error!(format!("sigenc: len_r == 0: {}", len_r)); }
   if (vch[4] & 0x80) != 0 { raise_secp256k1_error!(format!("sigenc: [4]&0x80 != 0: {:x}", vch[4])); }
   // R is minimal format so that heading zero is not allowed
   if (len_r > 1) && (vch[4] == 0x00) && ((vch[5] & 0x80) == 0) { raise_secp256k1_error!(format!("sigenc: len_r={}, [4]={:x}, [5]={:x}", len_r, vch[4], vch[5])); }
   
   if vch[len_r+4] != 0x02 { raise_secp256k1_error!(format!("sigenc: [{}+4] != 0x02: {:x}", len_r, vch[len_r+4])); }
   if len_s == 0 { raise_secp256k1_error!(format!("sigenc: len_s == 0: {}", len_s)); }
   if (vch[len_r+6] & 0x80) != 0  { raise_secp256k1_error!(format!("sigenc: [{}+6]&0x80 != 0: {:x}", len_r, vch[len_r+6])); }
   // S is minimal format so that heading zero is not allowed   
   if (len_s > 1) && (vch[len_r+6] == 0x00) && ((vch[len_r+7] & 0x80) == 0) { raise_secp256k1_error!(format!("sigenc: len_s={}, [{}+6]={:x}, [{}+7]={:x}", len_s, len_r, vch[len_r+6], len_r, vch[len_r+7])); }

   Ok(())
}

pub fn check_format_low_der(vch:&[u8]) -> ::Result<()> {
   let r = unsafe {
      // call ffi directly because rust-secp256k1 drops return value of normalize_s...
      extern crate libc;
      use self::secp256k1::ffi;
      let mut result = Ok(());
      let ctx = ffi::secp256k1_context_create(ffi::SECP256K1_START_VERIFY);
      let mut sig = ffi::Signature::blank();

      // bitcoin/src/pubkey.cpp/CheckLowS
      //   if (!ecdsa_signature_parse_der_lax(...) { return false; }
      //   return (!secp256k1_ecdsa_signature_normalize(...);
      let r = ffi::ecdsa_signature_parse_der_lax(ctx, &mut sig, vch.as_ptr(), vch.len() as libc::size_t);
      if r != 1 {
         result = Err(secp256k1_error!(format!("parse fail({})", r)));
      } else {
         let r = ffi::secp256k1_ecdsa_signature_normalize(ctx, &mut sig as *mut ffi::Signature, &sig as *const ffi::Signature);
         if r != 0 {
            result = Err(secp256k1_error!(format!("normalize fail({})", r)))
         }
      }
      ffi::secp256k1_context_destroy(ctx);
      result
   };
   let _ = r?;
   Ok(())
}


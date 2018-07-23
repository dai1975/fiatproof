use ::utils::BaseN;

lazy_static! {
   pub static ref BASE58:BaseN = {
      BaseN::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz")
   };
}

pub fn base58_encode(bytes:&[u8]) -> String {
   BASE58.encode(bytes)
}
pub fn base58_decode(s:&str) -> ::Result<Vec<u8>> {
   BASE58.decode(s)
}

pub fn base58check_encode(bytes:&[u8], version:&[u8]) -> String {
   let mut check = [0u8; 32];
   {
      use ::crypto::Digest;
      let mut hasher = ::crypto::DHash256::default();
      hasher.input(version);
      hasher.input(bytes);
      hasher.result(&mut check);
   }
   let mut v = Vec::with_capacity(version.len() + bytes.len() + 4);
   v.extend(version);
   v.extend(bytes);
   v.extend(&check[0..4]);
   println!("b58check={}", ::ui::b2h(v.as_slice()));
   BASE58.encode(v.as_slice())
}

pub fn base58check_decode(s:&str) -> ::Result<Vec<u8>> {
   let mut v = try!(BASE58.decode(s));
   let len0 = v.len() - 4;
   let mut check = [0u8; 32];
   {
      use ::crypto::Digest;
      let mut hasher = ::crypto::DHash256::default();
      hasher.input(&v[0..len0]);
      hasher.result(&mut check);
   }
   if &v[len0..] != &check[0..4] {
      raise_unknown_error!("mac mismatch")
   }
   v.truncate(len0);
   Ok(v)
}

mod tests {
   use ::bitcoin::utils::base58_encode;
   use ::bitcoin::utils::base58_decode;
   use ::bitcoin::utils::base58check_encode;
   use ::bitcoin::utils::base58check_decode;
   
   #[test]
   fn test_encode_b58() {
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "Rt5zm"; // 281563422 = 22*58^4 + 51*58^4 + 4*58^4 + 57*58^4 + 44*58^0
      let result = base58_encode(&data);
      assert_eq!(enc, result);
   }

   #[test]
   fn test_decode_b58() {
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "Rt5zm"; // 281563422 = 22*58^4 + 51*58^4 + 4*58^4 + 57*58^4 + 44*58^0
      let result = base58_decode(enc);
      assert_matches!(result, Ok(_));
      assert_eq!(data, result.unwrap().as_slice());
   }

   #[test]
   fn test_encode_b58check() {
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e];
      let enc = "13op3it3Aaiu";
      let result = base58check_encode(&data, &[0u8]);
      assert_eq!(enc, result);
   }

   #[test]
   fn test_decode_b58check() {
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "13op3it3Aaiu";
      let result = base58check_decode(enc);
      assert_matches!(result, Ok(_));
      let result = result.unwrap();
      assert_eq!(0u8, result[0]);
      assert_eq!(data, &result[1..]);
   }
}

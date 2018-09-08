use super::P2PKH;
use ::utils::Base58check;

pub struct Encoder<'a> {
   b58c: &'a Base58check,
}

impl <'a> Encoder<'a> {
   pub fn new(b58c: &'a Base58check) -> Self {
      Self { b58c: b58c }
   }
   pub fn base58check(&self) -> &Base58check { self.b58c }
   
   pub fn encode(&self, p2pkh: &P2PKH) -> String {
      self.b58c.serialize(p2pkh.pkh())
   }
}

pub struct Decoder<'a> {
   b58c: &'a Base58check,
}
impl <'a> Decoder<'a> {
   pub fn new(b58c: &'a Base58check) -> Self {
      Self { b58c: b58c }
   }
   pub fn base58check(&self) -> &Base58check { self.b58c }
   
   pub fn decode(&self, s:&str) -> ::Result<P2PKH> {
      let bytes = self.b58c.deserialize(s)?;
      let p2pkh = P2PKH::new_with_pkh(bytes.as_ref())?;
      Ok(p2pkh)
   }
}

#[cfg(test)]
mod tests {
   const HASH:&[u8] = hex!("1018853670f9f3b0582c5b9ee8ce93764ac32b93");
   const ADDR:&str = "12U7BmtQsKowQYEfrZ5SeFjsSMARUr2fSi";
      
   #[test]
   fn test_encode() {
      let b58c  = ::bitcoin::utils::new_base58check(&[0u8]);
      let enc   = ::bitcoin::p2pkh::AddressEncoder::new(&b58c);
      let p2pkh = ::bitcoin::p2pkh::P2PKH::new_with_pkh(HASH).unwrap();
      let addr  = enc.encode(&p2pkh);
      assert_eq!(addr, ADDR);
   }

   #[test]
   fn test_decode() {
      let b58c  = ::bitcoin::utils::new_base58check(&[0u8]);
      let dec   = ::bitcoin::p2pkh::AddressDecoder::new(&b58c);
      let result = dec.decode(ADDR);
      assert_matches!(result, Ok(_));
      assert_eq!(result.unwrap().pkh(), HASH);
   }
}

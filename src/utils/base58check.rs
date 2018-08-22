use super::BaseN;

pub struct Base58check {
   base_n: BaseN,
   version: Box<[u8]>,
}

impl Base58check {
   pub fn new(table: &str, version: &[u8]) -> Self {
      Self {
         base_n:  BaseN::new(table),
         version: version.to_vec().into_boxed_slice()
      }
   }
   pub fn base_size(&self) -> usize { 58usize }
   
   pub fn serialize(&self, bytes: &[u8]) -> String {
      let mut check = [0u8; 32];
      {
         use ::crypto::Digest;
         let mut hasher = ::crypto::DHash256::default();
         hasher.input(&self.version);
         hasher.input(bytes);
         hasher.result(&mut check);
      }
      let mut v = Vec::with_capacity(self.version.len() + bytes.len() + 4);
      v.extend(self.version.iter());
      v.extend(bytes);
      v.extend(&check[0..4]);
      self.base_n.serialize(v.as_slice())
   }

   pub fn deserialize(&self, s:&str) -> ::Result<Vec<u8>> {
      let mut v = try!(self.base_n.deserialize(s));
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
}

/*   
lazy_static! {
   pub static ref BASE58:BaseN = {
      BaseN::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz")
   };
}
*/


mod tests {
   #[allow(dead_code)]
   fn create() -> ::utils::Base58check {
      let table = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
      let version = [0u8];
      ::utils::Base58check::new(&table, &version)
   }

   #[test]
   fn test_serialize_b58check() {
      let base58check = create();
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e];
      let enc = "13op3it3Aaiu";
      let result = base58check.serialize(&data);
      assert_eq!(enc, result);
   }

   #[test]
   fn test_deserialize_b58check() {
      let base58check = create();
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "13op3it3Aaiu";
      let result = base58check.deserialize(enc);
      assert_matches!(result, Ok(_));
      let result = result.unwrap();
      assert_eq!(0u8, result[0]);
      assert_eq!(data, &result[1..]);
   }
}

use super::BaseN;

def_error! { Base58checkError }
macro_rules! raise_base58check_error {
   ($m:expr) => {
      try!( Err(::utils::Base58checkError::new($m, 0)) )
   }
}

pub struct Base58check {
   base_n:  BaseN,
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
   
   pub fn encode(&self, bytes: &[u8]) -> String {
      let mut check = [0u8; 32];
      {
         use ::crypto::digest::Digest;
         let mut hasher = ::crypto::digest::DHash256::new();
         hasher.input(&self.version);
         hasher.input(bytes);
         hasher.result(&mut check);
      }
      let mut v = Vec::with_capacity(self.version.len() + bytes.len() + 4);
      v.extend(self.version.iter());
      v.extend(bytes);
      v.extend(&check[0..4]);
      self.base_n.encode(v.as_slice())
   }

   pub fn decode(&self, s:&str) -> ::Result<Box<[u8]>> {
      let v = try!(self.base_n.decode(s));
      let verlen = self.version.as_ref().len();
      if v.len() < 4 + verlen {
         raise_base58check_error!(format!("deserizlied bytes is too short: {}", 4+verlen));
      }
      let len0 = v.len() - 4;
      let mut check = [0u8; 32];
      {
         use ::crypto::digest::Digest;
         let mut hasher = ::crypto::digest::DHash256::new();
         hasher.input(&v[0..len0]);
         hasher.result(&mut check);
      }
      if &v[len0..] != &check[0..4] {
         use ::utils::b2h;
         raise_base58check_error!(format!("checks are mismatch: {} but {}", b2h(&check[0..4]), b2h(&v[len0..])));
      }
      if &v[0..verlen] != self.version.as_ref() {
         use ::utils::b2h;
         raise_base58check_error!(format!("versions are mismatch: {} but {}", b2h(self.version.as_ref()), b2h(&v[0..verlen])));
      }
      Ok(v[verlen..len0].to_vec().into_boxed_slice())
   }
}

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
      let result = base58check.encode(&data);
      assert_eq!(enc, result);
   }

   #[test]
   fn test_deserialize_b58check() {
      let base58check = create();
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "13op3it3Aaiu";
      let result = base58check.decode(enc);
      assert_matches!(result, Ok(_));
      assert_eq!(data, result.unwrap().as_ref());
   }
}

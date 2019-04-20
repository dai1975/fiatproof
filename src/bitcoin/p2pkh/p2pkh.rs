#[derive(Debug,Default,Clone)]
pub struct P2PKH {
   hash: [u8; 20],
}

impl P2PKH {
   pub fn new(hash: [u8;20]) -> Self {
      Self { hash: hash }
   }
   pub fn new_with_pkh(hash: &[u8]) -> crate::Result<Self> {
      if hash.len() != 20 {
         raise_script_error!("not a 20");
      }
      let mut h = [0u8; 20];
      h.clone_from_slice(hash);
      Ok(Self::new(h))
   }
   pub fn pkh(&self) -> &[u8;20] { &self.hash }
}


pub struct Factory<'a> {
   chain: &'a ::bitcoin::ChainParams<'a>,
}

impl <'a> Factory<'a> {
   pub fn new(chain: &'a ::bitcoin::ChainParams) -> Self {
      Self {
         chain: chain
      }
   }
   pub fn create_base58check_pubkey_hash(&self) -> ::utils::Base58check {
      let t = &self.chain.base58check;
      ::utils::Base58check::new(&t.table, &t.versions.pubkey_hash)
   }
   pub fn create_base58check_script_hash(&self) -> ::utils::Base58check {
      let t = &self.chain.base58check;
      ::utils::Base58check::new(&t.table, &t.versions.script_hash)
   }
}

lazy_static! {
   pub static ref MAINNET:Factory<'static> = {
      Factory::new(&::bitcoin::presets::bitcoin_mainnet::CHAIN)
   };
}


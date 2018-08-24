
pub struct Factory<'a> {
   chain: &'a ::bitcoin::ChainParams<'a>,
}

impl <'a> Factory<'a> {
   pub fn new(chain: &'a ::bitcoin::ChainParams) -> Self {
      Self {
         chain: chain
      }
   }
   pub fn create_base58check_p2pkh(&self) -> ::utils::Base58check {
      let t = &self.chain.base58check;
      ::utils::Base58check::new(&t.table, &t.versions.p2pkh)
   }
   pub fn create_base58check_p2sh(&self) -> ::utils::Base58check {
      let t = &self.chain.base58check;
      ::utils::Base58check::new(&t.table, &t.versions.p2sh)
   }
}

lazy_static! {
   pub static ref MAINNET:Factory<'static> = {
      Factory::new(&::bitcoin::presets::bitcoin_mainnet::CHAIN)
   };
}


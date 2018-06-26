
pub struct Factory<'a> {
   chain: &'a ::bitcoin::ChainParams<'a>,
}

impl <'a> Factory<'a> {
   pub fn new(chain: &'a ::bitcoin::ChainParams) -> Self {
      Self {
         chain: chain
      }
   }
   pub fn create_base58check(&self) -> ::utils::Base58Check {
      let t = &self.chain.base58check;
      ::utils::Base58Check::new(&t.0, &t.1)
   }
}

lazy_static! {
   pub static ref BITCOIN_MAINNET:Factory<'static> = {
      Factory::new(&::bitcoin::presets::bitcoin_mainnet::CHAIN)
   };
}


#[derive(Clone)]
pub struct Chain {
   pub params: &'static ::bitcoin::ChainParams<'static>,
}

impl Chain {
   pub fn new(params: &'static ::bitcoin::ChainParams) -> Self {
      Self {
         params: params
      }
   }
   pub fn parse_address(&self, addr:&str) -> Option<::bitcoin::utils::PayTo> {
      ::bitcoin::utils::PayTo::parse_address(addr, &self.params.base58check)
   }
   pub fn create_base58check_p2pkh(&self) -> ::utils::Base58check {
      let t = &self.params.base58check;
      ::utils::Base58check::new(&t.table, &t.versions.p2pkh)
   }
   pub fn create_base58check_p2sh(&self) -> ::utils::Base58check {
      let t = &self.params.base58check;
      ::utils::Base58check::new(&t.table, &t.versions.p2sh)
   }
}

lazy_static! {
   pub static ref MAINNET:Chain = {
      Chain::new(&::bitcoin::presets::bitcoin_mainnet::CHAIN)
   };
   pub static ref TESTNET:Chain = {
      Chain::new(&::bitcoin::presets::bitcoin_testnet::CHAIN)
   };
   pub static ref REGTEST:Chain = {
      Chain::new(&::bitcoin::presets::bitcoin_regtest::CHAIN)
   };
}

pub fn get_chain(chain: &str) -> Option< Chain > {
   let chains: [&Chain; 3] = [&MAINNET, &TESTNET, &REGTEST];
   chains.into_iter().find(|c| chain == c.params.network).map(|c| (*c).clone())
}

impl ::std::fmt::Display for Chain {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
      write!(f, "{}", self.params.network)
   }
}
impl ::std::fmt::Debug for Chain {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
      write!(f, "{}", self.params.network)
   }
}


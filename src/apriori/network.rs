use super::super::{UInt256,ChainParams,ConsensusParams};

pub enum Network {
   MAIN    = 0,
   TESTNET = 1,
   REGTEST = 2,
}

pub const MAIN_ID:i32    = Network::MAIN    as i32;
pub const TESTNET_ID:i32 = Network::TESTNET as i32;
pub const REGTEST_ID:i32 = Network::REGTEST as i32;

pub const MAIN_NAME:   &'static str = "main";
pub const TESTNET_NAME:&'static str = "testnet";
pub const REGTEST_NAME:&'static str = "regtest";

use ::serialize::WithBytes;
lazy_static! {
   #[allow(dead_code)]
   pub static ref MAIN_PARAMS:ChainParams<'static> = ChainParams {
      id:   Network::MAIN as i32,
      name: MAIN_NAME,
      magic: 0xD9B4BEF9u32,
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::with_rhex("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f").unwrap(),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: 227931,
         bip34_hash: UInt256::with_rhex("000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8").unwrap(),
         pow_limit:  UInt256::with_rhex("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: false,
         pow_no_retargeting: false,
      },
   };

   #[allow(dead_code)]
   pub static ref TESTNET_PARAMS:ChainParams<'static> = ChainParams {  //testnet3
      id:   Network::TESTNET as i32,
      name: TESTNET_NAME,
      magic: 0x0709110Bu32,
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::with_rhex("000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943").unwrap(),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 51,
         majority_reject_block_outdated: 75,
         majority_window: 100,
         bip34_height: 21111,
         bip34_hash: UInt256::with_rhex("0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8").unwrap(),
         pow_limit:  UInt256::with_rhex("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: false,
      },
   };

   #[allow(dead_code)]
   pub static ref REGTEST_PARAMS:ChainParams<'static> = ChainParams {
      id:   Network::REGTEST as i32,
      name: REGTEST_NAME,
      magic: 0xDAB5BFFAu32,
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::with_rhex("0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206").unwrap(),
         subsidy_halving_interval: 150,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: -1,
         bip34_hash: UInt256::default(),
         pow_limit:  UInt256::with_rhex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: true,
      },
   };
}

pub fn get_chain_params_by_id<'a>(id : i32) -> Option<&'a ChainParams<'static> > {
   match id {
      MAIN_ID    => Some(&*MAIN_PARAMS),
      TESTNET_ID => Some(&*TESTNET_PARAMS),
      REGTEST_ID => Some(&*REGTEST_PARAMS),
      _          => None,
   }
}

pub fn get_chain_params_by_name<'a>(name : &str) -> Option<&'a ChainParams<'static> > {
   match name {
      MAIN_NAME    => Some(&*MAIN_PARAMS),
      TESTNET_NAME => Some(&*TESTNET_PARAMS),
      REGTEST_NAME => Some(&*REGTEST_PARAMS),
      _            => None,
   }
}


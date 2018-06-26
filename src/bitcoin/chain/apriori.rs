use ::bitcoin::datatypes::UInt256;
use ::serialize::FromOctets;
use super::{ChainParams,ConsensusParams};
lazy_static! {
   #[allow(dead_code)]
   pub static ref MAIN: ChainParams<'static> = ChainParams {
      coin:      "Bitcoin",
      network:   "main",
      magic: 0xD9B4BEF9u32,
      base58check: (&"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz", &[0u8]),
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::from_hex_string("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f","").unwrap(),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: 227931,
         bip34_hash: UInt256::from_hex_string("000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8", "").unwrap(),
         pow_limit:  UInt256::from_hex_string("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: false,
         pow_no_retargeting: false,
      },
   };

   #[allow(dead_code)]
   pub static ref TESTNET:ChainParams<'static> = ChainParams {  //testnet3
      coin:      "Bitcoin",
      network:   "testnet",
      magic: 0x0709110Bu32,
      base58check: (&"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz", &[0u8]),
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::from_hex_string("000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943", "").unwrap(),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 51,
         majority_reject_block_outdated: 75,
         majority_window: 100,
         bip34_height: 21111,
         bip34_hash: UInt256::from_hex_string("0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8", "").unwrap(),
         pow_limit:  UInt256::from_hex_string("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: false,
      },
   };

   #[allow(dead_code)]
   pub static ref REGTEST:ChainParams<'static> = ChainParams {
      coin:      "Bitcoin",
      network:   "regtest",
      magic: 0xDAB5BFFAu32,
      base58check: (&"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz", &[0u8]),
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::from_hex_string("0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206", "").unwrap(),
         subsidy_halving_interval: 150,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: -1,
         bip34_hash: UInt256::new_null(),
         pow_limit:  UInt256::from_hex_string("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: true,
      },
   };
}

pub fn get(name : &str) -> Option<&'static ChainParams<'static> > {
   match name {
      "main"    => Some(&*MAIN),
      "testnet" => Some(&*TESTNET),
      "regtest" => Some(&*REGTEST),
      _         => None,
   }
}


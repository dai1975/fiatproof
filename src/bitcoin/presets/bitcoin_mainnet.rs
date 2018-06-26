use ::bitcoin::datatypes::UInt256;
use ::serialize::FromOctets;
use ::bitcoin::chain::{ChainParams,ConsensusParams};

lazy_static! {
   #[allow(dead_code)]
   pub static ref CONSENSUS: ConsensusParams = ConsensusParams {
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
   };
      
   pub static ref CHAIN: ChainParams<'static> = ChainParams {
      coin:      "Bitcoin",
      network:   "main",
      magic:     0xD9B4BEF9u32,
      consensus: CONSENSUS.clone(),
      base58check: (&"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz", &[0u8]),
   };
}


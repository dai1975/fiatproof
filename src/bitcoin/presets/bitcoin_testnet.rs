use ::bitcoin::datatypes::UInt256;
use ::serialize::FromOctets;
use ::bitcoin::chainparams as cp;

lazy_static! {
   #[allow(dead_code)]
   pub static ref CHAIN: cp::Chain<'static> = cp::Chain {
      coin:        "Bitcoin",
      network:     "testnet",
      magic:       0x0709110Bu32,
      base58check: cp::Base58check {
         table: &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
         versions: cp::base58check::Versions {
            pubkey_hash: &[111],
            script_hash: &[196],
         },
      },
      consensus: cp::Consensus {
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
}


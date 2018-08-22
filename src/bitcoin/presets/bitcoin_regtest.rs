use ::bitcoin::datatypes::UInt256;
use ::bitcoin::chainparams as cp;

fn hex_to_uint256(s: &str) -> UInt256 { ::ui::bitcoin::hex_to_uint256(s).unwrap() }

lazy_static! {
   #[allow(dead_code)]
   pub static ref CHAIN: cp::Chain<'static> = cp::Chain {
      coin:        "Bitcoin",
      network:     "regtest",
      magic:       0xDAB5BFFAu32,
      base58check: cp::Base58check {
         table: &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
         versions: cp::base58check::Versions {
            pubkey_hash: &[111],
            script_hash: &[196],
         },
      },
      consensus: cp::Consensus {
         hash_genesis_block: hex_to_uint256("0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"),
         subsidy_halving_interval: 150,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: -1,
         bip34_hash: UInt256::new_null(),
         pow_limit:  hex_to_uint256("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: true,
      },
   };
}


use ::bitcoin::datatypes::UInt256;
use ::bitcoin::chainparams as cp;

fn hex_to_uint256(s: &str) -> UInt256 { ::ui::bitcoin::hex_to_uint256(s).unwrap() }

lazy_static! {
   #[allow(dead_code)]
   pub static ref CHAIN: cp::Chain<'static> = cp::Chain {
      coin:        "Bitcoin",
      network:     "main",
      magic:       0xD9B4BEF9u32,
      base58check: cp::Base58check {
         table: &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
         versions: cp::base58check::Versions {
            pubkey_hash: &[0],
            script_hash: &[5],
         },
      },
      consensus: cp::Consensus {
         hash_genesis_block: hex_to_uint256("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: 227931,
         bip34_hash: hex_to_uint256("000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8"),
         pow_limit:  hex_to_uint256("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: false,
         pow_no_retargeting: false,
      },
   };
}


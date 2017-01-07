#![feature(ptr_eq)]
#![feature(core_intrinsics)]
#![feature(associated_consts)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(specialization)]

#![feature(plugin)]
#![plugin(interpolate_idents)]


#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate num;

pub mod error;
pub use self::error::{Error, GenericError};

pub mod hexbytes;
pub use self::hexbytes::{ToHex, ToReverseHex, FromHex, WithHex, FromHexError,
                         ToBytes, FromBytes, WithBytes, FromBytesError, ToHash};

pub mod uint256;
pub use self::uint256::UInt256;

pub mod display;

pub mod structs;
pub use self::structs::{ ConsensusParams, ChainParams,
                         Transaction, LockTime, TxIn, TxOut,
                         PartialMerkleTree, MerkleBlock,
                         BlockHeader, Block, BlockLocator };
pub mod apriori;
pub use self::apriori::network::Network;
pub use self::apriori::network::{MAIN_PARAMS, TESTNET_PARAMS, REGTEST_PARAMS};
pub use self::apriori::network::{get_chain_params_by_id, get_chain_params_by_name};

pub mod crypto;
pub mod protocol;
pub mod script;

pub mod serialize;

#[test]
fn apriori_test() {
   assert!(::MAIN_PARAMS.name == ::get_chain_params_by_name("main").unwrap().name);
}



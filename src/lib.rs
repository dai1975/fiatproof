#![feature(ptr_eq)]
#![feature(associated_consts)]

#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate num;

pub mod error;
pub use error::{Error, GenericError};

pub mod uint256;
pub use uint256::UInt256;

pub mod display;

pub mod structs;
pub use structs::{ ConsensusParams, ChainParams,
                   Transaction, TxIn, TxOut,
                   PartialMerkleTree, MerkleBlock,
                   BlockHeader, Block, BlockLocator };
pub mod apriori;
pub use apriori::network::Network;
pub use apriori::network::{MAIN_PARAMS, TESTNET_PARAMS, REGTEST_PARAMS};
pub use apriori::network::{get_chain_params_by_id, get_chain_params_by_name};

pub mod protocol;
pub mod script;

#[test]
fn apriori_test() {
   assert!(::MAIN_PARAMS.name == ::get_chain_params_by_name("main").unwrap().name);
}



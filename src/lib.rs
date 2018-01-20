#![feature(core_intrinsics)]
#![feature(associated_consts)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(specialization)]
#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![feature(trace_macros)]
#![feature(slice_concat_ext)]
#![feature(try_from)]
#![feature(range_contains)]
#![feature(const_fn)]

#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
extern crate secp256k1;
extern crate streaming_iterator;
#[macro_use]
extern crate combine;

#[macro_use]
pub mod error;
pub use self::error::{Error, GenericError, ParseError};
pub type Result<T> = ::std::result::Result<T, ::Error>; 

#[macro_use]
pub mod utils;

#[macro_use]
pub mod serialize;

pub mod display;
pub mod crypto;

pub mod primitives;
pub use self::primitives::{
   UInt256, Script,
   TxOutPoint, TxIn, TxOut, Tx, LockTime,
   BlockHeader, PartialMerkleTree, MerkleBlock, Block, BlockLocator,
};

pub mod chain;
pub use self::chain::ConsensusParams;
pub use self::chain::ChainParams;

pub mod protocol;

#[macro_use]
pub mod script;
/*
pub use self::script::{Script};

#[test]
fn apriori_test() {
   assert!(::MAIN_PARAMS.name == ::get_chain_params_by_name("main").unwrap().name);
}

 */


//! Fiatproof the bitcoin library.
//!
//! ## Utility Apis
//! Utility Apis such as hexstring parser and base-n codec.
//! - [Utils API](utils/index.html)
//!
//! .
//!
//! ## Bitcoin APIs
//! Bitcoin data structure, serializer, script interpreter, and defined chains.
//!  - [Bitcoin API](bitcoin/index.html)
//!  - [Chain specific API](ui/bitcoin/chain/index.html)
//!  - [Serialize API](ui/bitcoin/serializer/index.html)
//!  - [Deserialize API](ui/bitcoin/deserializer/index.html)
//!
//! .
//!
//! ## Secp256k1 keypairs APIs
//! Secp256k1 ECDSA and its application apis.
//!  - [BIP32 API](crypto/bip32/index.html)
//!  - [Chain specific API](ui/bitcoin/chain/index.html)
//!  - [High level Secp256k1 API](ui/secp256k1/index.html)
//!
//! .
//!
//! ## Crypto digest APIs
//! Some digest apis. Provides low-leven and high-level ones.
//!  - [High level Digest API](ui/digest/index.html)
//!  - [High level HMac API](ui/hmac/index.html)
//!  - [Low level Digest API](crypto/digest/index.html)
//!  - [Low level HMac API](crypto/hmac/index.html)
//!
//! .
//!
#![allow(unused_imports)]
#![feature(core_intrinsics)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(specialization)]
#![feature(slice_patterns)]
#![feature(trace_macros)]
#![feature(slice_concat_ext)]
#![feature(const_fn)]

//#[macro_use] extern crate mashup;
#![feature(plugin)]
//#![plugin(hex_literals)]

#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
extern crate paste;
//extern crate secp256k1;
extern crate streaming_iterator;
extern crate combine;
extern crate backtrace;

#[macro_use]
pub mod error;
pub use self::error::{Error, GenericError, ParseError};
pub type Result<T> = std::result::Result<T, crate::Error>; 

pub mod utils;

pub mod crypto;

pub mod iostream;

pub mod bitcoin;

#[macro_use]
pub mod ui;



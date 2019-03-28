//!
//! BIP32 Hierarchcal Deterministic Wallet
//!
//! ## create BIP32 node
//!
//! * xprv node from seed
//!  XPrv::from_seed(Borrow<[u8]>) accepts 16bytes [u8] slice.
//!
//! ```
//!  let seed = ::fiatproof::utils::h2b("000102030405060708090a0b0c0d0e0f").unwrap();
//!  let xprv = ::fiatproof::crypto::bip32::XPrv::from_seed(seed).unwrap();
//! ```
//!
//!
//! * xpub/xprv node from xpub/xprv strings
//!
//! ```
//!  let xpub_str = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
//!  let xpub_decoder = ::fiatproof::ui::bitcoin::MAINNET.create_xpub_decoder();
//!  let xpub = xpub_decoder.decode(xpub_str).unwrap();

//!  let xprv_str = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";
//!  let xprv_decoder = ::fiatproof::ui::bitcoin::MAINNET.create_xprv_decoder();
//!  let xprv = xprv_decoder.decode(xprv_str).unwrap();
//! ```
//!
//! ## BIP32 node API
//!
//! * create coordinate xpub from xprv
//!
//! The struct of XPrv has pub properties of xpub :-)
//!
//! ```
//! let seed = ::fiatproof::utils::h2b("000102030405060708090a0b0c0d0e0f").unwrap();
//! let xprv = ::fiatproof::crypto::bip32::XPrv::from_seed(seed).unwrap();
//! let xpub = xprv.xpub;
//! ```
//! 
//! * derive from parent node
//! 
//! ```
//! let derive_index = 123;
//!
//! # let seed = ::fiatproof::utils::h2b("000102030405060708090a0b0c0d0e0f").unwrap();
//! let parent_xprv = ::fiatproof::crypto::bip32::XPrv::from_seed(seed).unwrap();
//! let child_xprv = parent_xprv.derive(derive_index).unwrap();
//!
//! let parent_xpub = parent_xprv.xpub;
//! let child_xpub = parent_xpub.derive(derive_index).unwrap();
//! assert_eq!(child_xprv.xpub, child_xpub);
//! ```
//!
//! * derive by path-string
//!
//! ```ignore
//! not yet implemented...
//! ```
//!
//! ## secret_key/public_key API
//!
//! ```ignore
//! not yet implemented...
//! ```
//!
#[macro_use]
pub mod error;
pub use self::error::{Bip32Error};

pub mod xpub;
pub use self::xpub::XPub;
pub mod xprv;
pub use self::xprv::XPrv;


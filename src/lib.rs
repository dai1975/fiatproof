#![allow(unused_imports)]
#![feature(core_intrinsics)]
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
extern crate combine;
extern crate backtrace;

#[macro_use]
pub mod error;
pub use self::error::{Error, GenericError, ParseError};
pub type Result<T> = ::std::result::Result<T, ::Error>; 

#[macro_use]
pub mod utils;

pub mod display;
pub mod crypto;

pub mod serialize;

pub mod bitcoin;

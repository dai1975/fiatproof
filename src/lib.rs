#![allow(unused_imports)]
#![feature(core_intrinsics)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(specialization)]
#![feature(slice_patterns)]
#![feature(trace_macros)]
#![feature(slice_concat_ext)]
#![feature(try_from)]
#![feature(range_contains)]
#![feature(const_fn)]

//#[macro_use] extern crate mashup;
#![feature(plugin)]
#![plugin(interpolate_idents)]
#![plugin(hex_literals)]

#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
//extern crate secp256k1;
extern crate streaming_iterator;
extern crate combine;
extern crate backtrace;

#[macro_use]
pub mod error;
pub use self::error::{Error, GenericError, ParseError};
pub type Result<T> = ::std::result::Result<T, ::Error>; 

pub mod utils;

pub mod crypto;

/*
pub mod iostream;

pub mod bitcoin;

#[macro_use]
pub mod ui;
 */


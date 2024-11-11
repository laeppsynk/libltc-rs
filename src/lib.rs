// lib.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod api;
mod error;
mod raw;

pub use api::*;

// Assert that the library version is correct
const _: () = assert!(consts::LIBLTC_VERSION_MAJOR == 1);
const _: () = assert!(consts::LIBLTC_VERSION_MINOR == 3);
const _: () = assert!(consts::LIBLTC_VERSION_MICRO == 2);

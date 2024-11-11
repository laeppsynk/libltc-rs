// lib.rs

mod api;
mod error;
mod raw;

pub use api::*;

// Assert that the library version is correct
const _: () = assert!(consts::LIBLTC_VERSION_MAJOR == 1);
const _: () = assert!(consts::LIBLTC_VERSION_MINOR == 3);
const _: () = assert!(consts::LIBLTC_VERSION_MICRO == 2);

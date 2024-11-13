// lib.rs

use api::consts;

mod api;
mod error;
mod raw;

pub mod prelude {
    pub use super::api::consts::*;
    pub use super::api::decoder::*;
    pub use super::api::encoder::*;
    pub use super::api::frame::*;
    pub use super::api::*;
}

// Assert that the library version is correct
const _: () = assert!(consts::LIBLTC_VERSION_MAJOR == 1);
const _: () = assert!(consts::LIBLTC_VERSION_MINOR == 3);
const _: () = assert!(consts::LIBLTC_VERSION_MICRO == 2);

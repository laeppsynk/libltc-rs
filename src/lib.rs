// lib.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod api;
mod error;
mod raw;

pub use api::*;

// TODO: impl  Frame
// TODO: impl Timecode methdods for accessing fields and creating

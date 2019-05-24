#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "use-bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "use-bindgen"))]
mod bindings;
#[cfg(not(feature = "use-bindgen"))]
pub use bindings::*;

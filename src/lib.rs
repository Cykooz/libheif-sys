#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(all(feature = "use-bindgen", not(docsrs)))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(any(not(feature = "use-bindgen"), docsrs))]
mod bindings;
#[cfg(any(not(feature = "use-bindgen"), docsrs))]
pub use bindings::*;

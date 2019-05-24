#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Fake using of bindings module to enable an autocomplete in IDE
#[cfg(fake)]
mod bindings;
#[cfg(fake)]
pub use bindings::*;

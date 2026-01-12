cfg_if::cfg_if! {
    if #[cfg(feature = "v1_21")] {
        mod v1_21;
        pub use v1_21::*;
    } else
    if #[cfg(feature = "v1_20")] {
        mod v1_20;
        pub use v1_20::*;
    } else
    if #[cfg(feature = "v1_19")] {
        mod v1_19;
        pub use v1_19::*;
    } else if #[cfg(feature = "v1_18")] {
        mod v1_18;
        pub use v1_18::*;
    } else if #[cfg(feature = "v1_17")] {
        mod v1_17;
        pub use v1_17::*;
    } else {
        compile_error!(
            "You MUST enable one of the crate features to specify minimal \
            supported version of 'libheif' API (e.g. v1_17)."
        );
    }
}

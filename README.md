# libheif-sys

[![github](https://img.shields.io/badge/github-Cykooz%2Flibheif--sys-8da0cb)](https://github.com/Cykooz/libheif-sys)
[![crates.io](https://img.shields.io/crates/v/libheif-sys?logo=rust)](https://crates.io/crates/libheif-sys)
[![docs.rs](https://img.shields.io/docsrs/libheif-sys?logo=docs.rs&color=66c2a5)](https://docs.rs/libheif-sys)

`libheif-sys` is a binding to [libheif](https://github.com/strukturag/libheif).

A high-level wrapper [libheif-rs](https://github.com/Cykooz/libheif-rs) is also
available.

[CHANGELOG](https://github.com/Cykooz/libheif-sys/blob/master/CHANGELOG.md)

## System dependencies

- `libheif-dev` >= 1.17.0.

## Minimal supported API version

This crate supports `libheif` versions 1.17, 1.18, 1.19, 1.20 and 1.21.

You MUST specify a minimal version of `libheif` that is required for you.
To do this, enable the corresponding feature: `v1_17`, `v1_18`, `v1_19`, `v1_20`
or `v1_21`.

Example:

```toml
[dependencies]
libheif-sys = { version = "4.0", features = ["v1_18"] }
```

There is also the `latest` feature. It always corresponds to
the maximal supported by the crate version of `libheif`.

### Linux

The crate uses `pkg-confing` to find installed `libheif` (with
help of `system-deps` crate).

You can also enable `embedded-libheif` feature to compile
`libheif v1.21.1` from embedded sources and then link it statically.

<div class="warning">

Note: Static linked version of `libheif` doesn't have statically linked
it dependencies, such as `libde256`, `libaom` and other.

</div>

### Windows

The crate uses [vcpkg crate](https://crates.io/crates/vcpkg)
to find `libheif` installed with help of `vcpkg`.

You can use [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
to install `libheif` with help of `cargo` command:

```shell
cargo vcpkg -v build
```

`cargo-vcpkg` can fetch and build a `vcpkg` installation of required
packages from scratch. It merges package requirements specified in
the `Cargo.toml` of all crates in the dependency tree.

## Example of reading and decoding HEIF-image

```rust
use std::ffi;
use std::ptr;

use libheif_sys as lh;

#[test]
fn read_and_decode_heif_file() {
    unsafe {
        lh::heif_init(ptr::null_mut());

        let ctx = lh::heif_context_alloc();
        assert!(!ctx.is_null());

        let c_name = ffi::CString::new("data/test.heif").unwrap();
        let err = lh::heif_context_read_from_file(
            ctx,
            c_name.as_ptr(),
            ptr::null()
        );
        assert_eq!(err.code, lh::heif_error_code_heif_error_Ok);

        let mut handle = ptr::null_mut();
        let err = lh::heif_context_get_primary_image_handle(
            ctx,
            &mut handle
        );
        assert_eq!(err.code, lh::heif_error_code_heif_error_Ok);
        assert!(!handle.is_null());

        let width = lh::heif_image_handle_get_width(handle);
        assert_eq!(width, 4032);
        let height = lh::heif_image_handle_get_height(handle);
        assert_eq!(height, 3024);

        let mut image = ptr::null_mut();
        let options = lh::heif_decoding_options_alloc();
        let err = lh::heif_decode_image(
            handle,
            &mut image,
            lh::heif_colorspace_heif_colorspace_RGB,
            lh::heif_chroma_heif_chroma_interleaved_RGB,
            options,
        );
        lh::heif_decoding_options_free(options);
        assert_eq!(err.code, lh::heif_error_code_heif_error_Ok);
        assert!(!image.is_null());

        let colorspace = lh::heif_image_get_colorspace(image);
        assert_eq!(
            colorspace,
            lh::heif_colorspace_heif_colorspace_RGB
        );
        let chroma_format = lh::heif_image_get_chroma_format(image);
        assert_eq!(
            chroma_format,
            lh::heif_chroma_heif_chroma_interleaved_RGB
        );
        let width = lh::heif_image_get_width(
            image,
            lh::heif_channel_heif_channel_interleaved
        );
        assert_eq!(width, 4032);
        let height = lh::heif_image_get_height(
            image,
            lh::heif_channel_heif_channel_interleaved
        );
        assert_eq!(height, 3024);

        lh::heif_context_free(ctx);

        lh::heif_deinit();
    };
}
```

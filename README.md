# libheif-sys is bindings to libheif

[CHANGELOG](https://github.com/Cykooz/libheif-sys/blob/master/CHANGELOG.md)

## System dependencies

- libheif-dev >= 1.10.0

## Example of reading and decoding of HEIF-image

```rust
use std::ffi;
use std::mem::MaybeUninit;
use std::ptr;

use libheif_sys as lh;

#[test]
unsafe fn read_and_decode_heic_file() {
    unsafe {
        let ctx = lh::heif_context_alloc();
        assert_ne!(ctx, ptr::null_mut());

        let c_name = ffi::CString::new("tests/window.heic").unwrap();
        let err = lh::heif_context_read_from_file(ctx, c_name.as_ptr(), ptr::null());
        assert_eq!(err.code, 0);

        let mut handle = MaybeUninit::<_>::uninit();
        let err = lh::heif_context_get_primary_image_handle(ctx, handle.as_mut_ptr());
        assert_eq!(err.code, 0);

        let handle = handle.assume_init();
        let width = lh::heif_image_handle_get_width(handle);
        assert_eq!(width, 4032);
        let height = lh::heif_image_handle_get_height(handle);
        assert_eq!(height, 3024);

        let options = lh::heif_decoding_options_alloc();

        let mut image = MaybeUninit::<_>::uninit();
        let err = lh::heif_decode_image(
            handle,
            image.as_mut_ptr(),
            lh::heif_colorspace_heif_colorspace_RGB,
            lh::heif_chroma_heif_chroma_444,
            options,
        );
        lh::heif_decoding_options_free(options);
        assert_eq!(err.code, 0);

        let image = image.assume_init();
        let colorspace = lh::heif_image_get_colorspace(image);
        assert_eq!(colorspace, lh::heif_colorspace_heif_colorspace_RGB);
        let chroma_format = lh::heif_image_get_chroma_format(image);
        assert_eq!(chroma_format, lh::heif_chroma_heif_chroma_444);
        let width = lh::heif_image_get_width(image, lh::heif_channel_heif_channel_R);
        assert_eq!(width, 4032);
        let height = lh::heif_image_get_height(image, lh::heif_channel_heif_channel_R);
        assert_eq!(height, 3024);

        lh::heif_context_free(ctx)
    };
}
```

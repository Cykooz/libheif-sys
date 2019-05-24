use libheif_sys::*;
use std::ptr;
use std::mem;
use std::ffi;


#[test]
fn create_heic_context() {
    unsafe {
        let ctx = heif_context_alloc();
        assert_ne!(ctx, ptr::null_mut());
        heif_context_free(ctx);
    }
}


#[test]
fn read_and_decode_heic_file() {
    unsafe {
        let ctx: *mut heif_context = heif_context_alloc();
        assert_ne!(ctx, ptr::null_mut());

        let c_name = ffi::CString::new("tests/window.heic").unwrap();
        let err: heif_error = heif_context_read_from_file(ctx, c_name.as_ptr(), ptr::null());
        assert_eq!(err.code, 0);

        let mut handle = Box::new(mem::uninitialized());
        let err: heif_error = heif_context_get_primary_image_handle(ctx, &mut *handle);
        assert_eq!(err.code, 0);

        let image_handle: *mut heif_image_handle = *handle;

        let width = heif_image_handle_get_width(image_handle);
        assert_eq!(width, 4032);
        let height = heif_image_handle_get_height(image_handle);
        assert_eq!(height, 3024);

        let options: *mut heif_decoding_options = heif_decoding_options_alloc();

        let mut image = Box::new(mem::uninitialized());
        let err = heif_decode_image(
            image_handle,
            &mut *image,
            heif_colorspace_heif_colorspace_undefined, // encoder->colorspace(has_alpha),
            heif_chroma_heif_chroma_undefined,         //encoder->chroma(has_alpha),
            options,
        );
        assert_eq!(err.code, 0);
        heif_decoding_options_free(options);

        let image: *mut heif_image = *image;

        let width = heif_image_get_width(image, heif_channel_heif_channel_Y);
        assert_eq!(width, 4032);
        let height = heif_image_get_height(image, heif_channel_heif_channel_Y);
        assert_eq!(height, 3024);

        heif_context_free(ctx)
    };
}

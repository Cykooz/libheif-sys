use std::ffi::CStr;
use std::ptr;

use libheif_sys as lh;

#[test]
fn create_heic_context() {
    unsafe {
        lh::heif_init(ptr::null_mut());
        let version = CStr::from_ptr(lh::heif_get_version()).to_string_lossy();
        assert_eq!(version, "1.18.2");

        let ctx = lh::heif_context_alloc();
        assert!(!ctx.is_null());
        lh::heif_context_free(ctx);

        lh::heif_deinit();
    }
}

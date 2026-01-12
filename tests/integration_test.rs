use std::ffi::CStr;
use std::ptr;

use libheif_sys as lh;

#[test]
fn create_heic_context() {
    let expected_version = get_version().to_string() + ".";
    unsafe {
        lh::heif_init(ptr::null_mut());
        let version = CStr::from_ptr(lh::heif_get_version()).to_string_lossy();
        assert!(version.starts_with(&expected_version));

        let ctx = lh::heif_context_alloc();
        assert!(!ctx.is_null());
        lh::heif_context_free(ctx);

        lh::heif_deinit();
    }
}

fn get_version() -> &'static str {
    if cfg!(feature = "v1_21") {
        "1.21"
    } else if cfg!(feature = "v1_20") {
        "1.20"
    } else if cfg!(feature = "v1_19") {
        "1.19"
    } else if cfg!(feature = "v1_18") {
        "1.18"
    } else if cfg!(feature = "v1_17") {
        "1.17"
    } else {
        "1.21"
    }
}

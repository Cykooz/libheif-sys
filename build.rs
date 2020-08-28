#[cfg(feature = "use-bindgen")]
use std::env;
#[cfg(feature = "use-bindgen")]
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system heif
    // shared library.
    println!("cargo:rustc-link-lib=heif");

    #[cfg(feature = "use-bindgen")]
    {
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            .generate_comments(true)
            .use_core()
            .ctypes_prefix("libc")
            .whitelist_function("heif_.*")
            .whitelist_type("heif_.*")
            // TODO: will disable size_t_is_usize in case of major release of libheif
            // https://github.com/rust-lang/rust-bindgen/issues/1671
            .size_t_is_usize(true)
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

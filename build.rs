fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Don't link with libheif in case of building documentation for docs.rs.
        return;
    }

    // Tell cargo to tell rustc to link the system heif
    // shared library.
    #[cfg(not(target_os = "windows"))]
    if let Err(err) = pkg_config::Config::new()
        .atleast_version("1.16")
        .probe("libheif")
    {
        println!("cargo:warning={}", err);
        std::process::exit(1);
    }
    #[cfg(target_os = "windows")]
    if let Err(err) = vcpkg::find_package("libheif") {
        println!("cargo:warning={}", err);
        std::process::exit(1);
    }

    #[cfg(feature = "use-bindgen")]
    {
        use std::env;
        use std::path::PathBuf;
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
            .allowlist_function("heif_.*")
            .allowlist_type("heif_.*")
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

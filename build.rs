use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
const MIN_LIBHEIF_VERSION: &str = "1.19";
#[allow(dead_code)]
const LIBHEIF_GITHUB_VERSION: &str = "v1.19.5";

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Don't link with libheif in case of building documentation for docs.rs.
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/wrapper.h");

    // Tell cargo to tell rustc to link the heif library.

    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(feature = "compile-libheif")]
        compile_libheif();

        find_libheif();
    }

    #[cfg(target_os = "windows")]
    install_libheif_by_vcpkg();

    #[cfg(feature = "use-bindgen")]
    run_bindgen();
}

#[allow(dead_code)]
fn run_command(description: &str, cmd: &mut Command) -> String {
    match cmd.output() {
        Ok(output) if !output.status.success() => {
            let std_err = String::from_utf8_lossy(&output.stderr);
            println!(
                "cargo:warning=Failed {}.\n{}\n{}",
                description, output.status, std_err
            );
            std::process::exit(1);
        }
        Err(e) => {
            println!("cargo:warning={}", e);
            std::process::exit(1);
        }
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
    }
}

#[allow(dead_code)]
fn fetch_libheif() -> PathBuf {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let libheif_dir = out_path.join("libheif");
    if libheif_dir.exists() {
        let stdout = run_command(
            "getting 'libheif' version from exists git repo",
            Command::new("git").args([
                "-C",
                libheif_dir.to_str().unwrap(),
                "describe",
                "--exact-match",
                "--tags",
            ]),
        );
        if stdout.trim() != LIBHEIF_GITHUB_VERSION {
            std::fs::remove_dir_all(&libheif_dir).unwrap();
        }
    }

    if !libheif_dir.exists() {
        run_command(
            "fetching 'libheif' from GitHub",
            Command::new("git").args([
                "clone",
                "--depth",
                "1",
                "--branch",
                LIBHEIF_GITHUB_VERSION,
                "https://github.com/strukturag/libheif.git",
                libheif_dir.to_str().unwrap(),
            ]),
        );
    }

    libheif_dir
}

#[cfg(feature = "compile-libheif")]
fn compile_libheif() {
    use std::path::PathBuf;

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let libheif_dir = fetch_libheif();

    // Patch CMakeLists.txt to disable a building `heifio` library
    // that is used for example applications.
    let cmake_lists_path = libheif_dir.join("CMakeLists.txt");
    let mut contents =
        std::fs::read_to_string(&cmake_lists_path).expect("failed to read libheif/CMakeLists.txt");
    contents = contents.replace("add_subdirectory(heifio)", "");
    std::fs::write(&cmake_lists_path, contents).expect("failed to write libheif/CMakeLists.txt");

    let mut build_config = cmake::Config::new(libheif_dir);
    build_config.out_dir(out_path.join("libheif_build"));

    // Disable some options
    for key in [
        "BUILD_SHARED_LIBS",
        "BUILD_TESTING",
        "WITH_GDK_PIXBUF",
        "WITH_EXAMPLES",
        "ENABLE_EXPERIMENTAL_FEATURES",
    ] {
        build_config.define(key, "OFF");
    }

    // Enable some options
    for key in [
        "ENABLE_PLUGIN_LOADING",
        "WITH_OPENJPH_ENCODER",
        "WITH_REDUCED_VISIBILITY",
        "WITH_LIBSHARPYUV",
        // TODO: Try to enable this in the future.
        //       Right now it is the reason of linker's errors like
        //       "undefined reference to `BrotliEncoderDestroyInstance'"
        // "WITH_UNCOMPRESSED_CODEC",
        // "WITH_HEADER_COMPRESSION",
    ] {
        build_config.define(key, "ON");
    }

    // List of encoders and decoders that have corresponding plugins
    let encoders_decoders = [
        "AOM_DECODER",
        "AOM_ENCODER",
        "DAV1D",
        "LIBDE265",
        "RAV1E",
        "SvtEnc",
        "X265",
        "JPEG_DECODER",
        "JPEG_ENCODER",
        "KVAZAAR",
        "OpenJPEG_DECODER",
        "OpenJPEG_ENCODER",
        "FFMPEG_DECODER",
        "OpenH264_DECODER",
        "UVG266",
        "VVDEC",
        "VVENC",
    ];

    // Enable encoders and decoders
    for key in encoders_decoders {
        build_config.define(format!("WITH_{}", key), "ON");
    }

    // Disable external plugins
    #[cfg(feature = "embedded-libheif-plugins")]
    const PLUGIN_OPTION_VALUE: &str = "OFF";
    // Enable external plugins
    #[cfg(not(feature = "embedded-libheif-plugins"))]
    const PLUGIN_OPTION_VALUE: &str = "ON";

    for key in encoders_decoders {
        build_config.define(format!("WITH_{}_PLUGIN", key), PLUGIN_OPTION_VALUE);
    }

    let libheif_build = build_config.build();

    let pkgconfig_dir = libheif_build
        .join("lib/pkgconfig")
        .to_string_lossy()
        .to_string();
    std::env::set_var("PKG_CONFIG_PATH", pkgconfig_dir);
}

#[cfg(not(target_os = "windows"))]
fn find_libheif() {
    let mut config = pkg_config::Config::new();
    config.atleast_version(MIN_LIBHEIF_VERSION);
    #[cfg(feature = "compile-libheif")]
    config.statik(true);

    if let Err(err) = config.probe("libheif") {
        println!("cargo:warning={}", err);
        std::process::exit(1);
    }
}

#[cfg(target_os = "windows")]
fn install_libheif_by_vcpkg() {
    let vcpkg_lib = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("libheif");
    if let Err(err) = vcpkg_lib {
        println!("cargo:warning={}", err);
        std::process::exit(1);
    }
}

#[cfg(feature = "use-bindgen")]
fn run_bindgen() {
    let mut base_builder = bindgen::Builder::default()
        .header("include/wrapper.h")
        .generate_comments(true)
        .formatter(bindgen::Formatter::Rustfmt)
        .generate_cstr(true)
        .disable_name_namespacing()
        .array_pointers_in_arguments(true)
        .ctypes_prefix("libc")
        .allowlist_function("heif_.*")
        .allowlist_type("heif_.*")
        .size_t_is_usize(true)
        .clang_args([
            "-fparse-all-comments",
            "-fretain-comments-from-system-headers",
        ]);

    // Don't derive Copy and Clone for structures with pointers.
    for struct_name in [
        "heif_plugin_info",
        "heif_decoding_options",
        "heif_encoding_options",
        "heif_property_user_description",
        "heif_reader_range_request_result",
        "heif_entity_group",
    ] {
        base_builder = base_builder.no_copy(struct_name);
    }

    // The main module
    // Finish the builder and generate the bindings.
    let bindings = base_builder
        .clone()
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.rs!");

    // Create linker_test.rs module for testing cases when not all
    // functions from *.h files are really available in libheif.
    let code = bindings.to_string();
    let mut func_names = Vec::new();
    for line in code.lines() {
        if !line.contains("pub fn ") {
            continue;
        }
        let line = line.trim();
        let res: Vec<&str> = line.split(&[' ', '(']).collect();
        if res.len() > 3 {
            if let &["pub", "fn", name] = &res[..3] {
                func_names.push(name)
            }
        }
    }

    let mut result = vec![
        "use super::*;\n\n",
        "#[test]\n",
        "fn is_all_functions_exists_in_libheif() {\n",
        "    let fn_pointers = [\n",
    ];
    for name in func_names {
        result.push("        ");
        result.push(name);
        result.push(" as *const fn(),\n")
    }
    result.extend(vec![
        "    ];\n",
        "    for pointer in fn_pointers.iter() {\n",
        "        assert!(!pointer.is_null());\n",
        "    }\n",
        "}\n",
    ]);
    let test_module = result.join("");
    let test_path = out_path.join("linker_test.rs");
    std::fs::write(&test_path, test_module).expect("Couldn't write test module!");

    let bindings = base_builder
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings without tests");
    bindings
        .write_to_file(out_path.join("bindings_wo_tests.rs"))
        .expect("Couldn't write bindings_wo_tests.rs!");
}

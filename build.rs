use std::io;
use std::path::{Path, PathBuf};

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Don't link with libheif in case of building documentation for docs.rs.
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Tell cargo to tell rustc to link the heif library.

    #[cfg(not(target_os = "windows"))]
    #[allow(unused_variables)]
    let include_paths = find_libheif();
    #[cfg(target_os = "windows")]
    #[allow(unused_variables)]
    let include_paths = Vec::new();

    #[cfg(target_os = "windows")]
    install_libheif_by_vcpkg();

    #[cfg(feature = "use-bindgen")]
    run_bindgen(&include_paths);
}

#[allow(dead_code)]
fn prepare_libheif_src() -> PathBuf {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let libheif_dir = crate_dir.join("vendor/libheif");
    let dst_dir = out_path.join("libheif");
    copy_dir_all(libheif_dir, &dst_dir).unwrap();

    // Patch CMakeLists.txt to disable a building `heifio` library
    // that is used for example applications.
    let cmake_lists_path = dst_dir.join("CMakeLists.txt");
    let mut contents =
        std::fs::read_to_string(&cmake_lists_path).expect("failed to read libheif/CMakeLists.txt");
    contents = contents.replace("add_subdirectory(heifio)", "");
    std::fs::write(&cmake_lists_path, contents).expect("failed to write libheif/CMakeLists.txt");
    dst_dir
}

#[cfg(feature = "embedded-libheif")]
fn compile_libheif() -> String {
    use std::path::PathBuf;

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let libheif_dir = prepare_libheif_src();

    let mut build_config = cmake::Config::new(libheif_dir);
    build_config.out_dir(out_path.join("libheif_build"));
    build_config.define("CMAKE_INSTALL_LIBDIR", "lib");

    // Disable some options
    for key in [
        "BUILD_SHARED_LIBS",
        "BUILD_TESTING",
        "WITH_GDK_PIXBUF",
        "WITH_EXAMPLES",
        "ENABLE_EXPERIMENTAL_FEATURES",
        "ENABLE_PLUGIN_LOADING",
    ] {
        build_config.define(key, "OFF");
    }

    // Enable some options
    for key in [
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
        "OPENJPH_ENCODER",
        "OpenJPEG_DECODER",
        "OpenJPEG_ENCODER",
        "FFMPEG_DECODER",
        "OpenH264_DECODER",
        "UVG266",
        "VVDEC",
        "VVENC",
    ];

    let disabled_enc_dec = [
        // A lot of errors like "undefined reference to `av_packet_free'" during linking
        "FFMPEG_DECODER",
    ];

    // Enable or disable encoders and decoders
    for key in encoders_decoders {
        let v = if disabled_enc_dec.contains(&key) {
            "OFF"
        } else {
            "ON"
        };
        build_config.define(format!("WITH_{}", key), v);

        // Disable external plugin
        build_config.define(format!("WITH_{}_PLUGIN", key), "OFF");
    }

    let libheif_build = build_config.build();

    libheif_build
        .join("lib/pkgconfig")
        .to_string_lossy()
        .to_string()
}

#[cfg(not(target_os = "windows"))]
fn find_libheif() -> Vec<String> {
    #[allow(unused_mut)]
    let mut config = system_deps::Config::new();

    #[cfg(feature = "embedded-libheif")]
    {
        std::env::set_var("SYSTEM_DEPS_LIBHEIF_BUILD_INTERNAL", "always");
        config = config.add_build_internal("libheif", |lib, version| {
            let pc_file_path = compile_libheif();
            system_deps::Library::from_internal_pkg_config(pc_file_path, lib, version)
        });
    }

    use system_deps::Error;

    match config.probe() {
        Ok(deps) => deps
            .all_include_paths()
            .iter()
            .filter_map(|p| p.to_str())
            .map(|p| p.to_string())
            .collect(),
        Err(err) => {
            let err_msg = match &err {
                Error::InvalidMetadata(msg) => {
                    if msg.contains("No version") && msg.contains("libheif") {
                        "You MUST enable one of the crate features to specify \
                    minimal supported version of 'libheif' API (e.g. v1_17)."
                            .to_string()
                    } else {
                        err.to_string()
                    }
                }
                _ => err.to_string(),
            };
            println!("cargo:error={err_msg}");
            std::process::exit(1);
        }
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
fn run_bindgen(include_paths: &[String]) {
    let mut base_builder = bindgen::Builder::default()
        .header("wrapper.h")
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

    for path in include_paths {
        base_builder = base_builder.clang_arg(format!("-I{path}"));
    }

    // Don't derive Copy and Clone for structures with pointers
    // and which represents shared_ptr from C++.
    for struct_name in [
        "heif_plugin_info",
        "heif_decoding_options",
        "heif_encoding_options",
        "heif_property_user_description",
        "heif_reader_range_request_result",
        "heif_entity_group",
        "heif_depth_representation_info",
        "heif_camera_extrinsic_matrix",
        "heif_track",
        "heif_raw_sequence_sample",
        "heif_track_options",
        "heif_sequence_encoding_options",
        "heif_context",
        "heif_image_handle",
        "heif_decoder_plugin",
        "heif_encoder_plugin",
        "heif_image",
        "heif_scaling_options",
        "heif_encoder",
        "heif_reading_options",
        "heif_encoder_descriptor",
        "heif_encoder_parameter",
        "heif_decoder_descriptor",
        "heif_region_item",
        "heif_region",
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

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

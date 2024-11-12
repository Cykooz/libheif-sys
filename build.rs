use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
const MIN_LIBHEIF_VERSION: &str = "1.18";
#[allow(dead_code)]
const LIBHEIF_GITHUB_VERSION: &str = "v1.18.2";

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Don't link with libheif in case of building documentation for docs.rs.
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Tell cargo to tell rustc to link the heif library.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    let mut include_dirs: Vec<String> = Vec::new();

    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(feature = "compile-libheif")]
        compile_libheif();

        include_dirs.extend(find_libheif());
    }

    #[cfg(target_os = "windows")]
    include_dirs.extend(install_libheif_by_vcpkg());

    #[cfg(feature = "use-bindgen")]
    run_bindgen(&include_dirs);
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
    let mut build_config = cmake::Config::new(libheif_dir);
    build_config.out_dir(out_path.join("libheif_build"));
    build_config.define("BUILD_SHARED_LIBS", "OFF");

    #[cfg(feature = "embedded-libheif-plugins")]
    for key in [
        "WITH_AOM_DECODER_PLUGIN",
        "WITH_AOM_ENCODER_PLUGIN",
        "WITH_DAV1D_PLUGIN",
        "WITH_FFMPEG_DECODER_PLUGIN",
        "WITH_JPEG_DECODER_PLUGIN",
        "WITH_JPEG_ENCODER_PLUGIN",
        "WITH_KVAZAAR_PLUGIN",
        "WITH_LIBDE265_PLUGIN",
        "WITH_OPENJPH_ENCODER_PLUGIN",
        "WITH_OpenJPEG_DECODER_PLUGIN",
        "WITH_OpenJPEG_ENCODER_PLUGIN",
        "WITH_RAV1E_PLUGIN",
        "WITH_SvtEnc_PLUGIN",
        "WITH_UVG266_PLUGIN",
        "WITH_VVDEC_PLUGIN",
        "WITH_VVENC_PLUGIN",
        "WITH_X265_PLUGIN",
    ] {
        build_config.define(key, "OFF");
    }

    let libheif_build = build_config.build();

    let pkgconfig_dir = libheif_build
        .join("lib/pkgconfig")
        .to_string_lossy()
        .to_string();
    std::env::set_var("PKG_CONFIG_PATH", pkgconfig_dir);
}

#[cfg(not(target_os = "windows"))]
fn find_libheif() -> Vec<String> {
    let mut config = pkg_config::Config::new();
    config.atleast_version(MIN_LIBHEIF_VERSION);
    #[cfg(feature = "compile-libheif")]
    config.statik(true);

    match config.probe("libheif") {
        Ok(library) => library
            .include_paths
            .iter()
            .map(|dir| dir.to_string_lossy().to_string())
            .collect(),
        Err(err) => {
            println!("cargo:warning={}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(target_os = "windows")]
fn install_libheif_by_vcpkg() -> Vec<String> {
    let vcpkg_lib = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("libheif");
    let mut include_dirs = Vec::new();
    match vcpkg_lib {
        Ok(lib) => {
            // https://users.rust-lang.org/t/bindgen-cant-find-included-file/62687
            use walkdir::WalkDir;
            for path in lib.include_paths {
                for subdir in WalkDir::new(path)
                    .into_iter()
                    .filter_entry(|e| e.file_type().is_dir())
                {
                    let dir = subdir.unwrap().path().to_string_lossy().to_string();
                    include_dirs.push(dir);
                }
            }
            include_dirs
        }
        Err(err) => {
            println!("cargo:warning={}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "use-bindgen")]
fn run_bindgen(include_dirs: &[String]) {
    let mut base_builder = bindgen::Builder::default()
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
    if !include_dirs.is_empty() {
        base_builder = base_builder.clang_args(
            include_dirs
                .iter()
                .map(|dir| format!("--include-directory={}", dir)),
        );
    }

    // Don't derive Copy and Clone for structures with pointers.
    for struct_name in [
        "heif_plugin_info",
        "heif_decoding_options",
        "heif_encoding_options",
        "heif_property_user_description",
    ] {
        base_builder = base_builder.no_copy(struct_name);
    }

    // The main module
    let builder = base_builder.clone().header("wrapper.hpp");
    // Finish the builder and generate the bindings.
    let bindings = builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Create linker_test.ts module for testing cases when not all
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
}

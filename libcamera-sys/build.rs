use std::{env, fs, path::PathBuf};

fn main() {
    let libcamera = match pkg_config::probe_library("libcamera") {
        Ok(lib) => Ok(lib),
        Err(e) => {
            // Older libcamera versions use camera name instead of libcamera, try that instead
            match pkg_config::probe_library("camera") {
                Ok(lib) => Ok(lib),
                // Return original error
                Err(_) => Err(e),
            }
        }
    }
    .unwrap();

    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_IPA");
    let ipa_enabled = env::var_os("CARGO_FEATURE_IPA").is_some();

    let libcamera_include_path = libcamera
        .include_paths
        .first()
        .expect("Unable to get libcamera include path");

    println!("cargo:rustc-link-lib=camera");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut c_api_headers: Vec<PathBuf> = Vec::new();
    let mut cpp_api_headers: Vec<PathBuf> = Vec::new();
    let mut c_api_sources: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir("c_api").unwrap() {
        let entry = entry.unwrap();

        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        match entry.path().extension().and_then(|s| s.to_str()) {
            Some("h") => c_api_headers.push(entry.path()),
            Some("hpp") => cpp_api_headers.push(entry.path()),
            Some("cpp") => c_api_sources.push(entry.path()),
            _ => {}
        }
    }

    for file in c_api_headers
        .iter()
        .chain(cpp_api_headers.iter())
        .chain(c_api_sources.iter())
    {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .files(c_api_sources)
        .include(libcamera_include_path)
        .include(
            libcamera_include_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| libcamera_include_path.to_path_buf()),
        )
        .compile("camera_c_api");

    // C bindings
    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", libcamera_include_path.display()))
        .constified_enum_module("libcamera_.*")
        .allowlist_function("libcamera_.*")
        .allowlist_var("LIBCAMERA_.*")
        .allowlist_var(".*LIBCAMERA_VERSION.*")
        .allowlist_type("libcamera_.*");
    if let Some(parent) = libcamera_include_path.parent() {
        builder = builder.clang_arg(format!("-I{}", parent.display()));
    }
    for header in c_api_headers {
        builder = builder.header(header.to_str().unwrap());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // CPP bindings
    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", libcamera_include_path.display()))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .allowlist_type(".*controls.*")
        .allowlist_type(".*properties.*");
    if let Some(parent) = libcamera_include_path.parent() {
        builder = builder.clang_arg(format!("-I{}", parent.display()));
    }
    for header in cpp_api_headers {
        builder = builder.header(header.to_str().unwrap());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings_cpp.rs"))
        .expect("Couldn't write bindings!");

    if ipa_enabled {
        let ipa_dir = ["ipa", "libcamera/ipa"]
            .iter()
            .map(|suffix| libcamera_include_path.join(suffix))
            .find(|p| p.exists())
            .unwrap_or_else(|| {
                panic!(
                    "feature \"ipa\" enabled but IPA headers were not found under {} (looked for ipa/ and libcamera/ipa)",
                    libcamera_include_path.display()
                )
            });

        let mut ipa_headers: Vec<PathBuf> = Vec::new();
        for required in ["ipa_controls.h", "ipa_interface.h", "ipa_module_info.h"] {
            let path = ipa_dir.join(required);
            println!("cargo:rerun-if-changed={}", path.display());
            if !path.exists() {
                panic!(
                    "feature \"ipa\" enabled but required header {} was not found (looked at {})",
                    required,
                    path.display()
                );
            }
            ipa_headers.push(path);
        }

        if let Ok(entries) = fs::read_dir(&ipa_dir) {
            let mut generated_headers: Vec<PathBuf> = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                if let Some(name) = path.file_name().and_then(|f| f.to_str()) {
                    if name.ends_with("_ipa_interface.h") || name == "core_ipa_interface.h" {
                        generated_headers.push(path);
                    }
                }
            }

            generated_headers.sort();
            generated_headers.dedup();
            for header in generated_headers {
                println!("cargo:rerun-if-changed={}", header.display());
                ipa_headers.push(header);
            }
        }

        let mut builder = bindgen::Builder::default()
            .clang_arg(format!("-I{}", libcamera_include_path.display()))
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-std=c++17")
            .allowlist_type(".*IPA.*")
            .allowlist_type("ipa_.*")
            .allowlist_var(".*IPA.*")
            .allowlist_var("ipa_.*")
            .allowlist_function(".*IPA.*")
            .opaque_type("std::.*")
            .layout_tests(false);
        if let Some(parent) = libcamera_include_path.parent() {
            builder = builder.clang_arg(format!("-I{}", parent.display()));
        }

        for header in ipa_headers {
            builder = builder.header(header.to_str().unwrap());
        }

        let bindings = builder
            .generate()
            .expect("Unable to generate IPA bindings");
        bindings
            .write_to_file(out_path.join("bindings_ipa.rs"))
            .expect("Couldn't write IPA bindings!");
    }
}

use core::panic;
use std::{
    env,
    path::{Path, PathBuf},
    fs,
};

use semver::{Comparator, Op, Version};

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

    let libcamera_version = match Version::parse(&libcamera.version) {
        Ok(v) => v,
        Err(e) => {
            panic!("bad version from pkgconfig, {e:?}")
        }
    };

    let versioned_files = Path::new("versioned_files");
    let mut candidates = std::fs::read_dir(versioned_files)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let version = Version::parse(path.file_name()?.to_str()?).ok()?;

            Some((version, path))
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable_by_key(|(version, _)| version.clone());

    // Filter to only compatible versions
    let matching = candidates.iter().filter(|(candidate, _)| {
        #[cfg(feature = "libcamera_semver_versioning")]
        let op = Op::Caret;
        #[cfg(not(feature = "libcamera_semver_versioning"))]
        let op = Op::Exact;

        let comparator = Comparator {
            op,
            major: candidate.major,
            minor: Some(candidate.minor),
            patch: Some(candidate.patch),
            pre: Default::default(),
        };

        comparator.matches(&libcamera_version)
    });

    // And take the most recent compatible version
    let (_, selected_version) = match matching.max_by_key(|(version, _)| version.clone()) {
        Some(v) => v,
        None => panic!(
            "Unsupported version of libcamera detected: {libcamera_version}\nsupported versions are: \n{}",
            candidates
                .iter()
                .map(|(v, _)| format!("\t{v}"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    for file in ["controls.rs", "properties.rs"] {
        std::fs::copy(selected_version.join(file), out_path.join(file)).unwrap();
        print!(
            "cargo:rerun-if-changed={}",
            selected_version.join(file).to_string_lossy()
        );
    }

    // Generate vendor feature flags from libcamera's generated control_ids.h
    let control_ids_header = libcamera
        .include_paths
        .first()
        .map(|p| p.join("libcamera/control_ids.h"))
        .expect("Unable to get libcamera include path");
    let header_contents = fs::read_to_string(&control_ids_header)
        .expect("Failed to read libcamera/control_ids.h");
    let mut feature_consts = String::new();
    for line in header_contents.lines() {
        if let Some(rest) = line.trim().strip_prefix("#define LIBCAMERA_HAS_") {
            let name = rest.split_whitespace().next().unwrap_or("").trim();
            if name.is_empty() {
                continue;
            }
            feature_consts.push_str(&format!("pub const LIBCAMERA_HAS_{}: bool = true;\n", name));
        }
    }
    let features_rs = out_path.join("vendor_features.rs");
    fs::write(&features_rs, feature_consts).expect("Failed to write vendor_features.rs");
    println!("cargo:rerun-if-changed={}", control_ids_header.to_string_lossy());
}

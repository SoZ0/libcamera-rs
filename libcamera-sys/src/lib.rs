#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// this is due to rust-lang/rust-bindgen#1651
#![allow(deref_nullptr)]
// libcamera documentation is incorrectly interpreted as rust code blocks
#![allow(rustdoc::invalid_rust_codeblocks)]
// Generated bindings can trigger these clippy/FFI lints; they are safe to suppress at the crate root.
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::unnecessary_cast)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings_cpp.rs"));

#[cfg(feature = "ipa")]
pub mod ipa {
    include!(concat!(env!("OUT_DIR"), "/bindings_ipa.rs"));
}

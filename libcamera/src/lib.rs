//! Safe Rust bindings for libcamera.
//!
//! Notable helper modules:
//! - [`formats`](crate::formats): generated pixel format constants matching `libcamera::formats`.
//! - [`controls`](crate::controls) and [`properties`](crate::properties): generated identifiers for the installed libcamera version.
#![warn(rust_2018_idioms)]

pub mod camera;
pub mod camera_manager;
pub mod color_space;
pub mod control;
pub mod control_value;
pub mod fence;
pub mod formats;
pub mod framebuffer;
pub mod framebuffer_allocator;
pub mod framebuffer_map;
pub mod geometry;
#[cfg(feature = "ipa")]
pub mod ipa;
pub mod logging;
pub mod pixel_format;
pub mod request;
pub mod stream;
pub mod transform;
pub mod utils;
pub mod vendor_features;
pub mod version;

#[cfg(all(feature = "vendor_rpi", not(libcamera_has_vendor_controls)))]
compile_error!("feature \"vendor_rpi\" requires libcamera headers that define vendor controls (LIBCAMERA_HAS_*).");
#[cfg(all(feature = "vendor_draft", not(libcamera_has_vendor_controls)))]
compile_error!("feature \"vendor_draft\" requires libcamera headers that define vendor controls (LIBCAMERA_HAS_*).");

mod generated;
pub use generated::*;

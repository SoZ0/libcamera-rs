//! Pixel format constants generated from the installed `libcamera/formats.h`.
//!
//! The names and values in this module mirror `libcamera::formats` for the
//! libcamera version detected at build time. Use these to avoid hand-rolling
//! fourcc/modifier pairs:
//!
//! ```
//! use libcamera::formats;
//! use libcamera::pixel_format::PixelFormat;
//!
//! let fmt: PixelFormat = formats::NV12;
//! let info = fmt.info().expect("pixel format info available");
//! assert_eq!(fmt.to_string(), info.name);
//! ```
include!(concat!(env!("OUT_DIR"), "/formats.rs"));

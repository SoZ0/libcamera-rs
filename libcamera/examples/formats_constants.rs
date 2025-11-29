//! Show the generated libcamera pixel-format constants and derived layout info.
use libcamera::{formats, geometry::Size};

fn main() {
    let fmt = formats::NV12;
    println!("Using constant formats::NV12 => {fmt:?}");

    let size = Size {
        width: 640,
        height: 480,
    };
    if let Some(info) = fmt.info() {
        println!(
            "name={} bits_per_pixel={} planes={} pixels_per_group={}",
            info.name,
            info.bits_per_pixel,
            info.planes.len(),
            info.pixels_per_group
        );
        println!(
            "frame size for {}x{} (align=0): {} bytes",
            size.width,
            size.height,
            fmt.frame_size(size, 0)
        );
    } else {
        eprintln!("PixelFormatInfo unavailable; ensure libcamera headers are discoverable.");
    }
}

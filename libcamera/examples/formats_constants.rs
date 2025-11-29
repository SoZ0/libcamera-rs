//! Demonstrates using generated formats::* PixelFormat constants instead of hard-coded fourcc strings.
use libcamera::formats;

fn main() {
    let formats = [
        ("MJPEG", formats::MJPEG),
        ("YUYV", formats::YUYV),
        ("NV12", formats::NV12),
        ("R8", formats::R8),
        ("XRGB8888", formats::XRGB8888),
        ("SRGGB10_CSI2P", formats::SRGGB10_CSI2P),
    ];

    println!("Known formats from libcamera::formats:");
    for (name, fmt) in formats {
        println!(
            "{name:>12}: fourcc=0x{:#08x} modifier=0x{:#016x} has_modifier={}",
            fmt.fourcc(),
            fmt.modifier(),
            fmt.has_modifier()
        );
    }
}

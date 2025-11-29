//! Show cloning and adjusting color spaces without mutating the original.
use libcamera::{color_space::ColorSpace, pixel_format::PixelFormat};

fn main() {
    let src = ColorSpace::rec709();
    let pf = PixelFormat::parse("YUYV").expect("parse");
    let adjusted = src.with_adjusted_for_format(pf);
    println!("source color space: {}", src);
    match adjusted {
        Some(cs) => println!("adjusted for {pf:?}: {cs}"),
        None => println!("color space not valid for format {pf:?}"),
    }
}

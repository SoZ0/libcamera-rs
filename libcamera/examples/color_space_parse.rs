//! Parse and adjust color spaces and pixel formats from strings.
use libcamera::{
    color_space::ColorSpace,
    pixel_format::PixelFormat,
};

fn main() {
    let cs_str = "Smpte170m/Rec709/Full";
    let pf_str = "YUYV";

    let mut cs = ColorSpace::from_string(cs_str).expect("parse color space");
    let pf = PixelFormat::from_str(pf_str).expect("parse pixel format");

    println!("Parsed color space: {}", cs.to_string());
    println!("Parsed pixel format: {:?}", pf);

    if cs.adjust_for_format(pf) {
        println!("Adjusted color space for {}: {}", pf_str, cs.to_string());
    } else {
        println!("Color space {} could not be adjusted for {}", cs_str, pf_str);
    }
}

//! Quick look at the core IPA bindings exposed via the `ipa` feature.
use libcamera::ipa;
use std::mem::size_of;

fn main() {
    println!("IPA controls format version: {}", ipa::IPA_CONTROLS_FORMAT_VERSION);
    println!("IPA module API version:     {}", ipa::IPA_MODULE_API_VERSION);
    println!();

    println!("Core IPA struct sizes (bytes):");
    println!("  IPAInterface:          {}", size_of::<ipa::libcamera_IPAInterface>());
    println!(
        "  IPACameraSensorInfo:   {}",
        size_of::<ipa::libcamera_IPACameraSensorInfo>()
    );
    println!("  IPABuffer:             {}", size_of::<ipa::libcamera_IPABuffer>());
    println!("  IPASettings:           {}", size_of::<ipa::libcamera_IPASettings>());
    println!("  IPAStream:             {}", size_of::<ipa::libcamera_IPAStream>());
    println!(
        "  FrameBuffer::Plane:    {}",
        size_of::<ipa::libcamera_FrameBuffer_Plane>()
    );

    println!();
    println!(
        "Pipeline-specific IPA interfaces (e.g. ipu3/soft) live under `libcamera::ipa::*` \
         when their generated headers are installed."
    );
}

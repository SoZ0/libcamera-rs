use libcamera::vendor_features::flat;

fn main() {
    println!("Vendor feature flags available:");

    println!(
        "  {:<60} {}",
        "LIBCAMERA_HAS_LIBCAMERA_VENDOR_CONTROLS_WDR_MODE",
        flat::LIBCAMERA_HAS_LIBCAMERA_VENDOR_CONTROLS_WDR_MODE
    );
    println!(
        "  {:<60} {}",
        "LIBCAMERA_HAS_LIBCAMERA_VENDOR_CONTROLS_LENS_DEWARP_ENABLE",
        flat::LIBCAMERA_HAS_LIBCAMERA_VENDOR_CONTROLS_LENS_DEWARP_ENABLE
    );
    println!(
        "  {:<60} {}",
        "LIBCAMERA_HAS_DRAFT_VENDOR_CONTROLS",
        flat::LIBCAMERA_HAS_DRAFT_VENDOR_CONTROLS
    );
    println!(
        "  {:<60} {}",
        "LIBCAMERA_HAS_DEBUG_VENDOR_CONTROLS",
        flat::LIBCAMERA_HAS_DEBUG_VENDOR_CONTROLS
    );
}

//! Configure a stream using generated pixel format constants and inspect the result.
use libcamera::{camera_manager::CameraManager, formats, geometry::Size, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().expect("camera manager");
    let cameras = mgr.cameras();
    let Some(cam) = cameras.get(0) else {
        eprintln!("No cameras available");
        return;
    };
    let mut cam = cam.acquire().expect("acquire");

    let mut config = cam
        .generate_configuration(&[StreamRole::ViewFinder])
        .expect("generate config");

    if let Some(mut cfg) = config.get_mut(0) {
        cfg.set_pixel_format(formats::NV12);
        cfg.set_size(Size::new(640, 480));
    }

    let status = config.validate();
    println!("validate status: {status:?}");
    cam.configure(&mut config).expect("configure");
    if let Some(cfg) = config.get(0) {
        println!(
            "configured: {:?}, stride={} frame_size={}",
            cfg,
            cfg.get_stride(),
            cfg.get_frame_size()
        );
    }
}

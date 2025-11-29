use std::time::Duration;

use libcamera::camera_manager::CameraManager;

fn main() {
    let mut mgr = CameraManager::new().expect("camera manager");

    mgr.on_camera_added(|cam| {
        println!("Camera added: {}", cam.id());
    });

    mgr.on_camera_removed(|cam| {
        println!("Camera removed: {}", cam.id());
    });

    println!("Waiting for hotplug events. Press Ctrl+C to exit.");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

use crate::camera::Orientation;
use libcamera_sys::*;

/// 2D plane transform matching libcamera::Transform.
#[derive(Clone, Copy, Debug)]
pub struct Transform(pub libcamera_transform_t);

impl Transform {
    pub fn identity() -> Self {
        Transform(unsafe { libcamera_transform_identity() })
    }

    /// Construct from rotation degrees, optionally applying hflip.
    pub fn from_rotation(angle: i32, hflip: bool) -> Option<Self> {
        let mut success = false;
        let t = unsafe { libcamera_transform_from_rotation(angle, hflip, &mut success) };
        if success {
            Some(Transform(t))
        } else {
            None
        }
    }

    pub fn inverse(self) -> Self {
        Transform(unsafe { libcamera_transform_inverse(self.0) })
    }

    pub fn combine(self, other: Transform) -> Self {
        Transform(unsafe { libcamera_transform_combine(self.0, other.0) })
    }

    pub fn to_string_repr(self) -> String {
        unsafe {
            let ptr = libcamera_transform_to_string(self.0);
            if ptr.is_null() {
                return String::new();
            }
            let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
            libc::free(ptr.cast());
            s
        }
    }
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_repr())
    }
}

impl Transform {
    /// Compute the transform between two orientations (equivalent to libcamera Orientation division).
    pub fn between_orientations(from: Orientation, to: Orientation) -> Self {
        Transform(unsafe { libcamera_transform_between_orientations(from.into(), to.into()) })
    }
}

pub fn apply_transform_to_orientation(orientation: Orientation, transform: Transform) -> Orientation {
    unsafe {
        libcamera_transform_apply_orientation(orientation.into(), transform.0)
            .try_into()
            .unwrap()
    }
}

#include "transform.h"
#include <cstring>

extern "C" {

libcamera_transform_t libcamera_transform_identity() {
    return libcamera::Transform::Identity;
}

libcamera_transform_t libcamera_transform_from_rotation(int angle, bool hflip, bool *success) {
    bool ok = false;
    libcamera::Transform t = libcamera::transformFromRotation(angle, &ok);
    if (!ok) {
        if (success)
            *success = false;
        return libcamera::Transform::Identity;
    }
    if (hflip)
        t = libcamera::Transform::HFlip * t;
    if (success)
        *success = ok;
    return t;
}

libcamera_transform_t libcamera_transform_inverse(libcamera_transform_t transform) {
    return -transform;
}

libcamera_transform_t libcamera_transform_combine(libcamera_transform_t a, libcamera_transform_t b) {
    return a * b;
}

char *libcamera_transform_to_string(libcamera_transform_t transform) {
    return ::strdup(libcamera::transformToString(transform));
}

}

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

libcamera_transform_t libcamera_transform_between_orientations(libcamera_orientation_t from, libcamera_orientation_t to) {
    return from / to;
}

libcamera_orientation_t libcamera_transform_apply_orientation(libcamera_orientation_t orientation, libcamera_transform_t transform) {
    return orientation * transform;
}

libcamera_orientation_t libcamera_orientation_from_rotation(int angle, bool *success) {
    bool ok = false;
    libcamera_orientation_t ori = libcamera::orientationFromRotation(angle, &ok);
    if (success)
        *success = ok;
    return ori;
}

libcamera_transform_t libcamera_transform_hflip() {
    return libcamera::Transform::HFlip;
}

libcamera_transform_t libcamera_transform_vflip() {
    return libcamera::Transform::VFlip;
}

libcamera_transform_t libcamera_transform_transpose() {
    return libcamera::Transform::Transpose;
}

libcamera_transform_t libcamera_transform_or(libcamera_transform_t a, libcamera_transform_t b) {
    return a | b;
}

libcamera_transform_t libcamera_transform_and(libcamera_transform_t a, libcamera_transform_t b) {
    return a & b;
}

libcamera_transform_t libcamera_transform_xor(libcamera_transform_t a, libcamera_transform_t b) {
    return a ^ b;
}

libcamera_transform_t libcamera_transform_not(libcamera_transform_t t) {
    return ~t;
}

}

#ifndef __LIBCAMERA_C_TRANSFORM__
#define __LIBCAMERA_C_TRANSFORM__

#ifdef __cplusplus
#include "camera.h"
#include <libcamera/transform.h>

typedef libcamera::Transform libcamera_transform_t;

extern "C" {
#else
typedef struct libcamera_transform libcamera_transform_t;
#endif

libcamera_transform_t libcamera_transform_identity();
libcamera_transform_t libcamera_transform_from_rotation(int angle, bool hflip, bool *success);
libcamera_transform_t libcamera_transform_inverse(libcamera_transform_t transform);
libcamera_transform_t libcamera_transform_combine(libcamera_transform_t a, libcamera_transform_t b);
char *libcamera_transform_to_string(libcamera_transform_t transform);
libcamera_transform_t libcamera_transform_between_orientations(libcamera_orientation_t from, libcamera_orientation_t to);
libcamera_orientation_t libcamera_transform_apply_orientation(libcamera_orientation_t orientation, libcamera_transform_t transform);

#ifdef __cplusplus
}
#endif

#endif

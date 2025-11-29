#ifndef __LIBCAMERA_C_PIXEL_FORMAT_INFO__
#define __LIBCAMERA_C_PIXEL_FORMAT_INFO__

#include "pixel_format.h"

#ifdef __cplusplus
#include <libcamera/internal/formats.h>

typedef struct libcamera_pixel_format_info {
    const char *name;
    libcamera_pixel_format_t format;
    unsigned int bits_per_pixel;
    unsigned int colour_encoding;
    bool packed;
    unsigned int pixels_per_group;
    struct {
        unsigned int bytes_per_group;
        unsigned int vertical_sub_sampling;
    } planes[3];
    unsigned int num_planes;
} libcamera_pixel_format_info_t;

extern "C" {
#else
typedef struct libcamera_pixel_format_info libcamera_pixel_format_info_t;
#endif

bool libcamera_pixel_format_info(const libcamera_pixel_format_t *format, libcamera_pixel_format_info_t *out);

#ifdef __cplusplus
}
#endif

#endif

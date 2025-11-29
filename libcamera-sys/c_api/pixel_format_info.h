#ifndef __LIBCAMERA_C_PIXEL_FORMAT_INFO__
#define __LIBCAMERA_C_PIXEL_FORMAT_INFO__

#include <stdbool.h>
#include "pixel_format.h"
#include "geometry.h"

struct libcamera_pixel_format_info {
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
    uint32_t v4l2_formats[8];
    unsigned int v4l2_format_count;
};

typedef struct libcamera_pixel_format_info libcamera_pixel_format_info_t;

#ifdef __cplusplus
extern "C" {
#endif

bool libcamera_pixel_format_info(const libcamera_pixel_format_t *format, libcamera_pixel_format_info_t *out);
unsigned int libcamera_pixel_format_info_stride(const libcamera_pixel_format_t *format, unsigned int width, unsigned int plane, unsigned int align);
unsigned int libcamera_pixel_format_info_plane_size(const libcamera_pixel_format_t *format, const libcamera_size_t *size, unsigned int plane, unsigned int align);
unsigned int libcamera_pixel_format_info_frame_size(const libcamera_pixel_format_t *format, const libcamera_size_t *size, unsigned int align);

#ifdef __cplusplus
}
#endif

#endif

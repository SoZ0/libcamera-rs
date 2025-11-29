#include "pixel_format_info.h"
#include "geometry.h"

#include <algorithm>
#include <cstring>

#if __has_include(<libcamera/internal/formats.h>)
#define LIBCAMERA_RS_HAS_INTERNAL_FORMATS 1
#include <libcamera/internal/formats.h>
#endif

extern "C" {

bool libcamera_pixel_format_info(const libcamera_pixel_format_t *format, libcamera_pixel_format_info_t *out) {
#ifdef LIBCAMERA_RS_HAS_INTERNAL_FORMATS
    if (!format || !out)
        return false;
    const auto &info = libcamera::PixelFormatInfo::info(*format);
    if (!info.isValid())
        return false;

    out->name = info.name;
    out->format = info.format;
    out->bits_per_pixel = info.bitsPerPixel;
    out->colour_encoding = static_cast<unsigned int>(info.colourEncoding);
    out->packed = info.packed;
    out->pixels_per_group = info.pixelsPerGroup;
    out->num_planes = info.numPlanes();
    for (unsigned int i = 0; i < info.planes.size(); ++i) {
        out->planes[i].bytes_per_group = info.planes[i].bytesPerGroup;
        out->planes[i].vertical_sub_sampling = info.planes[i].verticalSubSampling;
    }
    out->v4l2_format_count = std::min<unsigned int>(info.v4l2Formats.size(), 8);
    for (unsigned int i = 0; i < out->v4l2_format_count; ++i) {
        out->v4l2_formats[i] = info.v4l2Formats[i];
    }
    return true;
#else
    (void)format;
    (void)out;
    return false;
#endif
}

unsigned int libcamera_pixel_format_info_stride(const libcamera_pixel_format_t *format, unsigned int width, unsigned int plane, unsigned int align) {
#ifdef LIBCAMERA_RS_HAS_INTERNAL_FORMATS
    const auto &info = libcamera::PixelFormatInfo::info(*format);
    if (!info.isValid() || plane >= info.numPlanes())
        return 0;
    return info.stride(width, plane, align);
#else
    (void)format;
    (void)width;
    (void)plane;
    (void)align;
    return 0;
#endif
}

unsigned int libcamera_pixel_format_info_plane_size(const libcamera_pixel_format_t *format, const libcamera_size_t *size, unsigned int plane, unsigned int align) {
#ifdef LIBCAMERA_RS_HAS_INTERNAL_FORMATS
    if (!size)
        return 0;
    const auto &info = libcamera::PixelFormatInfo::info(*format);
    if (!info.isValid() || plane >= info.numPlanes())
        return 0;
    libcamera::Size s(size->width, size->height);
    return info.planeSize(s, plane, align);
#else
    (void)format;
    (void)size;
    (void)plane;
    (void)align;
    return 0;
#endif
}

unsigned int libcamera_pixel_format_info_frame_size(const libcamera_pixel_format_t *format, const libcamera_size_t *size, unsigned int align) {
#ifdef LIBCAMERA_RS_HAS_INTERNAL_FORMATS
    if (!size)
        return 0;
    const auto &info = libcamera::PixelFormatInfo::info(*format);
    if (!info.isValid())
        return 0;
    libcamera::Size s(size->width, size->height);
    return info.frameSize(s, align);
#else
    (void)format;
    (void)size;
    (void)align;
    return 0;
#endif
}

}

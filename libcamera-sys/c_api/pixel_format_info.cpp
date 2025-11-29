#include "pixel_format_info.h"

#include <cstring>
#include <libcamera/internal/formats.h>

extern "C" {

bool libcamera_pixel_format_info(const libcamera_pixel_format_t *format, libcamera_pixel_format_info_t *out) {
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
    return true;
}

}

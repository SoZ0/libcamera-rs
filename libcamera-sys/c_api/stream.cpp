#include "stream.h"

#include <libcamera/libcamera.h>

extern "C" {

libcamera_pixel_formats_t *libcamera_stream_formats_pixel_formats(const libcamera_stream_formats_t* formats) {
    return new libcamera_pixel_formats_t(std::move(formats->pixelformats()));
}

libcamera_sizes_t *libcamera_stream_formats_sizes(const libcamera_stream_formats_t* formats, const libcamera_pixel_format_t *pixel_format) {
    return new libcamera_sizes_t(std::move(formats->sizes(*pixel_format)));
}

libcamera_size_range_t libcamera_stream_formats_range(const libcamera_stream_formats_t* formats, const libcamera_pixel_format_t *pixel_format) {
    return formats->range(*pixel_format);
}

void libcamera_stream_configuration_destroy(libcamera_stream_configuration_t *config);
libcamera_pixel_format_t *libcamera_stream_configuration_pixel_format(libcamera_stream_configuration_t *config);
libcamera_size_t *libcamera_stream_configuration_size(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_stride(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_frame_size(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_buffer_count(libcamera_stream_configuration_t *config);
unsigned int *libcamera_stream_configuration_color_space(libcamera_stream_configuration_t *config);

const libcamera_stream_formats_t *libcamera_stream_configuration_formats(const libcamera_stream_configuration_t *config) {
    return &config->formats();
}

}

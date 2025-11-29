#include "color_space.h"

extern "C" {

libcamera_color_space_t libcamera_color_space_make(enum libcamera_color_space_primaries primaries,
                                                   enum libcamera_color_space_transfer_function tf,
                                                   enum libcamera_color_space_ycbcr_encoding ycbcr,
                                                   enum libcamera_color_space_range range) {
    return libcamera::ColorSpace{
        static_cast<libcamera::ColorSpace::Primaries>(primaries),
        static_cast<libcamera::ColorSpace::TransferFunction>(tf),
        static_cast<libcamera::ColorSpace::YcbcrEncoding>(ycbcr),
        static_cast<libcamera::ColorSpace::Range>(range),
    };
}

}

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

libcamera_color_space_t libcamera_color_space_raw() {
    return libcamera::ColorSpace::Raw;
}

libcamera_color_space_t libcamera_color_space_srgb() {
    return libcamera::ColorSpace::Srgb;
}

libcamera_color_space_t libcamera_color_space_sycc() {
    return libcamera::ColorSpace::Sycc;
}

libcamera_color_space_t libcamera_color_space_smpte170m() {
    return libcamera::ColorSpace::Smpte170m;
}

libcamera_color_space_t libcamera_color_space_rec709() {
    return libcamera::ColorSpace::Rec709;
}

libcamera_color_space_t libcamera_color_space_rec2020() {
    return libcamera::ColorSpace::Rec2020;
}

}

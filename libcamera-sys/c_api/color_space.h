#ifndef __LIBCAMERA_C_COLOR_SPACE__
#define __LIBCAMERA_C_COLOR_SPACE__

enum libcamera_color_space_primaries {
    LIBCAMERA_COLOR_SPACE_PRIMARIES_RAW,
    LIBCAMERA_COLOR_SPACE_PRIMARIES_SMPTE170M,
    LIBCAMERA_COLOR_SPACE_PRIMARIES_REC709,
    LIBCAMERA_COLOR_SPACE_PRIMARIES_REC2020,
};

enum libcamera_color_space_transfer_function {
    LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_LINEAR,
    LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_SRGB,
    LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_REC709,
};

enum libcamera_color_space_ycbcr_encoding {
    LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_NONE,
    LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC601,
    LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC709,
    LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC2020,
};

enum libcamera_color_space_range {
    LIBCAMERA_COLOR_SPACE_RANGE_FULL,
    LIBCAMERA_COLOR_SPACE_RANGE_LIMITED,
};

struct libcamera_color_space {
    enum libcamera_color_space_primaries primaries;
    enum libcamera_color_space_transfer_function transfer_function;
    enum libcamera_color_space_ycbcr_encoding ycbcr_encoding;
    enum libcamera_color_space_range range;
};

#ifdef __cplusplus
#include <libcamera/color_space.h>

static_assert(sizeof(struct libcamera_color_space) == sizeof(libcamera::ColorSpace));

typedef libcamera::ColorSpace libcamera_color_space_t;

extern "C" {
#else
typedef struct libcamera_color_space libcamera_color_space_t;
#endif

libcamera_color_space_t libcamera_color_space_make(enum libcamera_color_space_primaries primaries,
                                                   enum libcamera_color_space_transfer_function tf,
                                                   enum libcamera_color_space_ycbcr_encoding ycbcr,
                                                   enum libcamera_color_space_range range);

libcamera_color_space_t libcamera_color_space_raw();
libcamera_color_space_t libcamera_color_space_srgb();
libcamera_color_space_t libcamera_color_space_sycc();
libcamera_color_space_t libcamera_color_space_smpte170m();
libcamera_color_space_t libcamera_color_space_rec709();
libcamera_color_space_t libcamera_color_space_rec2020();

#ifdef __cplusplus
}
#endif

#endif

use libcamera_sys::*;

/// Color primaries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primaries {
    Raw,
    Smpte170m,
    Rec709,
    Rec2020,
}

/// Transfer function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferFunction {
    Linear,
    Srgb,
    Rec709,
}

/// YCbCr encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YcbcrEncoding {
    None,
    Rec601,
    Rec709,
    Rec2020,
}

/// Color range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Range {
    Full,
    Limited,
}

/// Represents `libcamera::ColorSpace`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorSpace {
    pub primaries: Primaries,
    pub transfer_function: TransferFunction,
    pub ycbcr_encoding: YcbcrEncoding,
    pub range: Range,
}

impl ColorSpace {
    pub const fn new(
        primaries: Primaries,
        transfer_function: TransferFunction,
        ycbcr_encoding: YcbcrEncoding,
        range: Range,
    ) -> Self {
        Self {
            primaries,
            transfer_function,
            ycbcr_encoding,
            range,
        }
    }

    // Predefined color spaces from libcamera
    pub const RAW: Self = Self::new(Primaries::Raw, TransferFunction::Linear, YcbcrEncoding::None, Range::Full);
    pub const SRGB: Self = Self::new(Primaries::Rec709, TransferFunction::Srgb, YcbcrEncoding::Rec601, Range::Full);
    pub const SMPTE170M: Self =
        Self::new(Primaries::Smpte170m, TransferFunction::Rec709, YcbcrEncoding::Rec601, Range::Limited);
    pub const REC709: Self = Self::new(Primaries::Rec709, TransferFunction::Rec709, YcbcrEncoding::Rec709, Range::Limited);
    pub const REC2020: Self =
        Self::new(Primaries::Rec2020, TransferFunction::Rec709, YcbcrEncoding::Rec2020, Range::Limited);
}

impl From<ColorSpace> for libcamera_color_space_t {
    fn from(cs: ColorSpace) -> Self {
        unsafe {
            libcamera_color_space_make(
                match cs.primaries {
                    Primaries::Raw => libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_RAW,
                    Primaries::Smpte170m => libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_SMPTE170M,
                    Primaries::Rec709 => libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_REC709,
                    Primaries::Rec2020 => libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_REC2020,
                },
                match cs.transfer_function {
                    TransferFunction::Linear => {
                        libcamera_color_space_transfer_function::LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_LINEAR
                    }
                    TransferFunction::Srgb => {
                        libcamera_color_space_transfer_function::LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_SRGB
                    }
                    TransferFunction::Rec709 => {
                        libcamera_color_space_transfer_function::LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_REC709
                    }
                },
                match cs.ycbcr_encoding {
                    YcbcrEncoding::None => {
                        libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_NONE
                    }
                    YcbcrEncoding::Rec601 => {
                        libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC601
                    }
                    YcbcrEncoding::Rec709 => {
                        libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC709
                    }
                    YcbcrEncoding::Rec2020 => {
                        libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC2020
                    }
                },
                match cs.range {
                    Range::Full => libcamera_color_space_range::LIBCAMERA_COLOR_SPACE_RANGE_FULL,
                    Range::Limited => libcamera_color_space_range::LIBCAMERA_COLOR_SPACE_RANGE_LIMITED,
                },
            )
        }
    }
}

impl From<libcamera_color_space_t> for ColorSpace {
    fn from(cs: libcamera_color_space_t) -> Self {
        let primaries = match cs.primaries {
            libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_RAW => Primaries::Raw,
            libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_SMPTE170M => Primaries::Smpte170m,
            libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_REC709 => Primaries::Rec709,
            libcamera_color_space_primaries::LIBCAMERA_COLOR_SPACE_PRIMARIES_REC2020 => Primaries::Rec2020,
            _ => Primaries::Raw,
        };
        let transfer_function = match cs.transfer_function {
            libcamera_color_space_transfer_function::LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_LINEAR => {
                TransferFunction::Linear
            }
            libcamera_color_space_transfer_function::LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_SRGB => {
                TransferFunction::Srgb
            }
            libcamera_color_space_transfer_function::LIBCAMERA_COLOR_SPACE_TRANSFER_FUNCTION_REC709 => {
                TransferFunction::Rec709
            }
            _ => TransferFunction::Linear,
        };
        let ycbcr_encoding = match cs.ycbcr_encoding {
            libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_NONE => YcbcrEncoding::None,
            libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC601 => YcbcrEncoding::Rec601,
            libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC709 => YcbcrEncoding::Rec709,
            libcamera_color_space_ycbcr_encoding::LIBCAMERA_COLOR_SPACE_YCBCR_ENCODING_REC2020 => {
                YcbcrEncoding::Rec2020
            }
            _ => YcbcrEncoding::None,
        };
        let range = match cs.range {
            libcamera_color_space_range::LIBCAMERA_COLOR_SPACE_RANGE_FULL => Range::Full,
            libcamera_color_space_range::LIBCAMERA_COLOR_SPACE_RANGE_LIMITED => Range::Limited,
            _ => Range::Full,
        };

        ColorSpace {
            primaries,
            transfer_function,
            ycbcr_encoding,
            range,
        }
    }
}

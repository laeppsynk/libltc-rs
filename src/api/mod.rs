pub mod consts;
pub mod decoder;
pub mod encoder;
pub mod frame;

use crate::error;
use crate::raw;
pub use error::{LTCDecoderError, LTCEncoderError};
use raw::ltcsnd_sample_t;

#[derive(Debug)]
pub struct SMPTETimecode {
    inner_unsafe_ptr: *mut raw::SMPTETimecode,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum LTCTVStandard {
    LTCTV_525_60,  // 30fps
    LTCTV_625_50,  // 25fps
    LTCTV_1125_60, // 30fps
    LTCTV_FILM_24, // 24fps
}

type SampleType = ltcsnd_sample_t;

// frame-related functions

impl Default for SMPTETimecode {
    fn default() -> Self {
        SMPTETimecode {
            inner_unsafe_ptr: &mut raw::SMPTETimecode::default(),
        }
    }
}

impl From<LTCTVStandard> for raw::LTC_TV_STANDARD {
    fn from(val: LTCTVStandard) -> Self {
        match val {
            LTCTVStandard::LTCTV_525_60 => raw::LTC_TV_STANDARD_LTC_TV_525_60,
            LTCTVStandard::LTCTV_625_50 => raw::LTC_TV_STANDARD_LTC_TV_625_50,
            LTCTVStandard::LTCTV_1125_60 => raw::LTC_TV_STANDARD_LTC_TV_1125_60,
            LTCTVStandard::LTCTV_FILM_24 => raw::LTC_TV_STANDARD_LTC_TV_FILM_24,
        }
    }
}

impl LTCTVStandard {
    // I like the name better than simply `into()`
    // TODO: make this a trait and implement it for everything
    pub(crate) fn to_raw(self) -> raw::LTC_TV_STANDARD {
        self.into()
    }
}

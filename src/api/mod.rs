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

// SAFETY: We are allocating the pointer as a Box so it outlives the function
// Drop is implemented for SMPTETimecode
impl Default for SMPTETimecode {
    fn default() -> Self {
        // Allocate the raw SMPTETimecode on the heap using Box
        let inner = Box::new(raw::SMPTETimecode::default());
        SMPTETimecode {
            inner_unsafe_ptr: Box::into_raw(inner),
        }
    }
}

impl Drop for SMPTETimecode {
    fn drop(&mut self) {
        dbg!("Dropping SMPTETimecode");
        if !self.inner_unsafe_ptr.is_null() {
            // SAFETY: the pointer is not null
            unsafe {
                let _ = Box::from_raw(self.inner_unsafe_ptr);
            }
        }
    }
}
impl SMPTETimecode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timezone: [i8; 6],
        years: u8,
        months: u8,
        days: u8,
        hours: u8,
        minutes: u8,
        seconds: u8,
        frame: u8,
    ) -> Self {
        // Allocate the `raw::SMPTETimecode` on the heap using Box
        let boxed_inner = Box::new(raw::SMPTETimecode {
            timezone,
            years,
            months,
            days,
            hours,
            mins: minutes,
            secs: seconds,
            frame,
        });

        // Convert the Box into a raw pointer, storing it in `inner_unsafe_ptr`
        SMPTETimecode {
            inner_unsafe_ptr: Box::into_raw(boxed_inner),
        }
    }
    pub fn timezone(&self) -> [i8; 6] {
        unsafe { (*self.inner_unsafe_ptr).timezone }
    }
    pub fn years(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).years }
    }
    pub fn months(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).months }
    }
    pub fn days(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).days }
    }
    pub fn hours(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).hours }
    }
    pub fn minutes(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).mins }
    }
    pub fn seconds(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).secs }
    }
    pub fn frame(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).frame }
    }
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
    pub(crate) fn to_raw(self) -> raw::LTC_TV_STANDARD {
        self.into()
    }
}

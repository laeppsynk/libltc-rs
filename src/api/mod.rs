pub mod consts;
pub mod decoder;
pub mod encoder;
pub mod frame;

use std::fmt::Display;

use crate::error;
use crate::error::TimecodeError;
use crate::raw;
pub use error::{LTCDecoderError, LTCEncoderError};

#[derive(Debug)]
pub struct SMPTETimecode {
    inner_unsafe_ptr: *mut raw::SMPTETimecode,
}

#[derive(Debug, Copy, Clone)]
pub enum TimecodeWasWrapped {
    No = 0,
    Yes = 1,
}

impl TryInto<TimecodeWasWrapped> for i32 {
    type Error = TimecodeError;
    fn try_into(self) -> Result<TimecodeWasWrapped, Self::Error> {
        match self {
            0 => Ok(TimecodeWasWrapped::No),
            1 => Ok(TimecodeWasWrapped::Yes),
            _ => Err(TimecodeError::InvalidReturn),
        }
    }
}

impl Display for SMPTETimecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timezone = self.timezone();
        let years = self.years();
        let months = self.months();
        let days = self.days();
        let hours = self.hours();
        let minutes = self.minutes();
        let seconds = self.seconds();
        let frame = self.frame();
        write!(
            f,
            "{timezone} {years:02}:{months:02}:{days:02}:{hours:02}:{minutes:02}:{seconds:02}:{frame:02}"
        )
    }
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
        timezone: Timezone,
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
            timezone: timezone.to_raw(),
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
    pub fn timezone(&self) -> Timezone {
        unsafe { (*self.inner_unsafe_ptr).timezone }.into()
    }
    pub const fn years(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).years }
    }
    pub const fn months(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).months }
    }
    pub const fn days(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).days }
    }
    pub const fn hours(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).hours }
    }
    pub const fn minutes(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).mins }
    }
    pub const fn seconds(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).secs }
    }
    pub const fn frame(&self) -> u8 {
        unsafe { (*self.inner_unsafe_ptr).frame }
    }

    pub const fn to_seconds_total(&self, fps: f32) -> f64 {
        assert!(fps > 0.0, "FPS must be greater than 0");

        (self.years() as f64 * 365.0 * 24.0 * 3600.0)
            + (self.months() as f64 * 30.0 * 24.0 * 3600.0)
            + (self.days() as f64 * 24.0 * 3600.0)
            + (self.hours() as f64 * 3600.0)
            + (self.minutes() as f64 * 60.0)
            + (self.seconds() as f64)
            + (self.frame() as f64 / fps as f64)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Timezone([i8; 6]);

impl Display for Timezone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u8_timezone = self.0.iter().map(|x| *x as u8).collect::<Vec<u8>>();

        let str = std::str::from_utf8(u8_timezone.as_slice()).unwrap();
        write!(f, "{str}")
    }
}

impl Default for Timezone {
    fn default() -> Self {
        let bytes: &[u8; 6] = b"+0000\0";
        bytes.into()
    }
}
impl From<&[i8; 6]> for Timezone {
    fn from(timezone: &[i8; 6]) -> Self {
        Timezone(timezone.to_owned())
    }
}

impl From<&[u8; 6]> for Timezone {
    fn from(timezone: &[u8; 6]) -> Self {
        Timezone(timezone.map(|x| x as i8))
    }
}

impl From<[i8; 6]> for Timezone {
    fn from(timezone: [i8; 6]) -> Self {
        Timezone(timezone)
    }
}

impl From<[u8; 6]> for Timezone {
    fn from(timezone: [u8; 6]) -> Self {
        Timezone(timezone.map(|x| x as i8))
    }
}

impl Timezone {
    pub fn new(timezone: [i8; 6]) -> Self {
        Timezone(timezone)
    }

    pub fn to_raw(&self) -> [i8; 6] {
        self.0
    }
}

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Default)]
pub enum LTCTVStandard {
    #[default]
    LTCTV_525_60 = 0, // 30fps
    LTCTV_625_50,  // 25fps
    LTCTV_1125_60, // 30fps
    LTCTV_FILM_24, // 24fps
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
    pub(crate) fn to_raw(self) -> raw::LTC_TV_STANDARD {
        self.into()
    }
}

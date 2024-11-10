use super::LTCTVStandard;
use super::SMPTETimecode;
use crate::raw;

#[derive(Debug)]
pub struct LTCFrame {
    pub(super) inner_unsafe_ptr: *mut raw::LTCFrame,
}

#[derive(Debug)]
pub struct LTCFrameExt {
    pub(super) inner_unsafe_ptr: *mut raw::LTCFrameExt,
}

impl Default for LTCFrame {
    fn default() -> Self {
        LTCFrame {
            inner_unsafe_ptr: &mut raw::LTCFrame::default(),
        }
    }
}

impl Default for LTCFrameExt {
    fn default() -> Self {
        LTCFrameExt {
            inner_unsafe_ptr: &mut raw::LTCFrameExt::default(),
        }
    }
}

impl LTCFrame {
    pub fn new() -> Self {
        let mut frame = LTCFrame {
            inner_unsafe_ptr: &mut raw::LTCFrame::default(),
        };

        // SAFETY: frame is created above and is not null
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_reset((&mut frame).inner_unsafe_ptr);
        }
        frame
    }

    pub fn to_timecode(&self, flags: i32) -> SMPTETimecode {
        let mut timecode = SMPTETimecode {
            inner_unsafe_ptr: &mut raw::SMPTETimecode::default(),
        };

        // SAFETY: We own timecode. The function is assumed to only read the frame.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_to_time(
                (&mut timecode).inner_unsafe_ptr,
                self.inner_unsafe_ptr,
                flags,
            );
        }

        timecode
    }

    pub fn from_timecode(timecode: &SMPTETimecode, standard: LTCTVStandard, flags: i32) -> Self {
        let mut frame = Self::new();

        // SAFETY: We own frame. The function is assumed to only read the timecode.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_time_to_frame(
                (&mut frame).inner_unsafe_ptr,
                timecode.inner_unsafe_ptr,
                standard.to_raw(),
                flags,
            );
        }

        frame
    }

    pub fn increment(&mut self, fps: i32, standard: LTCTVStandard, flags: i32) -> bool {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_increment(self.inner_unsafe_ptr, fps, standard.to_raw(), flags) != 0
        }
    }

    pub fn decrement(&mut self, fps: i32, standard: LTCTVStandard, flags: i32) -> bool {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_decrement(self.inner_unsafe_ptr, fps, standard.to_raw(), flags) != 0
        }
    }

    pub fn set_parity(&mut self, standard: LTCTVStandard) {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_set_parity(self.inner_unsafe_ptr, standard.to_raw());
        }
    }

    pub fn parse_bcg_flags(&self, standard: LTCTVStandard) -> i32 {
        // SAFETY: The function is assumed to only read self (the frame)
        unsafe { raw::ltc_frame_parse_bcg_flags(self.inner_unsafe_ptr, standard.to_raw()) }
    }

    pub fn get_user_bits(&self) -> u32 {
        // SAFETY: The function is assumed to only read self (the frame)
        unsafe { raw::ltc_frame_get_user_bits(self.inner_unsafe_ptr) as u32 }
    }
}

pub fn calc_frame_alignment(samples_per_frame: f64, standard: LTCTVStandard) -> i64 {
    // SAFETY: The function is assumed to be pure
    unsafe { raw::ltc_frame_alignment(samples_per_frame, standard.to_raw()) }
}

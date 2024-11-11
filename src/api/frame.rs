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

// SAFETY: We are allocating the pointer as a Box so it outlives the function
// Drop is implemented for LTCFrame
impl Default for LTCFrame {
    fn default() -> Self {
        let inner = Box::new(raw::LTCFrame::default());
        LTCFrame {
            inner_unsafe_ptr: Box::into_raw(inner),
        }
    }
}

impl Drop for LTCFrame {
    fn drop(&mut self) {
        dbg!("Dropping LTCFrame");
        if !self.inner_unsafe_ptr.is_null() {
            // SAFETY: the pointer is assumed to not be null
            unsafe {
                let _ = Box::from_raw(self.inner_unsafe_ptr);
            }
        }
    }
}

// SAFETY: We are allocating the pointer as a Box so it outlives the function
// Drop is implemented for LTCFrameExt
impl Default for LTCFrameExt {
    fn default() -> Self {
        let inner = Box::new(raw::LTCFrameExt::default());
        LTCFrameExt {
            inner_unsafe_ptr: Box::into_raw(inner),
        }
    }
}

impl Drop for LTCFrameExt {
    fn drop(&mut self) {
        dbg!("Dropping LTCFrameExt");
        if !self.inner_unsafe_ptr.is_null() {
            // SAFETY: the pointer is assumed to not be null
            unsafe {
                let _ = Box::from_raw(self.inner_unsafe_ptr);
            }
        }
    }
}

impl LTCFrame {
    pub fn new() -> Self {
        // SAFETY: The pointer will outlive the function because it is allocated in a Box
        let inner = Box::new(raw::LTCFrame::default());
        let mut frame = LTCFrame {
            inner_unsafe_ptr: Box::into_raw(inner),
        };

        // SAFETY: frame is created above and is not null
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_reset((&mut frame).inner_unsafe_ptr);
        }
        frame
    }

    pub fn to_timecode(&self, flags: crate::consts::LtcBgFlags) -> SMPTETimecode {
        let mut timecode = SMPTETimecode::default();

        // SAFETY: We own timecode. The function is assumed to only read the frame.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_to_time(
                (&mut timecode).inner_unsafe_ptr,
                self.inner_unsafe_ptr,
                flags as i32,
            );
        }

        timecode
    }

    pub fn from_timecode(
        timecode: &SMPTETimecode,
        standard: LTCTVStandard,
        flags: crate::consts::LtcBgFlags,
    ) -> Self {
        let mut frame = Self::new();

        // SAFETY: We own frame. The function is assumed to only read the timecode.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_time_to_frame(
                (&mut frame).inner_unsafe_ptr,
                timecode.inner_unsafe_ptr,
                standard.to_raw(),
                flags as i32,
            );
        }

        frame
    }

    pub fn increment(
        &mut self,
        fps: i32,
        standard: LTCTVStandard,
        flags: crate::consts::LtcBgFlags,
    ) -> bool {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_increment(self.inner_unsafe_ptr, fps, standard.to_raw(), flags as i32)
                != 0
        }
    }

    pub fn decrement(
        &mut self,
        fps: i32,
        standard: LTCTVStandard,
        flags: crate::consts::LtcBgFlags,
    ) -> bool {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_decrement(self.inner_unsafe_ptr, fps, standard.to_raw(), flags as i32)
                != 0
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

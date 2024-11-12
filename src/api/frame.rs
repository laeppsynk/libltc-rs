use std::ops::Deref;

use super::LTCTVStandard;
use super::SMPTETimecode;
use crate::consts;
use crate::consts::LtcBgFlags;
use crate::consts::SampleType;
use crate::error::TimecodeError;
use crate::raw;
use crate::TimecodeWasWrapped;

#[derive(Debug, Clone, Copy)]
pub struct LTCFrame {
    pub(super) inner_raw: raw::LTCFrame,
}

#[derive(Debug)]
pub struct LTCFrameRef<'a> {
    pub(super) raw_ref: &'a raw::LTCFrame,
}

impl<'a> LTCFrameRef<'a> {
    pub fn dfbit(&self) -> bool {
        self.raw_ref.dfbit() != 0
    }

    pub fn to_owned(&self) -> LTCFrame {
        LTCFrame {
            inner_raw: *self.raw_ref,
        }
    }
}
impl Deref for LTCFrameRef<'_> {
    type Target = raw::LTCFrame;
    fn deref(&self) -> &Self::Target {
        self.raw_ref
    }
}

#[derive(Debug)]
pub struct LTCFrameExt<'a> {
    pub(super) inner_unsafe_ptr: *mut raw::LTCFrameExt,
    phantom: std::marker::PhantomData<&'a raw::LTCFrameExt>,
}

impl<'a> LTCFrameExt<'a> {
    // FIX: this does not work. We are creating a copy of the outer LTCFrame, but not the inner
    // pointer. This is causing a double free error.
    // Either create a copy or add methods directly to LTCFrameExt
    fn raw_ltc_ref(&'a self) -> &'a raw::LTCFrame {
        &unsafe { &*self.inner_unsafe_ptr }.ltc
    }

    pub fn ltc_ref(&'a self) -> LTCFrameRef {
        let r = self.raw_ltc_ref();
        LTCFrameRef { raw_ref: r }
    }

    pub fn off_start(&self) -> i64 {
        unsafe { *self.inner_unsafe_ptr }.off_start
    }
    pub fn set_off_start(&self, off_start: i64) {
        unsafe {
            (*self.inner_unsafe_ptr).off_start = off_start;
        }
    }
    pub fn off_end(&self) -> i64 {
        unsafe { *self.inner_unsafe_ptr }.off_end
    }
    pub fn set_off_end(&self, off_end: i64) {
        unsafe {
            (*self.inner_unsafe_ptr).off_end = off_end;
        }
    }
    pub fn reverse(&self) -> bool {
        unsafe { *self.inner_unsafe_ptr }.reverse != 0
    }
    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            (*self.inner_unsafe_ptr).reverse = if reverse { 1 } else { 0 };
        }
    }
    pub fn biphase_tics(&self) -> [f32; 80usize] {
        unsafe { *self.inner_unsafe_ptr }.biphase_tics
    }
    pub fn set_biphase_tics(&self, biphase_tics: [f32; 80usize]) {
        unsafe {
            (*self.inner_unsafe_ptr).biphase_tics = biphase_tics;
        }
    }
    pub fn sample_min(&self) -> SampleType {
        unsafe { *self.inner_unsafe_ptr }.sample_min
    }
    pub fn set_sample_min(&self, sample_min: SampleType) {
        unsafe {
            (*self.inner_unsafe_ptr).sample_min = sample_min;
        }
    }
    pub fn sample_max(&self) -> SampleType {
        unsafe { *self.inner_unsafe_ptr }.sample_max
    }
    pub fn set_sample_max(&self, sample_max: SampleType) {
        unsafe {
            (*self.inner_unsafe_ptr).sample_max = sample_max;
        }
    }
    pub fn volume(&self) -> f64 {
        unsafe { *self.inner_unsafe_ptr }.volume
    }
    pub fn set_volume(&self, volume: f64) {
        unsafe {
            (*self.inner_unsafe_ptr).volume = volume;
        }
    }
}

// SAFETY: We are allocating the pointer as a Box so it outlives the function
// Drop is implemented for LTCFrame
impl Default for LTCFrame {
    fn default() -> Self {
        LTCFrame {
            inner_raw: raw::LTCFrame::default(),
        }
    }
}

// SAFETY: We are allocating the pointer as a Box so it outlives the function
// Drop is implemented for LTCFrameExt
impl<'a> Default for LTCFrameExt<'a> {
    fn default() -> Self {
        let inner = Box::new(raw::LTCFrameExt::default());
        LTCFrameExt {
            inner_unsafe_ptr: Box::into_raw(inner),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a> Drop for LTCFrameExt<'a> {
    fn drop(&mut self) {
        dbg!("Dropping LTCFrameExt");
        if !self.inner_unsafe_ptr.is_null() {
            // SAFETY: the pointer is assumed to not be null
            unsafe {
                let _ = Box::from_raw(self.inner_unsafe_ptr);
            }
            self.inner_unsafe_ptr = std::ptr::null_mut();
        }
    }
}

impl LTCFrame {
    pub fn new() -> Self {
        // SAFETY: The pointer will outlive the function because it is allocated in a Box
        let mut frame = LTCFrame {
            inner_raw: raw::LTCFrame::default(),
        };

        // SAFETY: frame is created above and is not null
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_reset(&mut (&mut frame).inner_raw);
        }
        frame
    }

    pub fn to_timecode(&self, flags: consts::LtcBgFlags) -> SMPTETimecode {
        let mut timecode = SMPTETimecode::default();
        let ptr = &self.inner_raw as *const raw::LTCFrame;

        // SAFETY: We own timecode. The function is assumed to only read the frame.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_to_time(
                (&mut timecode).inner_unsafe_ptr,
                ptr as *mut raw::LTCFrame,
                flags.into(),
            );
        }

        timecode
    }

    pub fn from_timecode(
        timecode: &SMPTETimecode,
        standard: LTCTVStandard,
        flags: consts::LtcBgFlags,
    ) -> Self {
        let mut frame = Self::new();

        // SAFETY: We own frame. The function is assumed to only read the timecode.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_time_to_frame(
                &mut frame.inner_raw,
                timecode.inner_unsafe_ptr,
                standard.to_raw(),
                flags.into(),
            );
        }

        frame
    }

    pub fn from_timecode_inplace(
        &mut self,
        timecode: &SMPTETimecode,
        standard: LTCTVStandard,
        flags: consts::LtcBgFlags,
    ) {
        // SAFETY: We own frame. The function is assumed to only read the timecode.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_time_to_frame(
                &mut self.inner_raw,
                timecode.inner_unsafe_ptr,
                standard.to_raw(),
                flags.into(),
            );
        }
    }

    pub fn increment(
        &mut self,
        fps: i32,
        standard: LTCTVStandard,
        flags: LtcBgFlags,
    ) -> Result<TimecodeWasWrapped, TimecodeError> {
        // SAFETY: We own self
        let timecode_was_wrapped = unsafe {
            raw::ltc_frame_increment(&mut self.inner_raw, fps, standard.to_raw(), flags.into())
        };
        match timecode_was_wrapped {
            0 => Ok(TimecodeWasWrapped::No),
            1 => Ok(TimecodeWasWrapped::Yes),
            _ => Err(TimecodeError::InvalidReturn),
        }
    }

    pub fn decrement(
        &mut self,
        fps: i32,
        standard: LTCTVStandard,
        flags: LtcBgFlags,
    ) -> Result<TimecodeWasWrapped, TimecodeError> {
        // SAFETY: We own self
        let timecode_was_wrapped = unsafe {
            raw::ltc_frame_decrement(&mut self.inner_raw, fps, standard.to_raw(), flags.into())
        };
        match timecode_was_wrapped {
            0 => Ok(TimecodeWasWrapped::No),
            1 => Ok(TimecodeWasWrapped::Yes),
            _ => Err(TimecodeError::InvalidReturn),
        }
    }

    pub fn set_parity(&mut self, standard: LTCTVStandard) {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_set_parity(&mut self.inner_raw, standard.to_raw());
        }
    }

    pub fn parse_bcg_flags(&self, standard: LTCTVStandard) -> LtcBgFlags {
        // SAFETY: The function is assumed to only read self (the frame) so it is safe to pass a
        // mut pointer to satisfy the C function signature
        let ptr = &self.inner_raw as *const raw::LTCFrame;
        unsafe { raw::ltc_frame_parse_bcg_flags(ptr as *mut raw::LTCFrame, standard.to_raw()) }
            .into()
    }

    pub fn get_user_bits(&self) -> u32 {
        // SAFETY: The function is assumed to only read self (the frame) so it is safe to pass a
        // mut pointer to satisfy the C function signature
        let ptr = &self.inner_raw as *const raw::LTCFrame;
        unsafe { raw::ltc_frame_get_user_bits(ptr as *mut raw::LTCFrame) as u32 }
    }
}

pub fn calc_frame_alignment(samples_per_frame: f64, standard: LTCTVStandard) -> i64 {
    // SAFETY: The function is assumed to be pure
    unsafe { raw::ltc_frame_alignment(samples_per_frame, standard.to_raw()) }
}

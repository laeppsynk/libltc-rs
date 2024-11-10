// lib.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod error;
mod raw;

use std::slice;

pub use error::{LTCDecoderError, LTCEncoderError};
use raw::ltcsnd_sample_t;

#[derive(Debug)]
pub struct SMPTETimecode {
    inner_unsafe: *mut raw::SMPTETimecode,
}

#[derive(Debug)]
pub struct LTCFrame {
    inner_unsafe: *mut raw::LTCFrame,
}

#[derive(Debug)]
pub struct LTCFrameExt {
    inner_unsafe: *mut raw::LTCFrameExt,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum LTCTVStandard {
    LTCTV_525_60,  // 30fps
    LTCTV_625_50,  // 25fps
    LTCTV_1125_60, // 30fps
    LTCTV_FILM_24, // 24fps
}

#[derive(Debug)]
pub struct LTCEncoder {
    inner_unsafe: *mut raw::LTCEncoder,
}

#[derive(Debug)]
pub struct LTCDecoder {
    inner_unsafe: *mut raw::LTCDecoder,
}

type SampleType = ltcsnd_sample_t;

// frame-related functions
impl LTCFrame {
    pub fn new() -> Self {
        let mut frame = LTCFrame {
            inner_unsafe: &mut raw::LTCFrame::default(),
        };

        // SAFETY: frame is created above and is not null
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_reset((&mut frame).inner_unsafe);
        }
        frame
    }

    pub fn to_timecode(&self, flags: i32) -> SMPTETimecode {
        let mut timecode = SMPTETimecode {
            inner_unsafe: &mut raw::SMPTETimecode::default(),
        };

        // SAFETY: We own timecode. The function is assumed to only read the frame.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_frame_to_time((&mut timecode).inner_unsafe, self.inner_unsafe, flags);
        }

        timecode
    }

    pub fn from_timecode(timecode: &SMPTETimecode, standard: LTCTVStandard, flags: i32) -> Self {
        let mut frame = Self::new();

        // SAFETY: We own frame. The function is assumed to only read the timecode.
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_time_to_frame(
                (&mut frame).inner_unsafe,
                timecode.inner_unsafe,
                standard.to_raw(),
                flags,
            );
        }

        frame
    }

    pub fn increment(&mut self, fps: i32, standard: LTCTVStandard, flags: i32) -> bool {
        // SAFETY: We own self
        unsafe { raw::ltc_frame_increment(self.inner_unsafe, fps, standard.to_raw(), flags) != 0 }
    }

    pub fn decrement(&mut self, fps: i32, standard: LTCTVStandard, flags: i32) -> bool {
        // SAFETY: We own self
        unsafe { raw::ltc_frame_decrement(self.inner_unsafe, fps, standard.to_raw(), flags) != 0 }
    }

    pub fn set_parity(&mut self, standard: LTCTVStandard) {
        // SAFETY: We own self
        unsafe {
            raw::ltc_frame_set_parity(self.inner_unsafe, standard.to_raw());
        }
    }

    pub fn parse_bcg_flags(&self, standard: LTCTVStandard) -> i32 {
        // SAFETY: The function is assumed to only read self (the frame)
        unsafe { raw::ltc_frame_parse_bcg_flags(self.inner_unsafe, standard.to_raw()) }
    }

    pub fn get_user_bits(&self) -> u32 {
        // SAFETY: The function is assumed to only read self (the frame)
        unsafe { raw::ltc_frame_get_user_bits(self.inner_unsafe) as u32 }
    }
}

pub fn calc_frame_alignment(samples_per_frame: f64, standard: LTCTVStandard) -> i64 {
    // SAFETY: The function is assumed to be pure
    unsafe { raw::ltc_frame_alignment(samples_per_frame, standard.to_raw()) }
}

impl Default for LTCFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl LTCDecoder {
    pub fn try_new(apv: i32, queue_size: i32) -> Result<Self, LTCDecoderError> {
        // Safety: the C function does not modify memory, it only allocates memory. Drop is
        // implemented for LTCDecoder
        let decoder = unsafe { raw::ltc_decoder_create(apv, queue_size) };
        if decoder.is_null() {
            Err(LTCDecoderError::CreateError)
        } else {
            Ok(LTCDecoder {
                inner_unsafe: decoder,
            })
        }
    }

    pub fn write(&mut self, buf: &[SampleType], posinfo: i64) {
        // SAFETY: We own self. buf is only read.
        unsafe {
            // SAFETY: we can cast *const SampleType as *mut SampleType to accomodate for the C function
            // signature, we assume that ltc_decoder_write is will only read from it
            let mut_ptr_buf = buf.as_ptr() as *mut SampleType;
            raw::ltc_decoder_write(
                self.inner_unsafe,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn write_double(&mut self, buf: &[f64], posinfo: i64) {
        // SAFETY: We own self. buf is only read.
        unsafe {
            // SAFETY: we can cast *const SampleType as *mut SampleType to accomodate for the C function
            // signature, we assume that ltc_decoder_write is will only read from it
            let mut_ptr_buf = buf.as_ptr() as *mut f64;
            raw::ltc_decoder_write_double(
                self.inner_unsafe,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn write_float(&mut self, buf: &[f32], posinfo: i64) {
        // SAFETY: We own self. buf is only read.
        unsafe {
            // SAFETY: we can cast *const SampleType as *mut SampleType to accomodate for the C function
            // signature, we assume that ltc_decoder_write is will only read from it
            let mut_ptr_buf = buf.as_ptr() as *mut f32;
            raw::ltc_decoder_write_float(
                self.inner_unsafe,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn write_s16(&mut self, buf: &[i16], posinfo: i64) {
        // SAFETY: We own self. buf is only read.
        unsafe {
            // SAFETY: we can cast *const SampleType as *mut SampleType to accomodate for the C function
            // signature, we assume that ltc_decoder_write is will only read from it
            let mut_ptr_buf = buf.as_ptr() as *mut i16;
            raw::ltc_decoder_write_s16(
                self.inner_unsafe,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn write_u16(&mut self, buf: &[u16], posinfo: i64) {
        // SAFETY: We own self. buf is only read.
        unsafe {
            // SAFETY: we can cast *const SampleType as *mut SampleType to accomodate for the C function
            // signature, we assume that ltc_decoder_write is will only read from it
            let mut_ptr_buf = buf.as_ptr() as *mut u16;
            raw::ltc_decoder_write_u16(
                self.inner_unsafe,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn read(&self) -> Option<LTCFrameExt> {
        let mut frame = LTCFrameExt {
            inner_unsafe: &mut raw::LTCFrameExt::default(),
        };

        // SAFETY: We own frame. The function is assumed to only read from self and write to frame
        let result = unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_decoder_read(self.inner_unsafe, (&mut frame).inner_unsafe)
        };
        if result == 0 {
            None
        } else {
            Some(frame)
        }
    }

    pub fn queue_flush(&mut self) {
        // SAFETY: We own self
        unsafe {
            raw::ltc_decoder_queue_flush(self.inner_unsafe);
        }
    }

    pub fn queue_length(&self) -> i32 {
        // SAFETY: The function is assumed to only read self
        unsafe { raw::ltc_decoder_queue_length(self.inner_unsafe) }
    }
}

impl Default for SMPTETimecode {
    fn default() -> Self {
        SMPTETimecode {
            inner_unsafe: &mut raw::SMPTETimecode::default(),
        }
    }
}

impl LTCEncoder {
    pub fn try_new(
        sample_rate: f64,
        fps: f64,
        standard: LTCTVStandard,
        flags: i32,
    ) -> Result<Self, LTCEncoderError> {
        let raw_standard = standard.to_raw();
        // Safety: the C function does not modify memory, it only allocates memory. Drop is implemented for LTCEncoder
        let encoder = unsafe { raw::ltc_encoder_create(sample_rate, fps, raw_standard, flags) };
        if encoder.is_null() {
            Err(LTCEncoderError::CreateError)
        } else {
            Ok(LTCEncoder {
                inner_unsafe: encoder,
            })
        }
    }

    pub fn set_timecode(&mut self, timecode: &SMPTETimecode) {
        // Safety: We own self, the function is assumed to only read the timecode and write to self
        unsafe {
            raw::ltc_encoder_set_timecode(self.inner_unsafe, timecode.inner_unsafe);
        }
    }

    pub fn get_timecode(&self) -> SMPTETimecode {
        let mut timecode = SMPTETimecode::default();
        // We own timecode, the function is assumed to only read from self and write to timecode
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_encoder_get_timecode(self.inner_unsafe, (&mut timecode).inner_unsafe);
        }
        timecode
    }

    pub fn set_user_bits(&mut self, data: u32) {
        // SAFETY: We own self
        unsafe {
            raw::ltc_encoder_set_user_bits(self.inner_unsafe, data as libc::c_ulong);
        }
    }

    pub fn inc_timecode(&mut self) -> bool {
        // SAFETY: We own self
        unsafe { raw::ltc_encoder_inc_timecode(self.inner_unsafe) != 0 }
    }

    pub fn dec_timecode(&mut self) -> bool {
        // SAFETY: We own self
        unsafe { raw::ltc_encoder_dec_timecode(self.inner_unsafe) != 0 }
    }

    pub fn set_frame(&mut self, frame: &LTCFrame) {
        // SAFETY: We own self, the function is assumed to only read the frame and write to self
        unsafe { raw::ltc_encoder_set_frame(self.inner_unsafe, frame.inner_unsafe) }
    }

    pub fn get_frame(&self) -> LTCFrame {
        let mut frame = LTCFrame::new();
        // SAFETY: We own frame. The function is assumed to only read from self and write to frame
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_encoder_get_frame(self.inner_unsafe, (&mut frame).inner_unsafe);
        }
        frame
    }

    pub fn copy_buffer_inplace(&self, buf: &mut [SampleType]) -> i32 {
        unsafe { raw::ltc_encoder_copy_buffer(self.inner_unsafe, buf.as_mut_ptr()) }
    }

    pub fn copy_buffer(&self) -> (Vec<u8>, i32) {
        let mut buf = vec![0; self.get_buffersize()];
        let size = unsafe { raw::ltc_encoder_copy_buffer(self.inner_unsafe, buf.as_mut_ptr()) };
        (buf, size)
    }

    // TODO: Possible leak? does ptr ever get deallocated - maybe when the encoder is deallocated?
    pub fn get_buf_ref(&self, flush: bool) -> (&[SampleType], usize) {
        let mut size: i32 = 0;
        // SAFETY: We own self, we own size.
        let ptr = unsafe {
            raw::ltc_encoder_get_bufptr(
                self.inner_unsafe,
                &mut size as *mut _,
                if flush { 1 } else { 0 },
            )
        };
        // SAFETY: We own ptr and size
        let mut_reference = unsafe { slice::from_raw_parts_mut(ptr, size as usize) };
        (mut_reference, size as usize)
    }

    // TODO: Possible leak? does ptr ever get deallocated - maybe when the encoder is deallocated?
    pub fn get_buf_ref_mut(&self, flush: bool) -> (&mut [SampleType], usize) {
        let mut size: i32 = 0;
        // SAFETY: We own self, we own size.
        // This function allocates a buffer pointed at by ptr
        let ptr = unsafe {
            raw::ltc_encoder_get_bufptr(
                self.inner_unsafe,
                &mut size as *mut i32,
                if flush { 1 } else { 0 },
            )
        };
        // SAFETY: We own ptr and size, size is safe to cast to usize
        let mut_reference = unsafe { slice::from_raw_parts_mut(ptr, size as usize) };
        (mut_reference, size as usize)
    }

    pub fn buffer_flush(&mut self) {
        unsafe {
            raw::ltc_encoder_buffer_flush(self.inner_unsafe);
        }
    }

    pub fn get_buffersize(&self) -> usize {
        // SAFETY: The function is assumed to only read self
        unsafe { raw::ltc_encoder_get_buffersize(self.inner_unsafe) }
    }

    pub fn reinit(
        &mut self,
        sample_rate: f64,
        fps: f64,
        standard: LTCTVStandard,
        flags: i32,
    ) -> Result<(), LTCEncoderError> {
        let result = unsafe {
            raw::ltc_encoder_reinit(
                self.inner_unsafe,
                sample_rate,
                fps,
                standard.to_raw(),
                flags,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::ReinitError)
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            raw::ltc_encoder_reset(self.inner_unsafe);
        }
    }

    pub fn set_bufsize(&mut self, sample_rate: f64, fps: f64) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_set_bufsize(self.inner_unsafe, sample_rate, fps) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::BufferSizeError)
        }
    }

    pub fn get_volume(&self) -> f64 {
        unsafe { raw::ltc_encoder_get_volume(self.inner_unsafe) }
    }

    pub fn set_volume(&mut self, dbfs: f64) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_set_volume(self.inner_unsafe, dbfs) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::VolumeError)
        }
    }

    pub fn get_filter(&self) -> f64 {
        unsafe { raw::ltc_encoder_get_filter(self.inner_unsafe) }
    }

    pub fn set_filter(&mut self, rise_time: f64) {
        unsafe {
            raw::ltc_encoder_set_filter(self.inner_unsafe, rise_time);
        }
    }

    pub fn encode_byte(&mut self, byte: i32, speed: f64) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_encode_byte(self.inner_unsafe, byte, speed) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::EncodeError)
        }
    }

    pub fn end_encode(&mut self) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_end_encode(self.inner_unsafe) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::EncodeError)
        }
    }

    pub fn encode_frame(&mut self) {
        unsafe {
            raw::ltc_encoder_encode_frame(self.inner_unsafe);
        }
    }

    pub fn encode_reversed_frame(&mut self) {
        unsafe {
            raw::ltc_encoder_encode_reversed_frame(self.inner_unsafe);
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

impl Drop for LTCEncoder {
    fn drop(&mut self) {
        unsafe {
            raw::ltc_encoder_free(self.inner_unsafe);
        }
    }
}

impl Drop for LTCDecoder {
    fn drop(&mut self) {
        unsafe {
            raw::ltc_decoder_free(self.inner_unsafe);
        }
    }
}
// TODO: impl  Frame
// TODO: impl Timecode methdods for accessing fields and creating

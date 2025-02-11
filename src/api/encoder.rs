use super::consts::LtcBgFlags;
use super::frame::LTCFrame;
use super::LTCTVStandard;
use super::SMPTETimecode;
use crate::api::consts::SampleType;
use crate::api::TimecodeWasWrapped;
use crate::error::LTCEncoderError;
use crate::error::TimecodeError;
use crate::raw;
use core::slice;

#[derive(Debug)]
pub struct LTCEncoder {
    inner_unsafe_ptr: *mut raw::LTCEncoder,
}

unsafe impl Send for LTCEncoder {}

impl Drop for LTCEncoder {
    fn drop(&mut self) {
        unsafe {
            raw::ltc_encoder_free(self.inner_unsafe_ptr);
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LTCEncoderConfig {
    pub sample_rate: f64,
    pub fps: f64,
    pub standard: LTCTVStandard,
    pub flags: LtcBgFlags,
}

impl Default for LTCEncoderConfig {
    fn default() -> Self {
        LTCEncoderConfig {
            sample_rate: 48_000.0,
            fps: 25.0,
            standard: LTCTVStandard::LTCTV_625_50,
            flags: LtcBgFlags::default(),
        }
    }
}

impl<'a> LTCEncoder {
    pub fn try_new(config: &LTCEncoderConfig) -> Result<Self, LTCEncoderError> {
        // Safety: the C function does not modify memory, it only allocates memory. Drop is implemented for LTCEncoder
        let encoder = unsafe {
            raw::ltc_encoder_create(
                config.sample_rate,
                config.fps,
                config.standard.to_raw(),
                config.flags.into(),
            )
        };
        if encoder.is_null() {
            Err(LTCEncoderError::CreateError)
        } else {
            Ok(LTCEncoder {
                inner_unsafe_ptr: encoder,
            })
        }
    }

    // TODO: this might be incorrect
    pub fn set_timecode(&mut self, timecode: &SMPTETimecode) {
        // Safety: We own self, the function is assumed to only read the timecode and write to self
        unsafe {
            raw::ltc_encoder_set_timecode(self.inner_unsafe_ptr, timecode.inner_unsafe_ptr);
        }
    }

    pub fn get_timecode(&self) -> SMPTETimecode {
        let mut timecode = SMPTETimecode::default();
        // We own timecode, the function is assumed to only read from self and write to timecode
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_encoder_get_timecode(self.inner_unsafe_ptr, (&mut timecode).inner_unsafe_ptr);
        }
        timecode
    }

    pub fn get_timecode_inplace(&self, timecode: &mut SMPTETimecode) {
        // We own timecode, the function is assumed to only read from self and write to timecode
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_encoder_get_timecode(self.inner_unsafe_ptr, timecode.inner_unsafe_ptr);
        }
    }

    pub fn set_user_bits(&mut self, data: u32) {
        // SAFETY: We own self
        unsafe {
            raw::ltc_encoder_set_user_bits(self.inner_unsafe_ptr, data as libc::c_ulong);
        }
    }

    pub fn inc_timecode(&mut self) -> Result<TimecodeWasWrapped, LTCEncoderError> {
        // SAFETY: We own self
        unsafe { raw::ltc_encoder_inc_timecode(self.inner_unsafe_ptr) }
            .try_into()
            .map_err(|e: TimecodeError| e.into())
    }

    pub fn dec_timecode(&mut self) -> Result<TimecodeWasWrapped, LTCEncoderError> {
        // SAFETY: We own self
        unsafe { raw::ltc_encoder_dec_timecode(self.inner_unsafe_ptr) }
            .try_into()
            .map_err(|e: TimecodeError| e.into())
    }

    pub fn set_frame(&mut self, frame: &LTCFrame) {
        let mut inner_raw = frame.inner_raw;
        // SAFETY: We own self, the function is assumed to only read the frame and write to self
        unsafe { raw::ltc_encoder_set_frame(self.inner_unsafe_ptr, &mut inner_raw) }
    }

    pub fn get_frame(&self) -> LTCFrame {
        let mut frame = LTCFrame::new();
        // SAFETY: We own frame. The function is assumed to only read from self and write to frame
        unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_encoder_get_frame(self.inner_unsafe_ptr, &mut frame.inner_raw);
        }
        frame
    }

    pub fn copy_buffer_inplace(&self, buf: &mut [SampleType]) -> i32 {
        unsafe { raw::ltc_encoder_copy_buffer(self.inner_unsafe_ptr, buf.as_mut_ptr()) }
    }

    pub fn copy_buffer(&self) -> (Vec<u8>, usize) {
        let mut buf = vec![0; self.get_buffersize()];
        let size = unsafe { raw::ltc_encoder_copy_buffer(self.inner_unsafe_ptr, buf.as_mut_ptr()) };
        (buf, size as usize)
    }

    // TODO: Possible leak? does ptr ever get deallocated - maybe when the encoder is deallocated?
    pub fn get_buf_ref(&'a self, flush: bool) -> (&'a [SampleType], usize) {
        // SAFETY: The buffer (pointed at by ptr) outlives the function as it has the same
        // lifetime as self
        let mut ptr = std::ptr::null_mut();
        // SAFETY: Self is assumed to only be read - for the buffersize
        let size = unsafe {
            raw::ltc_encoder_get_bufferptr(
                self.inner_unsafe_ptr,
                &mut ptr,
                if flush { 1 } else { 0 },
            )
        };

        (
            unsafe { slice::from_raw_parts(ptr, size as usize) },
            size as usize,
        )
    }

    // TODO: Possible leak? does ptr ever get deallocated - maybe when the encoder is deallocated?
    pub fn get_buf_ref_mut(&'a mut self, flush: bool) -> (&'a mut [SampleType], usize) {
        // SAFETY: The buffer (pointed at by ptr) outlives the function as it has the same
        // lifetime as self
        let mut ptr = std::ptr::null_mut();
        // SAFETY: Self is assumed to only be read - for the buffersize
        let size = unsafe {
            raw::ltc_encoder_get_bufferptr(
                self.inner_unsafe_ptr,
                &mut ptr,
                if flush { 1 } else { 0 },
            )
        };
        (
            unsafe { slice::from_raw_parts_mut(ptr, size as usize) },
            size as usize,
        )
    }

    pub fn buffer_flush(&mut self) {
        unsafe {
            raw::ltc_encoder_buffer_flush(self.inner_unsafe_ptr);
        }
    }

    pub fn get_buffersize(&self) -> usize {
        // SAFETY: The function is assumed to only read self
        unsafe { raw::ltc_encoder_get_buffersize(self.inner_unsafe_ptr) }
    }

    pub fn reinit(
        &mut self,
        sample_rate: f64,
        fps: f64,
        standard: LTCTVStandard,
        flags: LtcBgFlags,
    ) -> Result<(), LTCEncoderError> {
        let result = unsafe {
            raw::ltc_encoder_reinit(
                self.inner_unsafe_ptr,
                sample_rate,
                fps,
                standard.to_raw(),
                flags.into(),
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
            raw::ltc_encoder_reset(self.inner_unsafe_ptr);
        }
    }

    pub fn set_buffersize(&mut self, sample_rate: f64, fps: f64) -> Result<(), LTCEncoderError> {
        let result =
            unsafe { raw::ltc_encoder_set_buffersize(self.inner_unsafe_ptr, sample_rate, fps) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::BufferSizeError)
        }
    }

    pub fn get_volume(&self) -> f64 {
        unsafe { raw::ltc_encoder_get_volume(self.inner_unsafe_ptr) }
    }

    pub fn set_volume(&mut self, dbfs: f64) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_set_volume(self.inner_unsafe_ptr, dbfs) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::VolumeError)
        }
    }

    pub fn get_filter(&self) -> f64 {
        unsafe { raw::ltc_encoder_get_filter(self.inner_unsafe_ptr) }
    }

    pub fn set_filter(&mut self, rise_time: f64) {
        unsafe {
            raw::ltc_encoder_set_filter(self.inner_unsafe_ptr, rise_time);
        }
    }

    pub fn encode_byte(&mut self, byte: i32, speed: f64) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_encode_byte(self.inner_unsafe_ptr, byte, speed) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::EncodeError)
        }
    }

    pub fn end_encode(&mut self) -> Result<(), LTCEncoderError> {
        let result = unsafe { raw::ltc_encoder_end_encode(self.inner_unsafe_ptr) };
        if result == 0 {
            Ok(())
        } else {
            Err(LTCEncoderError::EncodeError)
        }
    }

    pub fn encode_frame(&mut self) {
        unsafe {
            raw::ltc_encoder_encode_frame(self.inner_unsafe_ptr);
        }
    }

    pub fn encode_reversed_frame(&mut self) {
        unsafe {
            raw::ltc_encoder_encode_reversed_frame(self.inner_unsafe_ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::consts::LtcBgFlags;

    use super::*;
    #[test]
    fn test_encoder_volume() {
        let encoder_config = LTCEncoderConfig {
            sample_rate: 48_000.0,
            fps: 25.0,
            standard: LTCTVStandard::LTCTV_625_50,
            flags: LtcBgFlags::default(),
        };
        let mut encoder = LTCEncoder::try_new(&encoder_config).unwrap();
        assert!(encoder.set_volume(-18.0).is_ok());

        // We need to account for floating point rounding errors
        const FLOAT_ERROR: f64 = 0.001;
        assert!(
            encoder.get_volume() < -18.0 + FLOAT_ERROR
                || encoder.get_volume() > -18.0 - FLOAT_ERROR
        );
        assert!(encoder.set_volume(1.0).is_err());
    }

    #[test]
    fn test_encoder_reinit() {
        let encoder_config = LTCEncoderConfig {
            sample_rate: 48_000.0,
            fps: 25.0,
            standard: LTCTVStandard::LTCTV_625_50,
            flags: LtcBgFlags::default(),
        };
        let mut encoder = LTCEncoder::try_new(&encoder_config).unwrap();
        assert_eq!(encoder.get_buffersize(), 1921);

        // The buffersize calculation is:
        // size_t bufsize = 1 + ceil(sample_rate / fps);

        // We explicitly set the buffersize to the appropiate value
        // which means the reinit wont fail
        encoder.set_buffersize(192_000.0, 25.0).unwrap();
        assert_eq!(encoder.get_buffersize(), 7681);
        assert!(encoder
            .reinit(192_000.0, 25.0, LTCTVStandard::LTCTV_525_60, 0.into())
            .is_ok());

        // Now the buffersize should be smaller because the fps are higher
        // we deliberately set the wrong number of frames to cause an error
        encoder.set_buffersize(192_000.0, 30.0).unwrap();
        assert_eq!(encoder.get_buffersize(), 6401);
        assert!(encoder
            .reinit(192_000.0, 25.0, LTCTVStandard::LTCTV_525_60, 0.into())
            .is_err());
    }
}

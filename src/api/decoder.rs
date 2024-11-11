use super::frame::LTCFrameExt;
use super::SampleType;
use crate::error::LTCDecoderError;

use crate::raw;
#[derive(Debug)]
pub struct LTCDecoder {
    inner_unsafe_ptr: *mut raw::LTCDecoder,
}

impl Drop for LTCDecoder {
    fn drop(&mut self) {
        unsafe {
            raw::ltc_decoder_free(self.inner_unsafe_ptr);
        }
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
                inner_unsafe_ptr: decoder,
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
                self.inner_unsafe_ptr,
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
                self.inner_unsafe_ptr,
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
                self.inner_unsafe_ptr,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn write_i16(&mut self, buf: &[i16], posinfo: i64) {
        // SAFETY: We own self. buf is only read.
        unsafe {
            // SAFETY: we can cast *const SampleType as *mut SampleType to accomodate for the C function
            // signature, we assume that ltc_decoder_write is will only read from it
            let mut_ptr_buf = buf.as_ptr() as *mut i16;
            raw::ltc_decoder_write_s16(
                self.inner_unsafe_ptr,
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
                self.inner_unsafe_ptr,
                mut_ptr_buf,
                buf.len() as libc::size_t,
                posinfo,
            );
        }
    }

    pub fn read(&self) -> Option<LTCFrameExt> {
        // SAFE: The LTCFrameExt is allocated in a box so it outlives the function
        let mut frame = LTCFrameExt::default();

        // SAFETY: We own frame. The function is assumed to only read from self and write to frame
        let result = unsafe {
            #[allow(clippy::needless_borrow)] // for clarity
            raw::ltc_decoder_read(self.inner_unsafe_ptr, (&mut frame).inner_unsafe_ptr)
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
            raw::ltc_decoder_queue_flush(self.inner_unsafe_ptr);
        }
    }

    pub fn queue_length(&self) -> i32 {
        // SAFETY: The function is assumed to only read self
        unsafe { raw::ltc_decoder_queue_length(self.inner_unsafe_ptr) }
    }
}

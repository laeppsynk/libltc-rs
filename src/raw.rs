// raw.rs

extern crate libc;

use libc::{c_double, c_int, c_short, c_ulong, c_ushort, size_t};

use crate::{LTCFrame, LTCFrameExt, SMPTETimecode};

#[repr(C)]
pub(crate) struct LTCEncoder {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct LTCDecoder {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type ltc_off_t = i64;
#[allow(non_camel_case_types)]
pub type ltcsnd_sample_t = i32;

#[repr(C)]
#[allow(non_camel_case_types)]
pub(crate) enum LTC_TV_STANDARD {
    LTC_TV_525_60,  // 30fps
    LTC_TV_625_50,  // 25fps
    LTC_TV_1125_60, // 30fps
    LTC_TV_FILM_24, // 24fps
}

#[link(name = "ltc")]
extern "C" {
    // Frame operations
    pub(crate) fn ltc_frame_to_time(
        stime: *mut SMPTETimecode,
        frame: *const LTCFrame,
        flags: c_int,
    );
    pub(crate) fn ltc_time_to_frame(
        frame: *mut LTCFrame,
        stime: *const SMPTETimecode,
        standard: LTC_TV_STANDARD,
        flags: c_int,
    );
    pub(crate) fn ltc_frame_reset(frame: *mut LTCFrame);
    pub(crate) fn ltc_frame_increment(
        frame: *mut LTCFrame,
        fps: c_int,
        standard: LTC_TV_STANDARD,
        flags: c_int,
    ) -> c_int;
    pub(crate) fn ltc_frame_decrement(
        frame: *mut LTCFrame,
        fps: c_int,
        standard: LTC_TV_STANDARD,
        flags: c_int,
    ) -> c_int;
    pub(crate) fn ltc_frame_set_parity(frame: *mut LTCFrame, standard: LTC_TV_STANDARD);
    pub(crate) fn ltc_frame_parse_bcg_flags(
        frame: *mut LTCFrame,
        standard: LTC_TV_STANDARD,
    ) -> c_int;
    pub(crate) fn ltc_frame_get_user_bits(frame: *const LTCFrame) -> c_ulong;
    pub(crate) fn ltc_frame_alignment(
        samples_per_frame: c_double,
        standard: LTC_TV_STANDARD,
    ) -> ltc_off_t;

    // Decoder operations
    pub(crate) fn ltc_decoder_create(apv: c_int, queue_size: c_int) -> *mut LTCDecoder;
    pub(crate) fn ltc_decoder_free(decoder: *mut LTCDecoder) -> c_int;
    pub(crate) fn ltc_decoder_write(
        decoder: *mut LTCDecoder,
        buf: *const ltcsnd_sample_t,
        size: size_t,
        posinfo: ltc_off_t,
    );
    pub(crate) fn ltc_decoder_write_double(
        decoder: *mut LTCDecoder,
        buf: *const c_double,
        size: size_t,
        posinfo: ltc_off_t,
    );
    pub(crate) fn ltc_decoder_write_float(
        decoder: *mut LTCDecoder,
        buf: *const f32,
        size: size_t,
        posinfo: ltc_off_t,
    );
    pub(crate) fn ltc_decoder_write_s16(
        decoder: *mut LTCDecoder,
        buf: *const c_short,
        size: size_t,
        posinfo: ltc_off_t,
    );
    pub(crate) fn ltc_decoder_write_u16(
        decoder: *mut LTCDecoder,
        buf: *const c_ushort,
        size: size_t,
        posinfo: ltc_off_t,
    );
    pub(crate) fn ltc_decoder_read(decoder: *mut LTCDecoder, frame: *mut LTCFrameExt) -> c_int;
    pub(crate) fn ltc_decoder_queue_flush(decoder: *mut LTCDecoder);
    pub(crate) fn ltc_decoder_queue_length(decoder: *mut LTCDecoder) -> c_int;

    // Encoder operations
    pub(crate) fn ltc_encoder_create(
        sample_rate: c_double,
        fps: c_double,
        standard: LTC_TV_STANDARD,
        flags: c_int,
    ) -> *mut LTCEncoder;
    pub(crate) fn ltc_encoder_free(encoder: *mut LTCEncoder);
    pub(crate) fn ltc_encoder_set_timecode(encoder: *mut LTCEncoder, t: *const SMPTETimecode);
    pub(crate) fn ltc_encoder_get_timecode(encoder: *mut LTCEncoder, t: *mut SMPTETimecode);
    pub(crate) fn ltc_encoder_set_user_bits(encoder: *mut LTCEncoder, data: c_ulong);
    pub(crate) fn ltc_encoder_inc_timecode(encoder: *mut LTCEncoder) -> c_int;
    pub(crate) fn ltc_encoder_dec_timecode(encoder: *mut LTCEncoder) -> c_int;
    pub(crate) fn ltc_encoder_set_frame(encoder: *mut LTCEncoder, frame: *const LTCFrame);
    pub(crate) fn ltc_encoder_get_frame(encoder: *mut LTCEncoder, frame: *mut LTCFrame);
    pub(crate) fn ltc_encoder_get_buffer(
        encoder: *mut LTCEncoder,
        buf: *mut ltcsnd_sample_t,
    ) -> c_int;
    pub(crate) fn ltc_encoder_get_bufptr(
        encoder: *mut LTCEncoder,
        size: *mut c_int,
        flush: c_int,
    ) -> *mut ltcsnd_sample_t;
    pub(crate) fn ltc_encoder_buffer_flush(encoder: *mut LTCEncoder);
    pub(crate) fn ltc_encoder_get_buffersize(encoder: *mut LTCEncoder) -> size_t;
    pub(crate) fn ltc_encoder_reinit(
        encoder: *mut LTCEncoder,
        sample_rate: c_double,
        fps: c_double,
        standard: LTC_TV_STANDARD,
        flags: c_int,
    ) -> c_int;
    pub(crate) fn ltc_encoder_reset(encoder: *mut LTCEncoder);
    pub(crate) fn ltc_encoder_set_bufsize(
        encoder: *mut LTCEncoder,
        sample_rate: c_double,
        fps: c_double,
    ) -> c_int;
    pub(crate) fn ltc_encoder_get_volume(encoder: *mut LTCEncoder) -> c_double;
    pub(crate) fn ltc_encoder_set_volume(encoder: *mut LTCEncoder, dbfs: c_double) -> c_int;
    pub(crate) fn ltc_encoder_get_filter(encoder: *mut LTCEncoder) -> c_double;
    pub(crate) fn ltc_encoder_set_filter(encoder: *mut LTCEncoder, rise_time: c_double);
    pub(crate) fn ltc_encoder_encode_byte(
        encoder: *mut LTCEncoder,
        byte: c_int,
        speed: c_double,
    ) -> c_int;
    pub(crate) fn ltc_encoder_end_encode(encoder: *mut LTCEncoder) -> c_int;
    pub(crate) fn ltc_encoder_encode_frame(encoder: *mut LTCEncoder);
    pub(crate) fn ltc_encoder_encode_reversed_frame(encoder: *mut LTCEncoder);
}

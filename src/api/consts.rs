use crate::raw;

pub const LTC_H: u32 = raw::LTC_H;
pub const LIBLTC_VERSION: &[u8; 6] = raw::LIBLTC_VERSION;
pub const LIBLTC_VERSION_MAJOR: u32 = raw::LIBLTC_VERSION_MAJOR;
pub const LIBLTC_VERSION_MINOR: u32 = raw::LIBLTC_VERSION_MINOR;
pub const LIBLTC_VERSION_MICRO: u32 = raw::LIBLTC_VERSION_MICRO;
pub const LIBLTC_CUR: u32 = raw::LIBLTC_CUR;
pub const LIBLTC_REV: u32 = raw::LIBLTC_REV;
pub const LIBLTC_AGE: u32 = raw::LIBLTC_AGE;
pub const LTC_FRAME_BIT_COUNT: u32 = raw::LTC_FRAME_BIT_COUNT;
pub type WcharT = raw::wchar_t;
pub const LTC_BG_FLAGS_LTC_USE_DATE: LtcBgFlags = raw::LTC_BG_FLAGS_LTC_USE_DATE;
pub const LTC_BG_FLAGS_LTC_TC_CLOCK: LtcBgFlags = raw::LTC_BG_FLAGS_LTC_TC_CLOCK;
pub const LTC_BG_FLAGS_LTC_BGF_DONT_TOUCH: LtcBgFlags = raw::LTC_BG_FLAGS_LTC_BGF_DONT_TOUCH;
pub const LTC_BG_FLAGS_LTC_NO_PARITY: LtcBgFlags = raw::LTC_BG_FLAGS_LTC_NO_PARITY;
pub type LtcBgFlags = raw::LTC_BG_FLAGS;

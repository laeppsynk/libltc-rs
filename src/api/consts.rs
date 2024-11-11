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

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LtcBgFlags {
    LTC_USE_DATE = raw::LTC_BG_FLAGS_LTC_USE_DATE,
    LTC_TC_CLOCK = raw::LTC_BG_FLAGS_LTC_TC_CLOCK,
    LTC_BGF_DONT_TOUCH = raw::LTC_BG_FLAGS_LTC_BGF_DONT_TOUCH,
    LTC_NO_PARITY = raw::LTC_BG_FLAGS_LTC_NO_PARITY,
}

impl LtcBgFlags {
    // Check if a flag is set using bitwise AND
    pub fn contains(flags: u32, flag: LtcBgFlags) -> bool {
        flags & (flag as u32) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ltc_bg_flags() {
        let flags = LtcBgFlags::LTC_USE_DATE as u32 | LtcBgFlags::LTC_TC_CLOCK as u32;
        assert!(LtcBgFlags::contains(flags, LtcBgFlags::LTC_USE_DATE));
        assert!(!LtcBgFlags::contains(flags, LtcBgFlags::LTC_BGF_DONT_TOUCH));
    }

    #[test]
    fn test_ltc_bg_flags_negative() {
        let flags = LtcBgFlags::LTC_USE_DATE as u32 | LtcBgFlags::LTC_TC_CLOCK as u32;
        assert!(!LtcBgFlags::contains(flags, LtcBgFlags::LTC_BGF_DONT_TOUCH));
    }
}

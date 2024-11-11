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
pub enum LtcBgFlagsKind {
    LTC_USE_DATE = raw::LTC_BG_FLAGS_LTC_USE_DATE,
    LTC_TC_CLOCK = raw::LTC_BG_FLAGS_LTC_TC_CLOCK,
    LTC_BGF_DONT_TOUCH = raw::LTC_BG_FLAGS_LTC_BGF_DONT_TOUCH,
    LTC_NO_PARITY = raw::LTC_BG_FLAGS_LTC_NO_PARITY,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LtcBgFlags(u32);

impl LtcBgFlags {
    // Check if a flag is set using bitwise AND
    pub fn contains(&self, flag: LtcBgFlagsKind) -> bool {
        self.0 & (flag as u32) != 0
    }

    pub fn new(flags: u32) -> Self {
        LtcBgFlags(flags)
    }

    pub fn set(&mut self, flag: LtcBgFlagsKind) -> &mut Self {
        self.0 |= flag as u32;
        self
    }

    pub fn unset(&mut self, flag: LtcBgFlagsKind) -> &mut Self {
        self.0 &= !(flag as u32);
        self
    }
}

impl Default for LtcBgFlags {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<u32> for LtcBgFlags {
    fn from(flags: u32) -> Self {
        LtcBgFlags(flags)
    }
}

impl From<i32> for LtcBgFlags {
    fn from(flags: i32) -> Self {
        LtcBgFlags(flags as u32)
    }
}

impl From<LtcBgFlags> for i32 {
    fn from(val: LtcBgFlags) -> Self {
        val.0 as i32
    }
}

impl From<LtcBgFlags> for u32 {
    fn from(val: LtcBgFlags) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ltc_bg_flags() {
        let mut flags: LtcBgFlags =
            (LtcBgFlagsKind::LTC_USE_DATE as u32 | LtcBgFlagsKind::LTC_TC_CLOCK as u32).into();

        assert!(flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        flags.set(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH);
        assert!(flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        flags.set(LtcBgFlagsKind::LTC_NO_PARITY);
        assert!(flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        flags.unset(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH);
        assert!(flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        flags.unset(LtcBgFlagsKind::LTC_NO_PARITY);
        assert!(flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        flags.unset(LtcBgFlagsKind::LTC_USE_DATE);
        assert!(!flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        flags.unset(LtcBgFlagsKind::LTC_TC_CLOCK);
        assert!(!flags.contains(LtcBgFlagsKind::LTC_USE_DATE));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_TC_CLOCK));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_BGF_DONT_TOUCH));
        assert!(!flags.contains(LtcBgFlagsKind::LTC_NO_PARITY));

        assert_eq!(flags, LtcBgFlags::default());
    }
}

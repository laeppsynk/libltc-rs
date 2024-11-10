pub(crate) use autogen::*;

// Several functions in this library are deprecated. We allow dead code to avoid warnings.
#[allow(dead_code)]
mod autogen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/* For some reason, the generated code does not have default implementations for the structs.
* This is very annoying specially for bitwise structs, hence why I add them here */
mod additions {
    use super::autogen;

    #[allow(clippy::derivable_impls)]
    impl Default for autogen::LTCFrame {
        fn default() -> Self {
            Self {
                _bitfield_align_1: Default::default(),
                _bitfield_1: Default::default(),
                __bindgen_padding_0: Default::default(),
            }
        }
    }

    impl Default for autogen::LTCFrameExt {
        fn default() -> Self {
            Self {
                ltc: Default::default(),
                off_start: Default::default(),
                off_end: Default::default(),
                reverse: Default::default(),
                biphase_tics: [Default::default(); 80],
                sample_min: Default::default(),
                sample_max: Default::default(),
                volume: Default::default(),
            }
        }
    }

    #[allow(clippy::derivable_impls)]
    impl Default for autogen::SMPTETimecode {
        fn default() -> Self {
            Self {
                timezone: Default::default(),
                years: Default::default(),
                months: Default::default(),
                days: Default::default(),
                hours: Default::default(),
                mins: Default::default(),
                secs: Default::default(),
                frame: Default::default(),
            }
        }
    }
}

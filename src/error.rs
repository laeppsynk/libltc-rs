use std::error::Error;

// error.rs
#[derive(Debug)]
pub enum LTCEncoderError {
    CreateError,
    ReinitError,
    BufferSizeError,
    VolumeError,
    EncodeError,
    TimecodeError(TimecodeError),
}

#[derive(Debug)]
pub enum LTCDecoderError {
    CreateError,
    TImecodeError(TimecodeError),
}

#[derive(Debug)]
pub enum TimecodeError {
    InvalidReturn,
}

impl Error for LTCEncoderError {}
impl Error for LTCDecoderError {}
impl Error for TimecodeError {}

impl From<TimecodeError> for LTCEncoderError {
    fn from(e: TimecodeError) -> Self {
        LTCEncoderError::TimecodeError(e)
    }
}

impl From<TimecodeError> for LTCDecoderError {
    fn from(e: TimecodeError) -> Self {
        LTCDecoderError::TImecodeError(e)
    }
}

impl std::fmt::Display for LTCEncoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LTCEncoderError::CreateError => write!(f, "Error creating LTC encoder"),
            LTCEncoderError::ReinitError => write!(f, "Error reinitializing LTC encoder"),
            LTCEncoderError::BufferSizeError => write!(f, "Error setting buffer size"),
            LTCEncoderError::VolumeError => write!(f, "Error setting volume"),
            LTCEncoderError::EncodeError => write!(f, "Error during encoding"),
            LTCEncoderError::TimecodeError(e) => write!(f, "Timecode error: {}", e),
        }
    }
}

impl std::fmt::Display for LTCDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LTCDecoderError::CreateError => write!(f, "Error creating LTC decoder"),
            LTCDecoderError::TImecodeError(e) => write!(f, "Timecode error: {}", e),
        }
    }
}

impl std::fmt::Display for TimecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TimecodeError::InvalidReturn => write!(f, "Invalid return value from C function"),
        }
    }
}

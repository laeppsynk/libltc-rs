// error.rs
#[derive(Debug)]
pub enum LTCEncoderError {
    CreateError,
    ReinitError,
    BufferSizeError,
    VolumeError,
    EncodeError,
}

#[derive(Debug)]
pub enum LTCDecoderError {
    CreateError,
}

impl std::fmt::Display for LTCEncoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LTCEncoderError::CreateError => write!(f, "Error creating LTC encoder"),
            LTCEncoderError::ReinitError => write!(f, "Error reinitializing LTC encoder"),
            LTCEncoderError::BufferSizeError => write!(f, "Error setting buffer size"),
            LTCEncoderError::VolumeError => write!(f, "Error setting volume"),
            LTCEncoderError::EncodeError => write!(f, "Error during encoding"),
        }
    }
}

impl std::fmt::Display for LTCDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LTCDecoderError::CreateError => write!(f, "Error creating LTC decoder"),
        }
    }
}

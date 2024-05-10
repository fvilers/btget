use crate::bencode::DecodeError;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum RunError {
    IO(io::Error),
    Decode(DecodeError),
}

impl error::Error for RunError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IO(e) => Some(e),
            Self::Decode(e) => Some(e),
        }
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(e) => e.fmt(f),
            Self::Decode(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for RunError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<DecodeError> for RunError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

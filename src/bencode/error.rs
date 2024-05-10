use std::{error, fmt, num, str};

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnexpectedByte((u8, usize)),
    UnexpectedEndOfFile,
    Utf8(str::Utf8Error),
    ParseInt(num::ParseIntError),
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::UnexpectedByte(_) => None,
            Self::UnexpectedEndOfFile => None,
            Self::Utf8(e) => Some(e),
            Self::ParseInt(e) => Some(e),
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedByte((byte, index)) => {
                write!(f, "Unexpected byte 0x{byte:02x} at index {index}")
            }
            Self::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            Self::Utf8(e) => e.fmt(f),
            Self::ParseInt(e) => e.fmt(f),
        }
    }
}

impl From<str::Utf8Error> for DecodeError {
    fn from(value: str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<num::ParseIntError> for DecodeError {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

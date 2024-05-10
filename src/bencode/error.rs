use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnexpectedByte((u8, usize)),
    UnexpectedEndOfFile,
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::UnexpectedByte(_) => None,
            Self::UnexpectedEndOfFile => None,
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
        }
    }
}

use std::{error, fmt};

#[derive(Debug)]
pub struct DecodeError;

impl error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not decode file.")
    }
}

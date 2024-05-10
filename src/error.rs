use std::{error, fmt};

#[derive(Debug)]
pub struct RunError;

impl error::Error for RunError {}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error has occurred")
    }
}

pub use self::{error::DecodeError, value::Value};

mod error;
mod value;

pub fn decode(source: &[u8]) -> Result<Value, DecodeError> {
    todo!()
}

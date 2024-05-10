pub use self::{error::DecodeError, value::Value};

mod error;
mod value;

type DecodeResult = Result<Value, DecodeError>;

pub fn decode(source: &[u8]) -> DecodeResult {
    decode_bytes(source)
}

fn decode_bytes(source: &[u8]) -> DecodeResult {
    match source.first() {
        Some(byte) => Err(DecodeError::UnexpectedByte((*byte, 0))),
        None => Err(DecodeError::UnexpectedEndOfFile),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_bytes_should_return_an_error_for_unexpected_byte() {
        let byte = b'x';
        let result = decode_bytes(&[byte]).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((byte, 0)));
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_unexpected_eof() {
        let result = decode_bytes(&[]).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedEndOfFile);
    }
}

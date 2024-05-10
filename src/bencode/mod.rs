pub use self::{error::DecodeError, value::Value};
use std::str;

mod error;
mod value;

type DecodeResult<'a> = Result<Value<'a>, DecodeError>;

pub fn decode(source: &[u8]) -> DecodeResult {
    decode_bytes(source)
}

fn decode_bytes(source: &[u8]) -> DecodeResult {
    match source.first() {
        Some(b'i') => decode_integer(&source[1..]),
        Some(byte) if byte.is_ascii_digit() => decode_byte_string(source),
        Some(byte) => Err(DecodeError::UnexpectedByte((*byte, 0))),
        None => Err(DecodeError::UnexpectedEndOfFile),
    }
}

fn decode_integer(source: &[u8]) -> DecodeResult {
    let mut index = 0;

    loop {
        match source.get(index) {
            Some(b'-') if index == 0 => index += 1,
            Some(byte) if byte.is_ascii_digit() => index += 1,
            Some(b'e') => {
                let value = str::from_utf8(&source[..index])?.parse()?;

                break Ok(Value::Integer(value));
            }
            Some(byte) => break Err(DecodeError::UnexpectedByte((*byte, index))),
            None => break Err(DecodeError::UnexpectedEndOfFile),
        }
    }
}

fn decode_byte_string(source: &[u8]) -> DecodeResult {
    let mut index = 0;

    loop {
        match source.get(index) {
            Some(byte) if byte.is_ascii_digit() => index += 1,
            Some(b':') => {
                let length = str::from_utf8(&source[..index])?.parse()?;
                let start = index + 1;
                let value = source
                    .get(start..start + length)
                    .ok_or(DecodeError::InvalidByteStringLength((length, index)))?;

                break Ok(Value::ByteString(value));
            }
            Some(byte) => break Err(DecodeError::UnexpectedByte((*byte, index))),
            None => break Err(DecodeError::UnexpectedEndOfFile),
        }
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

    #[test]
    fn decode_integer_should_return_an_error_for_unexpected_byte() {
        let result = decode_integer("4x2".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((b'x', 1)));
    }

    #[test]
    fn decode_integer_should_return_an_error_for_unexpected_eof() {
        let result = decode_integer("42".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedEndOfFile);
    }

    #[test]
    fn decode_integer_should_return_an_error_for_unexpected_minus_sign() {
        let result = decode_integer("42-".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((b'-', 2)));
    }

    #[test]
    fn decode_integer_should_return_the_integer_value() {
        let result = decode_integer("42e".as_bytes()).unwrap();

        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn decode_integer_should_return_the_0_value() {
        let result = decode_integer("0e".as_bytes()).unwrap();

        assert_eq!(result, Value::Integer(0));
    }

    #[test]
    fn decode_integer_should_return_a_negative_integer_value() {
        let result = decode_integer("-42e".as_bytes()).unwrap();

        assert_eq!(result, Value::Integer(-42));
    }

    #[test]
    fn decode_byte_string_should_return_an_error_for_unexpected_byte() {
        let result = decode_byte_string("4x:spam".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((b'x', 1)));
    }

    #[test]
    fn decode_byte_string_should_return_an_error_for_unexpected_eof() {
        let result = decode_byte_string("4".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedEndOfFile);
    }

    #[test]
    fn decode_byte_string_should_return_an_error_for_invalid_length() {
        let result = decode_byte_string("42:spam".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::InvalidByteStringLength((42, 2)));
    }

    #[test]
    fn decode_byte_string_should_return_a_0_length_byte_string() {
        let result = decode_byte_string("0:".as_bytes()).unwrap();

        assert_eq!(result, Value::ByteString(&[]));
    }

    #[test]
    fn decode_byte_string_should_return_the_byte_string() {
        let result = decode_byte_string("4:spam".as_bytes()).unwrap();

        assert_eq!(result, Value::ByteString("spam".as_bytes()));
    }
}

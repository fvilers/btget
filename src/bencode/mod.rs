pub use self::{error::DecodeError, value::Value};
use std::str;

mod error;
mod value;

pub fn decode(source: &[u8]) -> Result<Value, DecodeError> {
    decode_bytes(source).map(|(value, _)| value)
}

type DecodeResult<'a> = Result<(Value<'a>, usize), DecodeError>;

fn decode_bytes(source: &[u8]) -> DecodeResult {
    let inc_offset = |(value, offset)| (value, offset + 1);

    match source.first() {
        Some(b'i') => decode_integer(&source[1..]).map(inc_offset),
        Some(byte) if byte.is_ascii_digit() => decode_byte_string(source),
        Some(b'l') => decode_list(&source[1..]).map(inc_offset),
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

                break Ok((Value::Integer(value), index + 1));
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
                let end = start + length;
                let value = source
                    .get(start..end)
                    .ok_or(DecodeError::InvalidByteStringLength((length, index)))?;

                break Ok((Value::ByteString(value), end));
            }
            Some(byte) => break Err(DecodeError::UnexpectedByte((*byte, index))),
            None => break Err(DecodeError::UnexpectedEndOfFile),
        }
    }
}

fn decode_list(source: &[u8]) -> DecodeResult {
    let mut index = 0;
    let mut values = Vec::new();

    loop {
        match source.get(index) {
            Some(b'e') => break Ok((Value::List(values), index + 1)),
            Some(_) => {
                let (value, offset) = decode_bytes(&source[index..])?;
                index += offset;

                values.push(value);
            }
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
    fn decode_bytes_should_return_an_error_for_integer_unexpected_byte() {
        let result = decode_bytes("i4x2e".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((b'x', 1)));
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_integer_unexpected_eof() {
        let result = decode_bytes("i42".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedEndOfFile);
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_unexpected_minus_sign() {
        let result = decode_bytes("i42-".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((b'-', 2)));
    }

    #[test]
    fn decode_bytes_should_return_the_integer_value() {
        let (result, _) = decode_bytes("i42e".as_bytes()).unwrap();

        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn decode_bytes_should_return_the_0_value() {
        let (result, _) = decode_bytes("i0e".as_bytes()).unwrap();

        assert_eq!(result, Value::Integer(0));
    }

    #[test]
    fn decode_bytes_should_return_a_negative_integer_value() {
        let (result, _) = decode_bytes("i-42e".as_bytes()).unwrap();

        assert_eq!(result, Value::Integer(-42));
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_byte_string_unexpected_byte() {
        let result = decode_bytes("4x:spam".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedByte((b'x', 1)));
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_byte_string_unexpected_eof() {
        let result = decode_bytes("4".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedEndOfFile);
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_invalid_length() {
        let result = decode_bytes("42:spam".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::InvalidByteStringLength((42, 2)));
    }

    #[test]
    fn decode_bytes_should_return_a_0_length_byte_string() {
        let (result, _) = decode_bytes("0:".as_bytes()).unwrap();

        assert_eq!(result, Value::ByteString(&[]));
    }

    #[test]
    fn decode_bytes_should_return_the_byte_string() {
        let (result, _) = decode_bytes("4:spam".as_bytes()).unwrap();

        assert_eq!(result, Value::ByteString("spam".as_bytes()));
    }

    #[test]
    fn decode_bytes_should_return_an_error_for_list_unexpected_eof() {
        let result = decode_bytes("l".as_bytes()).unwrap_err();

        assert_eq!(result, DecodeError::UnexpectedEndOfFile);
    }

    #[test]
    fn decode_bytes_should_return_the_list() {
        let (result, _) = decode_bytes("l4:spami42ee".as_bytes()).unwrap();

        assert_eq!(
            result,
            Value::List(Vec::from([
                Value::ByteString("spam".as_bytes()),
                Value::Integer(42)
            ]))
        );
    }
}

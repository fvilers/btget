use std::{borrow::Cow, collections::BTreeMap};

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Integer(isize),
    ByteString(&'a [u8]),
    List(Vec<Value<'a>>),
    Dictionary(BTreeMap<Cow<'a, str>, Value<'a>>),
}

impl<'a> Value<'a> {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::Integer(value) => format!("i{value}e").as_bytes().to_vec(),
            Self::ByteString(value) => {
                [value.len().to_string().as_bytes(), &[b':'], value].concat()
            }
            Self::List(values) => [
                &[b'l'],
                values
                    .iter()
                    .flat_map(|value| value.encode())
                    .collect::<Vec<u8>>()
                    .as_slice(),
                &[b'e'],
            ]
            .concat(),
            Self::Dictionary(values) => [
                &[b'd'],
                values
                    .iter()
                    .flat_map(|(k, v)| [Value::ByteString(k.as_bytes()).encode(), v.encode()])
                    .flatten()
                    .collect::<Vec<u8>>()
                    .as_slice(),
                &[b'e'],
            ]
            .concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_should_return_an_encoded_integer() {
        let result = Value::Integer(42).encode();
        assert_eq!(result, "i42e".as_bytes());
    }

    #[test]
    fn encode_should_return_an_encoded_byte_string() {
        let result = Value::ByteString("spam".as_bytes()).encode();
        assert_eq!(result, "4:spam".as_bytes());
    }

    #[test]
    fn encode_should_return_an_encoded_list() {
        let result = Value::List(Vec::from([
            Value::ByteString("spam".as_bytes()),
            Value::Integer(42),
        ]))
        .encode();
        assert_eq!(result, "l4:spami42ee".as_bytes());
    }

    #[test]
    fn encode_should_return_an_encoded_dictionary() {
        let result = Value::Dictionary(BTreeMap::from([
            (Cow::from("bar"), Value::ByteString("spam".as_bytes())),
            (Cow::from("foo"), Value::Integer(42)),
        ]))
        .encode();
        assert_eq!(result, "d3:bar4:spam3:fooi42ee".as_bytes());
    }
}

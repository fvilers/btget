use crate::bencode::Value;
use sha1::{Digest, Sha1};
use std::{error, fmt};

#[derive(Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
    pub info_hash: Vec<u8>,
}

#[derive(Debug)]
pub struct Info {
    pub length: isize,
    pub name: String,
    pub piece_length: isize,
    pub pieces: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum TorrentError {
    InvalidMetadata,
}

impl error::Error for TorrentError {}

impl fmt::Display for TorrentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetadata => write!(f, "Invalid torrent metadata"),
        }
    }
}

impl<'a> TryFrom<Value<'a>> for Torrent {
    type Error = TorrentError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let Value::Dictionary(dict) = value else {
            return Err(TorrentError::InvalidMetadata);
        };
        let Some(Value::ByteString(announce)) = dict.get("announce") else {
            return Err(TorrentError::InvalidMetadata);
        };
        let Some(info_value) = dict.get("info") else {
            return Err(TorrentError::InvalidMetadata);
        };

        let info = Info::try_from(info_value)?;
        let info_hash = compute_hash(&info_value.encode());

        Ok(Self {
            announce: String::from_utf8(announce.to_vec())
                .map_err(|_| TorrentError::InvalidMetadata)?,
            info,
            info_hash,
        })
    }
}

impl<'a> TryFrom<&Value<'a>> for Info {
    type Error = TorrentError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Value::Dictionary(info) = value else {
            return Err(TorrentError::InvalidMetadata);
        };
        let Some(Value::Integer(length)) = info.get("length") else {
            return Err(TorrentError::InvalidMetadata);
        };
        let Some(Value::ByteString(name)) = info.get("name") else {
            return Err(TorrentError::InvalidMetadata);
        };
        let Some(Value::Integer(piece_length)) = info.get("piece length") else {
            return Err(TorrentError::InvalidMetadata);
        };
        let Some(Value::ByteString(pieces)) = info.get("pieces") else {
            return Err(TorrentError::InvalidMetadata);
        };

        Ok(Self {
            length: *length,
            name: String::from_utf8(name.to_vec()).map_err(|_| TorrentError::InvalidMetadata)?,
            piece_length: *piece_length,
            pieces: pieces.chunks(20).map(|chunk| chunk.to_vec()).collect(),
        })
    }
}

fn compute_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();

    result.to_vec()
}

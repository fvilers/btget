use crate::{error::RunError, torrent::Torrent};
use std::{fmt, fs};

mod bencode;
mod error;
mod torrent;

struct Bytes(Vec<u8>);

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

pub fn run(file_name: String) -> Result<(), RunError> {
    let content = fs::read(file_name)?;
    let metadata = bencode::decode(&content)?;
    let torrent: Torrent = metadata.try_into()?;

    println!("Announce: {}", torrent.announce);
    println!("Info:");
    println!("\tLength: {} bytes", torrent.info.length);
    println!("\tName: {}", torrent.info.name);
    println!("\tPiece length: {} bytes", torrent.info.piece_length);
    println!("\tPieces: {}", torrent.info.pieces.len());
    println!("Info hash: {}", Bytes(torrent.info_hash));

    Ok(())
}

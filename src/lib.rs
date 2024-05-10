use crate::{error::RunError, torrent::Torrent};
use std::fs;

mod bencode;
mod error;
mod torrent;

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

    Ok(())
}

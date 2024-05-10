use error::RunError;
use std::fs;

mod bencode;
mod error;

pub fn run(file_name: String) -> Result<(), RunError> {
    let content = fs::read(file_name)?;
    let metadata = bencode::decode(&content)?;

    println!("{metadata:?}");

    Ok(())
}

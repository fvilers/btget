use error::RunError;
use std::fs;

mod error;

pub fn run(file_name: String) -> Result<(), RunError> {
    let content = fs::read(&file_name)?;

    println!("Read {} bytes from {file_name}", content.len());

    Ok(())
}

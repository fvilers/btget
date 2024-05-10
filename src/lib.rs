use error::RunError;

mod error;

pub fn run(file_name: String) -> Result<(), RunError> {
    println!("File name: {file_name}");

    Ok(())
}

use error::RunError;

mod error;

pub fn run() -> Result<(), RunError> {
    println!("Hello, world!");

    Ok(())
}

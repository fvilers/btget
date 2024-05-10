use btget::run;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let Some(file_name) = env::args().nth(1) else {
        eprintln!("Usage: btget <file_name.torrent>");
        return ExitCode::FAILURE;
    };

    match run(file_name) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

use btget::run;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

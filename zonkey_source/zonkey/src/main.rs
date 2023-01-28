mod wrapper;

use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    if let Some(file) = args().skip(1).next() {
        wrapper::run(file)
    } else {
        eprintln!("Error: Missing source file argument\nUsage: zonkey <FILE_PATH>");
        ExitCode::FAILURE
    }
}

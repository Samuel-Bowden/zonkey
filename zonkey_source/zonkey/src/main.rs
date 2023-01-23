mod wrapper;

use std::{process::ExitCode, env::args};
use wrapper::Wrapper;

fn main() -> ExitCode {
    if let Some(file) = args().skip(1).next() {
        Wrapper::new().run(file)
    } else {
        eprintln!("Error: Missing source file argument\nUsage: zonkey <FILE_PATH>");
        ExitCode::FAILURE
    }
}

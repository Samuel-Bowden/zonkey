use interpreter::err;
use std::{env::args, fs::read_to_string, path::Path, process::ExitCode};
use unicode_segmentation::UnicodeSegmentation;

fn main() -> ExitCode {
    if let Some(file) = args().skip(1).next() {
        run(file)
    } else {
        eprintln!("Error: Missing source file argument\nUsage: zonkey <FILE_PATH>");
        ExitCode::FAILURE
    }
}

fn run(file: String) -> ExitCode {
    let source = match read_to_string(Path::new(&file)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Couldn't open file: {e}");
            // Exit code for non-existent or unreadable file
            return ExitCode::from(66);
        }
    };

    let graphemes = UnicodeSegmentation::graphemes(source.as_str(), true).collect::<Vec<&str>>();

    match interpreter::run(&graphemes) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            err::handler::run(e, &graphemes);
            ExitCode::FAILURE
        }
    }
}

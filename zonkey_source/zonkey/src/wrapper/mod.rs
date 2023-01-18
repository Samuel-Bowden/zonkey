use std::{fs::read_to_string, io::Write, path::Path, process::ExitCode};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct Wrapper {
    stderr: StandardStream,
}

impl Wrapper {
    pub fn new() -> Self {
        Self {
            stderr: StandardStream::stderr(termcolor::ColorChoice::Always),
        }
    }

    pub fn run(&mut self, file: String) -> ExitCode {
        let source = match read_to_string(Path::new(&file)) {
            Ok(s) => s,
            Err(e) => {
                self.error(format!("Couldn't open file: {e}"));
                // Exit code for non-existent or unreadable file
                return ExitCode::from(66);
            }
        };

        match interpreter::run(&source) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                self.error(format!("{e}"));
                ExitCode::FAILURE
            }
        }
    }

    fn error(&mut self, string: String) {
        self.stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .expect("Failed to change the color of stderr.");

        write!(&mut self.stderr, "(ERROR)").expect("Failed to write `(ERROR)` to stderr.");

        self.stderr
            .reset()
            .expect("Failed to reset color of stderr.");

        writeln!(&mut self.stderr, " {string}").expect("Failed to write error message to stderr.")
    }
}

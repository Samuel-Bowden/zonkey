use std::{process::ExitCode, io::{stdin, Write}, fs::read_to_string, path::Path};
use interpreter::Interpreter;
use termcolor::{StandardStream, Color, ColorSpec, WriteColor};

pub struct Shell {
    debug: bool,
    stderr: StandardStream,
    stdout: StandardStream,
}

impl Shell {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            stderr: StandardStream::stderr(termcolor::ColorChoice::Always),
            stdout: StandardStream::stdout(termcolor::ColorChoice::Always),
        }
    }

    pub fn prompt(&mut self) -> ExitCode {
        loop {
            self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).expect("Failed to change the color of stdout.");
            write!(&mut self.stdout, "> ").expect("Failed to write `>` to stdout.");
            self.stdout.reset().expect("Failed to reset color of stderr.");

            self.stdout
                .flush()
                .expect("Failed to flush stdout for prompt.");

            let mut input = String::new();

            stdin()
                .read_line(&mut input)
                .expect("Failed to read line for input.");

            let command = input.trim();

            if command == "exit" {
                break;
            }

            if let Err(e) = Interpreter::new(self.debug, command).run() {
                self.error(format!("{}", e));
            }
        }

        ExitCode::SUCCESS
    }

    pub fn file(&mut self, file: String) -> ExitCode {
        let source = match read_to_string(Path::new(&file)) {
            Ok(s) => s,
            Err(e) => {
                self.error(format!("Couldn't open file: {e}"));
                // Exit code for non-existent or unreadable file
                return ExitCode::from(66);
            }
        };

        match Interpreter::new(self.debug, &source).run() {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                self.error(format!("{e}"));
                ExitCode::FAILURE
            }
        }
    }

    fn error(&mut self, string: String) {
        self.stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Failed to change the color of stderr.");

        write!(&mut self.stderr, "(ERROR)").expect("Failed to write `(ERROR)` to stderr.");

        self.stderr.reset().expect("Failed to reset color of stderr.");

        writeln!(&mut self.stderr, " {string}").expect("Failed to write error message to stderr.")
    }
}

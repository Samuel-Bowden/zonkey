use interpreter::{status::InterpreterStatus, Interpreter};
use rustyline::{error::ReadlineError, Editor};
use std::{fs::read_to_string, io::Write, path::Path, process::ExitCode};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct Shell {
    debug: bool,
    stderr: StandardStream,
}

impl Shell {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            stderr: StandardStream::stderr(termcolor::ColorChoice::Always),
        }
    }

    pub fn prompt(&mut self) -> ExitCode {
        let mut prompt = match Editor::<()>::new() {
            Ok(p) => p,
            Err(e) => {
                self.error(format!("Failed to setup prompt: {e}"));
                return ExitCode::FAILURE;
            }
        };

        let mut interpreter = Interpreter::new(self.debug);

        loop {
            match prompt.readline("> ") {
                Ok(command) => {
                    prompt.add_history_entry(command.as_str());

                    match interpreter.run(&command) {
                        Ok(InterpreterStatus::Alive) => continue,
                        Ok(InterpreterStatus::Dead) => break,
                        Err(err) => self.error(format!("{}", err)),
                    }
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(e) => {
                    self.error(format!("Failed to readline from prompt: {e}"));
                    return ExitCode::FAILURE;
                }
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

        match Interpreter::new(self.debug).run(&source) {
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

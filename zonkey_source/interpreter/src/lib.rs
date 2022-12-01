use std::io::Write;
use termcolor::{StandardStream, ColorSpec, Color, WriteColor};
use self::{token::Token, lexer::Lexer, err::InterpreterErr};

mod lexer;
mod token;
mod err;
mod literal;

pub struct Interpreter<'a> {
    debug: bool,
    tokens: Vec<Token>,
    source: &'a str,
    stdout: StandardStream,
}

impl<'a> Interpreter<'a> {
    pub fn new(debug: bool, source: &'a str) -> Self {
        Self {
            debug,
            tokens: Vec::new(),
            source,
            stdout: StandardStream::stdout(termcolor::ColorChoice::Always),
        }
    }

    pub fn run(mut self) -> Result<Self, InterpreterErr> {
        self.status("Debug mode is on");

        self.status("Starting lexer");
        self.run_lexer()?;
        self.status("Lexer completed successfully.");

        Ok(self)
    }

    fn run_lexer(&mut self) -> Result<(), InterpreterErr> {
        let lexer = Lexer::new(self.source).run();

        match lexer {
            Ok(lexer) => self.tokens = lexer.tokens,
            Err(e) => return Err(InterpreterErr::LexerFailed(e)),
        }

        if self.debug {
            self.print_tokens();
        }

        Ok(())
    }

    fn print_tokens(&mut self) {
        self.debug_information("Printing tokens:");
        for (i, token) in self.tokens.iter().enumerate() {
            println!("  {}. {token}", i+1);
        }
    }

    fn status(&mut self, string: &str) {
        if self.debug {
            self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).expect("Failed to change colour of stdout.");

            write!(&mut self.stdout, "(STATUS)").expect("Failed to write `(STATUS)` to stdout.");

            self.stdout.reset().expect("Failed to reset color of stdout.");

            writeln!(&mut self.stdout, " {string}").expect("Failed to write status message to stdout.");
        }
    }

    fn debug_information(&mut self, string: &str) {
        if self.debug {
            self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,114,20)))).expect("Failed to change colour of stdout.");

            write!(&mut self.stdout, "(DEBUG INFO)").expect("Failed to write `(DEBUG INFO)` to stdout.");

            self.stdout.reset().expect("Failed to reset color of stdout.");

            writeln!(&mut self.stdout, " {string}").expect("Failed to write status message to stdout.");
        }
    }
}

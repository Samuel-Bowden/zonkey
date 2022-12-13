use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use parser::Parser;
use status::InterpreterStatus;
use std::{collections::HashMap, io::Write};
use stmt::Stmt;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use tree_walker::{status::TreeWalkerStatus, value::Value, TreeWalker};

mod err;
mod expr;
mod lexer;
mod literal;
mod parser;
pub mod status;
mod stmt;
mod token;
mod tree_walker;

pub struct Interpreter {
    debug: bool,
    tokens: Vec<Token>,
    stdout: StandardStream,
    statements: Vec<Stmt>,
    environment: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            tokens: Vec::new(),
            stdout: StandardStream::stdout(termcolor::ColorChoice::Always),
            statements: Vec::new(),
            environment: HashMap::new(),
        }
    }

    pub fn run(&mut self, source: &str) -> Result<InterpreterStatus, InterpreterErr> {
        self.status("Debug mode is on");

        self.run_lexer(source)?;

        self.run_parser()?;

        self.run_tree_walker()
    }

    fn run_lexer(&mut self, source: &str) -> Result<(), InterpreterErr> {
        self.status("Starting lexer");

        let lexer = Lexer::new(source).run();

        match lexer {
            Ok(lexer) => self.tokens = lexer.tokens,
            Err(e) => return Err(InterpreterErr::LexerFailed(e)),
        }

        self.status("Lexer completed successfully.");

        if self.debug {
            self.print_tokens();
        }

        Ok(())
    }

    fn run_parser(&mut self) -> Result<(), InterpreterErr> {
        self.status("Starting parser:");

        let mut binding = self.tokens.iter().peekable();
        let parser = Parser::new(&mut binding).run();

        match parser {
            Ok(parser) => self.statements = parser.statements,
            Err(e) => return Err(InterpreterErr::ParserFailed(e)),
        }

        self.status("Parser completed successfully.");

        if self.debug {
            self.debug_information("Printing statements:");
            println!("  {:?}", self.statements);
        }

        Ok(())
    }

    fn run_tree_walker(&mut self) -> Result<InterpreterStatus, InterpreterErr> {
        self.status("Starting tree walker:");

        match TreeWalker::new(&mut self.environment).run(self.statements.iter()) {
            Ok(status) => {
                self.status("Tree walker completed successfully.");
                match status {
                    TreeWalkerStatus::Ok => Ok(InterpreterStatus::Alive),
                    TreeWalkerStatus::Exit => Ok(InterpreterStatus::Dead),
                }
            }
            Err(err) => return Err(InterpreterErr::TreeWalkerFailed(err)),
        }
    }

    fn print_tokens(&mut self) {
        self.debug_information("Printing tokens:");
        for (i, token) in self.tokens.iter().enumerate() {
            println!("  {}. {:?}", i + 1, token);
        }
    }

    fn status(&mut self, string: &str) {
        if self.debug {
            self.stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                .expect("Failed to change colour of stdout.");

            write!(&mut self.stdout, "(STATUS)").expect("Failed to write `(STATUS)` to stdout.");

            self.stdout
                .reset()
                .expect("Failed to reset color of stdout.");

            writeln!(&mut self.stdout, " {string}")
                .expect("Failed to write status message to stdout.");
        }
    }

    fn debug_information(&mut self, string: &str) {
        if self.debug {
            self.stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 114, 20))))
                .expect("Failed to change colour of stdout.");

            write!(&mut self.stdout, "(DEBUG INFO)")
                .expect("Failed to write `(DEBUG INFO)` to stdout.");

            self.stdout
                .reset()
                .expect("Failed to reset color of stdout.");

            writeln!(&mut self.stdout, " {string}")
                .expect("Failed to write status message to stdout.");
        }
    }
}

use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use expr::Expr;
use parser::Parser;
use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use tree_walker::TreeWalker;

mod err;
mod expr;
mod lexer;
mod literal;
mod parser;
mod token;
mod tree_walker;

pub struct Interpreter<'a> {
    debug: bool,
    tokens: Vec<Token>,
    source: &'a str,
    stdout: StandardStream,
    expressions: Vec<Expr>,
}

impl<'a> Interpreter<'a> {
    pub fn new(debug: bool, source: &'a str) -> Self {
        Self {
            debug,
            tokens: Vec::new(),
            source,
            stdout: StandardStream::stdout(termcolor::ColorChoice::Always),
            expressions: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<Self, InterpreterErr> {
        self.status("Debug mode is on");

        self.run_lexer()?;

        self.run_parser()?;

        self.run_tree_walker()?;

        Ok(self)
    }

    fn run_lexer(&mut self) -> Result<(), InterpreterErr> {
        self.status("Starting lexer");

        let lexer = Lexer::new(self.source).run();

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
            Ok(parser) => self.expressions = parser.expressions,
            Err(e) => return Err(InterpreterErr::ParserFailed(e)),
        }

        self.status("Parser completed successfully.");

        if self.debug {
            self.debug_information("Printing expressions:");
            println!("  {:?}", self.expressions);
        }

        Ok(())
    }

    fn run_tree_walker(&mut self) -> Result<(), InterpreterErr> {
        self.status("Starting tree walker:");

        if let Err(e) = TreeWalker::new(&mut self.expressions.iter()).run() {
            return Err(InterpreterErr::TreeWalkerFailed(e));
        }

        self.status("Tree walker completed successfully.");

        Ok(())
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

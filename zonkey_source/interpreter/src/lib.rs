use std::io::Write;
use abstract_syntax_tree::AbstractSyntaxTree;
use parser::Parser;
use termcolor::{StandardStream, ColorSpec, Color, WriteColor};
use tree_walker::TreeWalker;
use self::{token::Token, lexer::Lexer, err::InterpreterErr};

mod lexer;
mod token;
mod err;
mod literal;
mod parser;
mod abstract_syntax_tree;
mod tree_walker;

pub struct Interpreter<'a> {
    debug: bool,
    tokens: Vec<Token>,
    source: &'a str,
    stdout: StandardStream,
    abstract_syntax_tree: Option<AbstractSyntaxTree>,
}

impl<'a> Interpreter<'a> {
    pub fn new(debug: bool, source: &'a str) -> Self {
        Self {
            debug,
            tokens: Vec::new(),
            source,
            stdout: StandardStream::stdout(termcolor::ColorChoice::Always),
            abstract_syntax_tree: None,
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

        let mut binding = self.tokens.iter();
        let parser = Parser::new(&mut binding).run();

        match parser {
            Ok(parser) => self.abstract_syntax_tree = parser.abstract_syntax_tree,
            Err(e) => return Err(InterpreterErr::ParserFailed(e)),
        }

        self.status("Parser completed successfully.");

        if self.debug {
            self.debug_information("Printing abstract syntax tree:");
            println!("  {:?}", self.abstract_syntax_tree);
        }

        Ok(())
    }

    fn run_tree_walker(&mut self) -> Result<(), InterpreterErr> {
        self.status("Starting tree walker:");

        if let Some(ast) = &self.abstract_syntax_tree {
            if let Err(e) = TreeWalker::new(ast).run() {
                return Err(InterpreterErr::TreeWalkerFailed(e));
            }
        }

        self.status("Tree walker completed successfully.");

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

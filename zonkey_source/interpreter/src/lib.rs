use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use environment::Environment;
use parser::Parser;
use status::InterpreterStatus;
use stmt::Stmt;
use tree_walker::{status::TreeWalkerStatus, TreeWalker};

mod debugger;
mod environment;
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
    statements: Vec<Stmt>,
    environment: Environment,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        interpreter_debug!(debug, "Debug mode activated");

        #[cfg(not(debug_assertions))]
        if debug {
            crate::debugger::report(
                "WARNING",
                "Debug mode not available in release build",
                termcolor::Color::Yellow,
            );
        }

        Self {
            debug,
            tokens: Vec::new(),
            statements: Vec::new(),
            environment: Environment::new(),
        }
    }

    pub fn run(&mut self, source: &str) -> Result<InterpreterStatus, InterpreterErr> {
        self.run_lexer(source)?;

        self.run_parser()?;

        self.run_tree_walker()
    }

    fn run_lexer(&mut self, source: &str) -> Result<(), InterpreterErr> {
        interpreter_debug!(self.debug, "Starting lexer");

        let lexer = Lexer::new(source, self.debug).run();

        match lexer {
            Ok(lexer) => self.tokens = lexer.tokens,
            Err(e) => return Err(InterpreterErr::LexerFailed(e)),
        }

        interpreter_debug!(self.debug, "Lexer finished successfully");

        Ok(())
    }

    fn run_parser(&mut self) -> Result<(), InterpreterErr> {
        interpreter_debug!(self.debug, "Starting parser");

        let mut binding = self.tokens.iter().peekable();
        let parser = Parser::new(&mut binding, self.debug).run();

        match parser {
            Ok(parser) => self.statements = parser.statements,
            Err(e) => return Err(InterpreterErr::ParserFailed(e)),
        }

        interpreter_debug!(self.debug, "Parser completed successfully");

        Ok(())
    }

    fn run_tree_walker(&mut self) -> Result<InterpreterStatus, InterpreterErr> {
        interpreter_debug!(self.debug, "Starting tree walker");

        match TreeWalker::new(&mut self.environment).run(self.statements.iter()) {
            Ok(status) => {
                interpreter_debug!(self.debug, "Tree walker completed successfully");
                match status {
                    TreeWalkerStatus::Ok | TreeWalkerStatus::Break => Ok(InterpreterStatus::Alive),
                    TreeWalkerStatus::Exit => Ok(InterpreterStatus::Dead),
                }
            }
            Err(err) => return Err(InterpreterErr::TreeWalkerFailed(err)),
        }
    }
}

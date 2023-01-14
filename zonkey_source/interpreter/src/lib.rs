use std::collections::VecDeque;

use crate::{environment::Environment, global::Global};

use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use parser::Parser;
use status::InterpreterStatus;
use stmt::Stmt;
use tree_walker::{status::TreeWalkerStatus, TreeWalker};

mod debugger;
mod environment;
mod err;
mod expr;
mod function;
mod global;
mod lexer;
mod literal;
mod parser;
pub mod status;
mod stmt;
mod token;
mod tree_walker;

pub struct Interpreter {
    debug: bool,
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

        Self { debug }
    }

    pub fn run(&mut self, source: &str) -> Result<InterpreterStatus, InterpreterErr> {
        let tokens = self.run_lexer(source)?;

        let statements = self.run_parser(tokens)?;

        self.run_tree_walker(statements)
    }

    fn run_lexer(&mut self, source: &str) -> Result<VecDeque<Token>, InterpreterErr> {
        interpreter_debug!(self.debug, "Starting lexer");

        let lexer = Lexer::new(source, self.debug).run();

        match lexer {
            Ok(lexer) => {
                interpreter_debug!(self.debug, "Lexer finished successfully");
                Ok(lexer.tokens)
            }
            Err(e) => Err(InterpreterErr::LexerFailed(e)),
        }
    }

    fn run_parser(&mut self, tokens: VecDeque<Token>) -> Result<VecDeque<Stmt>, InterpreterErr> {
        interpreter_debug!(self.debug, "Starting parser");

        let parser = Parser::new(tokens, self.debug).run();

        match parser {
            Ok(parser) => {
                interpreter_debug!(self.debug, "Parser completed successfully");
                Ok(parser.statements)
            }
            Err(e) => Err(InterpreterErr::ParserFailed(e)),
        }
    }

    fn run_tree_walker(
        &mut self,
        statements: VecDeque<Stmt>,
    ) -> Result<InterpreterStatus, InterpreterErr> {
        interpreter_debug!(self.debug, "Starting tree walker");

        let mut environment = Environment::new();
        let mut global = Global::new();
        let start_block = match global.scan_global(statements) {
            Ok(sb) => sb,
            Err(e) => return Err(InterpreterErr::TreeWalkerFailed(e)),
        };

        match TreeWalker::new(&mut environment, &mut global).interpret(&start_block) {
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

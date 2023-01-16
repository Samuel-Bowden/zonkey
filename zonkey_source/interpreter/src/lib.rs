use std::collections::VecDeque;

use crate::{global::Global, tree_walker::TreeWalker};

use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use crate::environment::Environment;
use parser::Parser;
use stmt::Stmt;

mod assignment_operator;
mod comparison;
mod environment;
mod err;
mod expr;
mod function;
mod global;
mod lexer;
mod native_function;
mod operator;
mod parser;
mod stmt;
mod token;
mod tree_walker;
mod value_type;
mod debugger;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        interpreter_debug!("Debug build");

        Self {}
    }

    pub fn run(&mut self, source: &str) -> Result<(), InterpreterErr> {
        let tokens = self.run_lexer(source)?;

        let statements = self.run_parser(tokens)?;

        self.run_tree_walker(statements)
    }

    fn run_lexer(&mut self, source: &str) -> Result<VecDeque<Token>, InterpreterErr> {
        interpreter_debug!("Starting lexer");

        let lexer = Lexer::new(source).run();

        match lexer {
            Ok(lexer) => {
                interpreter_debug!("Lexer finished successfully");
                Ok(lexer.tokens)
            }
            Err(e) => Err(InterpreterErr::LexerFailed(e)),
        }
    }

    fn run_parser(&mut self, tokens: VecDeque<Token>) -> Result<VecDeque<Stmt>, InterpreterErr> {
        interpreter_debug!("Starting parser");

        let parser = Parser::new(tokens).run();

        match parser {
            Ok(parser) => {
                interpreter_debug!("Parser completed successfully");
                Ok(parser.statements)
            }
            Err(e) => Err(InterpreterErr::ParserFailed(e)),
        }
    }

    fn run_tree_walker(&mut self, statements: VecDeque<Stmt>) -> Result<(), InterpreterErr> {
        interpreter_debug!("Starting tree walker");

        let mut environment = Environment::new();
        let mut global = Global::new();

        let start_block = match global.scan_global(statements) {
            Ok(sb) => sb,
            Err(e) => return Err(InterpreterErr::TreeWalkerFailed(e)),
        };

        match TreeWalker::new(&mut environment).interpret(&start_block) {
            Ok(_) => Ok(()),
            Err(e) => Err(InterpreterErr::TreeWalkerFailed(e)),
        }
    }
}

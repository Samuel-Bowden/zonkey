use std::collections::VecDeque;

use crate::{environment::Environment, global_parser::GlobalParser, tree_walker::TreeWalker};

use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use function::Function;
use stmt::Stmt;

mod assignment_operator;
mod comparison;
mod debugger;
mod environment;
mod err;
mod expr;
mod function;
mod function_declaration;
mod global_parser;
mod lexer;
mod local_parser;
mod native_function;
mod operator;
mod stmt;
mod token;
mod tree_walker;
mod value_type;

pub fn run(source: &str) -> Result<(), InterpreterErr> {
    interpreter_debug!("Debug build");

    let tokens = run_lexer(source)?;

    let (start, functions) = run_parser(tokens)?;

    run_tree_walker(start, functions)
}

fn run_lexer(source: &str) -> Result<VecDeque<Token>, InterpreterErr> {
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

fn run_parser(tokens: VecDeque<Token>) -> Result<(Stmt, Vec<Function>), InterpreterErr> {
    interpreter_debug!("Starting parser");

    match GlobalParser::new(tokens).run() {
        Ok((start, functions)) => {
            interpreter_debug!("Parser completed successfully");
            Ok((start, functions))
        }
        Err(e) => Err(InterpreterErr::ParserFailed(e)),
    }
}

fn run_tree_walker(start: Stmt, functions: Vec<Function>) -> Result<(), InterpreterErr> {
    interpreter_debug!("Starting tree walker");

    let environment = Environment::new();

    match TreeWalker::new(&functions, environment).interpret(&start) {
        Ok(_) => Ok(()),
        Err(e) => Err(InterpreterErr::TreeWalkerFailed(e)),
    }
}

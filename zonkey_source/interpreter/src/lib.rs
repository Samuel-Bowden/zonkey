use std::sync::mpsc::Sender;

use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use crate::{environment::Environment, parser::Parser, tree_walker::TreeWalker};
use callable::Callable;
use event::Event;
use stmt::Stmt;

mod assignment_operator;
mod comparison;
mod debugger;
mod environment;
pub mod err;
pub mod event;
mod expr;
mod callable;
mod declaration;
mod lexer;
mod native_function;
mod operator;
mod parser;
pub mod return_type;
mod start;
mod stmt;
pub mod token;
mod tree_walker;
mod unary_operator;
pub mod value_type;
mod value;

pub fn run(source: &Vec<&str>, sender: Sender<Event>) -> Result<(), InterpreterErr> {
    interpreter_debug!("Debug build");

    let tokens = run_lexer(source)?;

    let (start, functions) = run_parser(tokens)?;

    run_tree_walker(start, functions, sender)?;

    Ok(())
}

fn run_lexer(source: &Vec<&str>) -> Result<Vec<Token>, InterpreterErr> {
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

fn run_parser(tokens: Vec<Token>) -> Result<(Stmt, Vec<Callable>), InterpreterErr> {
    interpreter_debug!("Starting parser");

    match Parser::new(tokens).run() {
        Ok((start, callables)) => {
            interpreter_debug!("Parser completed successfully");
            Ok((start, callables))
        }
        Err(e) => Err(InterpreterErr::ParserFailed(e)),
    }
}

fn run_tree_walker(
    start: Stmt,
    callables: Vec<Callable>,
    sender: Sender<Event>,
) -> Result<(), InterpreterErr> {
    interpreter_debug!("Starting tree walker");

    let environment = Environment::new();

    match TreeWalker::new(&callables, environment, sender).interpret(&start) {
        Ok(_) => Ok(()),
        Err(e) => Err(InterpreterErr::TreeWalkerFailed(e)),
    }
}

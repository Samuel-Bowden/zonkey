use std::sync::mpsc::Sender;

use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use crate::{parser::Parser, tree_walker::TreeWalker};
use ast::AST;
use event::Event;

mod ast;
mod debugger;
pub mod err;
pub mod event;
mod expr;
mod lexer;
mod parser;
mod prelude;
mod stmt;
pub mod token;
mod tree_walker;

pub fn run(source: &Vec<&str>, sender: Sender<Event>) -> Result<(), InterpreterErr> {
    interpreter_debug!("Debug build");

    let tokens = run_lexer(source)?;

    let ast = run_parser(tokens)?;

    run_tree_walker(ast, sender)
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

fn run_parser(tokens: Vec<Token>) -> Result<AST, InterpreterErr> {
    interpreter_debug!("Starting parser");

    match Parser::new(tokens).run() {
        Ok(ast) => {
            interpreter_debug!("Parser completed successfully");
            Ok(ast)
        }
        Err(e) => Err(InterpreterErr::ParserFailed(e)),
    }
}

fn run_tree_walker(ast: AST, sender: Sender<Event>) -> Result<(), InterpreterErr> {
    interpreter_debug!("Starting tree walker");

    match TreeWalker::run(ast, sender) {
        Ok(_) => Ok(()),
        Err(e) => Err(InterpreterErr::TreeWalkerFailed(e)),
    }
}

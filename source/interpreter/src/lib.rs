use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use crate::{parser::Parser, tree_walker::TreeWalker, err::tree_walker::TreeWalkerErr};
use ast::AST;
use resource_loader::Address;
use std::sync::mpsc::{Receiver, Sender};
use ui::event::*;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

mod ast;
mod debugger;
pub mod err;
mod expr;
mod lexer;
mod parser;
mod stack;
mod standard_prelude;
mod stmt;
pub mod token;
mod tree_walker;

pub fn run_with_std_stream_error_handling(
    address: Address,
    mut sender: Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
) {
    let source = match address.load_script() {
        Ok(source) => source,
        Err(e) => {
            sender
                .send(InterpreterEvent::LoadAddressError(e.to_string()))
                .unwrap();
            return;
        }
    };

    let graphemes = UnicodeSegmentation::graphemes(source.as_str(), true).collect::<Vec<&str>>();

    match run(&graphemes, &mut sender, receiver) {
        Ok(_) => (),
        Err(e) => {
            let error_message = err::handler::run(e, &graphemes);
            eprintln!("{}", error_message);
            sender
                .send(InterpreterEvent::ScriptError(error_message))
                .unwrap();
        }
    }
}

pub fn run(
    source: &Vec<&str>,
    sender: &mut Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
) -> Result<(), InterpreterErr> {
    interpreter_debug!("Debug build");

    let tokens = run_lexer(source)?;

    let ast = run_parser(tokens)?;

    run_tree_walker(ast, sender, receiver)
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

fn run_tree_walker(
    ast: AST,
    sender: &mut Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
) -> Result<(), InterpreterErr> {
    interpreter_debug!("Starting tree walker");

    match TreeWalker::run(ast, sender, receiver) {
        Ok(_) => Ok(()),
        Err(TreeWalkerErr::Exit) => Ok(()),
        Err(e) => Err(InterpreterErr::TreeWalkerFailed(e)),
    }
}

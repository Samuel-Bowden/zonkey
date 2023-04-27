use self::{err::InterpreterErr, lexer::Lexer};
use crate::{
    err::InterpreterErrType,
    parser::Parser,
    tree_walker::{err::TreeWalkerErr, TreeWalker},
};
pub use address::Address;
use event::{InterpreterEvent, PageEvent};
pub use iced;
pub use iced_native;
use permission::PermissionLevel;
use std::sync::mpsc::{Receiver, Sender};

#[cfg(test)]
mod tests;

pub mod address;
mod ast;
mod debugger;
pub mod element;
pub mod err;
pub mod event;
mod expr;
pub mod lexer;
pub mod parser;
mod permission;
mod stack;
mod standard_prelude;
mod stmt;
mod token;
mod tree_walker;

pub fn run_with_error_messages(
    address: Address,
    mut sender: Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
) {
    let source = match address.read_string() {
        Ok(source) => source,
        Err(e) => {
            sender
                .send(InterpreterEvent::LoadAddressError(e.to_string()))
                .unwrap();
            return;
        }
    };

    match run(
        &source,
        &mut sender,
        receiver,
        PermissionLevel::new(&address),
        address.arguments,
    ) {
        Ok(_) => (),
        Err(error) => {
            let error_messages = error.get_err_messages();
            eprint!("{}", error_messages);
            sender
                .send(InterpreterEvent::ScriptError(error_messages))
                .unwrap();
        }
    }
}

pub fn run<'a>(
    source: &'a str,
    sender: &mut Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
    permission_level: PermissionLevel,
    arguments: Vec<String>,
) -> Result<(), InterpreterErr<'a>> {
    interpreter_debug!("Debug build");

    interpreter_debug!("Starting lexer");
    let (result, graphemes) = Lexer::run(source);
    let tokens = match result {
        Ok(tokens) => {
            interpreter_debug!("Lexer finished successfully");
            tokens
        }
        Err(e) => {
            return Err(InterpreterErr::new(
                InterpreterErrType::LexerFailed(e),
                graphemes,
            ))
        }
    };

    interpreter_debug!("Starting parser");
    let ast = match Parser::run(tokens) {
        Ok(ast) => {
            interpreter_debug!("Parser completed successfully");
            ast
        }
        Err(e) => {
            return Err(InterpreterErr::new(
                InterpreterErrType::ParserFailed(e),
                graphemes,
            ))
        }
    };

    interpreter_debug!("Starting tree walker");
    match TreeWalker::run(ast, sender, receiver, permission_level, arguments) {
        Ok(_) => Ok(()),
        Err(TreeWalkerErr::Exit) => Ok(()),
        Err(e) => {
            return Err(InterpreterErr::new(
                InterpreterErrType::TreeWalkerFailed(e),
                graphemes,
            ))
        }
    }
}

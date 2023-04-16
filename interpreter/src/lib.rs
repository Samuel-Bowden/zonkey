use self::{err::InterpreterErr, lexer::Lexer, token::Token};
use crate::{err::tree_walker::TreeWalkerErr, parser::Parser, tree_walker::TreeWalker};
use ast::AST;
use event::{InterpreterEvent, PageEvent};
pub use iced;
pub use iced_native;
use normalize_line_endings::normalized;
pub use resource_loader::Address;
use std::sync::mpsc::{Receiver, Sender};
pub use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

mod ast;
mod debugger;
pub mod element;
pub mod err;
pub mod event;
mod expr;
mod lexer;
mod parser;
pub mod resource_loader;
mod stack;
mod standard_prelude;
mod stmt;
mod token;
mod tree_walker;

pub enum PermissionLevel {
    All,
    NetworkOnly,
}

pub fn run_with_std_stream_error_handling(
    address: Address,
    mut sender: Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
) {
    let permission_level = match address {
        Address::Zonkey(_) | Address::File(_) | Address::Invalid(..) => PermissionLevel::All,
        Address::HTTP(..) => PermissionLevel::NetworkOnly,
    };

    let source = match address.load_script() {
        Ok(source) => source,
        Err(e) => {
            sender
                .send(InterpreterEvent::LoadAddressError(e.to_string()))
                .unwrap();
            return;
        }
    };

    let source: String = normalized(source.chars()).collect();

    let graphemes = UnicodeSegmentation::graphemes(source.as_str(), true).collect::<Vec<&str>>();

    match run(&graphemes, &mut sender, receiver, permission_level) {
        Ok(_) => (),
        Err(e) => {
            let error_message = err::handler::run(e, &graphemes);
            eprint!("{}", error_message);
            sender
                .send(InterpreterEvent::ScriptError(error_message))
                .unwrap();
        }
    }
}

// Please ensure you provide a source file with normalised line endings - e.g. \r\n gets translated to \n
pub fn run(
    source: &Vec<&str>,
    sender: &mut Sender<InterpreterEvent>,
    receiver: Receiver<PageEvent>,
    permission_level: PermissionLevel,
) -> Result<(), InterpreterErr> {
    interpreter_debug!("Debug build");

    let tokens = run_lexer(source)?;

    let ast = run_parser(tokens)?;

    run_tree_walker(ast, sender, receiver, permission_level)
}

pub fn run_lexer(source: &Vec<&str>) -> Result<Vec<Token>, InterpreterErr> {
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

pub fn run_parser(tokens: Vec<Token>) -> Result<AST, InterpreterErr> {
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
    permission_level: PermissionLevel,
) -> Result<(), InterpreterErr> {
    interpreter_debug!("Starting tree walker");

    match TreeWalker::run(ast, sender, receiver, permission_level) {
        Ok(_) => Ok(()),
        Err(TreeWalkerErr::Exit) => Ok(()),
        Err(e) => Err(InterpreterErr::TreeWalkerFailed(e)),
    }
}

use self::err_reporter::ErrReporter;
use crate::{lexer::err::LexerErr, parser::err::ParserErr, tree_walker::err::TreeWalkerErr};

mod err_reporter;
mod lexer;
mod parser;
mod tree_walker;

pub struct InterpreterErr<'a> {
    pub err_type: InterpreterErrType,
    pub graphemes: Vec<&'a str>,
}

#[derive(Debug)]
pub enum InterpreterErrType {
    LexerFailed(LexerErr),
    ParserFailed(ParserErr),
    TreeWalkerFailed(TreeWalkerErr),
}

impl<'a> InterpreterErr<'a> {
    pub fn new(err_type: InterpreterErrType, graphemes: Vec<&'a str>) -> Self {
        Self {
            err_type,
            graphemes,
        }
    }

    pub fn get_err_messages(self) -> String {
        let mut err_reporter = ErrReporter::new(&self.graphemes);

        match self.err_type {
            InterpreterErrType::LexerFailed(err) => lexer::err_handler(&mut err_reporter, err),
            InterpreterErrType::ParserFailed(err) => parser::err_handler(&mut err_reporter, err),
            InterpreterErrType::TreeWalkerFailed(err) => {
                tree_walker::err_handler(&mut err_reporter, err)
            }
        }

        err_reporter.stderr
    }
}

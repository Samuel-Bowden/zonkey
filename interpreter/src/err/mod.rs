use self::{lexer::LexerErr, parser::ParserErr, tree_walker::TreeWalkerErr};

pub mod handler;
pub mod lexer;
pub mod parser;
pub mod tree_walker;

#[derive(Debug)]
pub enum InterpreterErr {
    LexerFailed(LexerErr),
    ParserFailed(ParserErr),
    TreeWalkerFailed(TreeWalkerErr),
}

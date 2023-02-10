use self::{lexer::LexerErr, parser::ParserErr, tree_walker::TreeWalkerErr};

pub mod lexer;
pub mod parser;
pub mod tree_walker;

pub enum InterpreterErr {
    LexerFailed(LexerErr),
    ParserFailed(ParserErr),
    TreeWalkerFailed(TreeWalkerErr),
}

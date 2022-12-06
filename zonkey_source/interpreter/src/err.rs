use std::fmt::Display;
use super::tree_walker::err::TreeWalkerErr;
use super::lexer::err::LexerErr;
use super::parser::err::ParserErr;

#[derive(Debug)]
pub enum InterpreterErr {
    LexerFailed(LexerErr),
    ParserFailed(ParserErr),
    TreeWalkerFailed(TreeWalkerErr),
}

impl Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexerFailed(e) => write!(f, "Lexer failed: {e}"),
            Self::ParserFailed(e) => write!(f, "Parser failed: {e}"),
            Self::TreeWalkerFailed(e) => write!(f, "Tree walker failed: {e}"),
        }
    }
}

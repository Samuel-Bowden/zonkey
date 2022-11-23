use std::fmt::Display;

use super::lexer::err::LexerErr;

#[derive(Debug)]
pub enum InterpreterErr {
    LexerFailed(LexerErr),
}

impl Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexerFailed(e) => write!(f, "{e}"),
        }
    }
}

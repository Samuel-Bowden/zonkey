use self::{lexer::LexerErr, parser::ParserErr};

pub mod lexer;
pub mod parser;

pub enum InterpreterErr {
    LexerFailed(LexerErr),
    ParserFailed(ParserErr),
}

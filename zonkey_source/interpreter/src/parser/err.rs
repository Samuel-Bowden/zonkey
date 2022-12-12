use std::fmt::Display;

#[derive(Debug)]
pub enum ParserErr {
    ExpectedLiteral,
    UnterminatedStatement,
    ParserNotReachedEOF,
}

impl Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedLiteral => write!(f, "Expected literal"),
            Self::UnterminatedStatement => write!(f, "Unterminated statement"),
            Self::ParserNotReachedEOF => write!(f, "Parser failed to process all source code"),
        }
    }
}
